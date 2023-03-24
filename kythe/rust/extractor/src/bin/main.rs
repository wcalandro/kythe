// Copyright 2023 The Kythe Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use analysis_rust_proto::*;
use anyhow::{Context, Result};
use clap::Parser;
use extractor_lib::RustProject;
use extractor_lib::{vname_util::VNameRule, ExtractionInfo};
use glob::glob;
use protobuf::Message;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::{fs::File, io::Read, path::PathBuf};
use zip::{write::FileOptions, ZipWriter};

#[derive(Parser)]
#[clap(author = "The Kythe Authors")]
#[clap(about = "Rust Kythe Extractor", long_about = None)]
#[clap(rename_all = "snake_case")]
struct Args {
    /// Path to .rust_extraction_info.json file
    #[clap(long, value_parser)]
    extraction_info: PathBuf,

    /// Desired output path for the kzip
    #[clap(long, value_parser)]
    output: PathBuf,

    /// Location of the vnames configuration file
    #[clap(long, value_parser)]
    vnames_config: PathBuf,
}

fn main() -> Result<()> {
    let config = Args::parse();

    // Parse the extraction info
    let extraction_info_file =
        File::open(&config.extraction_info).context("Failed to open extraction info file")?;
    let extraction_info: ExtractionInfo = serde_json::from_reader(extraction_info_file)
        .context("Failed to parse extraction info file")?;

    // See if the KYTHE_CORPUS variable is set
    let default_corpus = std::env::var("KYTHE_CORPUS").unwrap_or_default();

    // Parse the VName configuration rules
    let mut vname_rules = VNameRule::parse_vname_rules(&config.vnames_config)?;

    // Create the output kzip
    let kzip_file = File::create(&config.output)
        .with_context(|| format!("Failed to create kzip file at path {:?}", &config.output))?;
    let mut kzip = ZipWriter::new(kzip_file);
    kzip.add_directory("root/", FileOptions::default())?;

    // Get paths for files present in the build script out_dir if there is one
    let mut out_dir_inputs: Vec<PathBuf> = Vec::new();
    if let Some(out_dir) = &extraction_info.out_dir_path {
        let glob_pattern = format!("{out_dir}/**/*");
        for path in glob(&glob_pattern).unwrap().flatten() {
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    // Has to be nested if because of the `if let`
                    if extension.eq("rmeta") {
                        continue;
                    }
                }
                out_dir_inputs.push(path);
            }
        }
    }

    // Get list of paths that are required inputs
    let mut required_input_paths: Vec<PathBuf> = Vec::new();
    required_input_paths.extend_from_slice(&extraction_info.relevant_srcs);
    required_input_paths.extend_from_slice(&out_dir_inputs);
    for dep in &extraction_info.deps {
        required_input_paths.extend_from_slice(&dep.relevant_srcs)
    }

    // Collect all of the required inputs and add them to the kzip
    let mut required_inputs: Vec<CompilationUnit_FileInput> = Vec::new();
    for path in required_input_paths {
        let path_string = path.display().to_string();
        let vname: VName = create_vname(&mut vname_rules, &path_string, &default_corpus);
        kzip_add_required_input(&path, vname, &mut kzip, &mut required_inputs)?;
    }

    // Create the Rust project JSON file and add it to the kzip
    let rust_project = RustProject::from(&extraction_info);
    let rust_project_bytes = serde_json::to_vec(&rust_project)
        .context("Failed to convert Rust project to JSON bytes")?;
    let rust_project_digest = sha256digest(&rust_project_bytes);
    kzip_add_file(format!("root/files/{rust_project_digest}"), &rust_project_bytes, &mut kzip)?;

    // Add the Rust project to the required inputs
    let mut rust_project_input = CompilationUnit_FileInput::new();
    let rust_project_vname =
        create_vname(&mut vname_rules, "kythe-rust-project.json", &default_corpus);
    rust_project_input.set_v_name(rust_project_vname);
    let mut rust_project_file_info = FileInfo::new();
    rust_project_file_info.set_path("kythe-rust-project.json".to_string());
    rust_project_file_info.set_digest(rust_project_digest);
    rust_project_input.set_info(rust_project_file_info);
    required_inputs.push(rust_project_input);

    // Collect all of the source files for only the crate being built. This includes
    // files from the build script output directory.
    let mut source_files: Vec<String> = Vec::new();
    source_files.extend(extraction_info.relevant_srcs.iter().map(|p| p.display().to_string()));
    source_files.extend(out_dir_inputs.iter().map(|p| p.display().to_string()));

    // Create the VName for the compilation unit
    let root_path_string = extraction_info.root.display().to_string();
    let mut unit_vname = create_vname(&mut vname_rules, &root_path_string, &default_corpus);
    unit_vname.set_language("rust".to_string());
    if !default_corpus.is_empty() {
        unit_vname.set_corpus(default_corpus);
    }

    // Create the compilation unit and the encompassing indexed compilation
    let mut compilation_unit = CompilationUnit::new();
    compilation_unit.set_v_name(unit_vname);
    compilation_unit.set_source_file(protobuf::RepeatedField::from_vec(source_files));
    compilation_unit.set_required_input(protobuf::RepeatedField::from_vec(required_inputs));
    if let Some(arguments) = &extraction_info.arguments {
        compilation_unit.set_argument(protobuf::RepeatedField::from_vec(arguments.clone()));
    }
    if let Some(output) = &extraction_info.output {
        compilation_unit.set_output_key(output.to_string());
    }

    let mut indexed_compilation = IndexedCompilation::new();
    indexed_compilation.set_unit(compilation_unit);

    // Add the indexed compilation to the kzip
    let ic_bytes = indexed_compilation
        .write_to_bytes()
        .context("Failed to convert IndexedCompilation to bytes")?;
    let ic_digest = sha256digest(&ic_bytes);
    kzip_add_file(format!("root/pbunits/{ic_digest}"), &ic_bytes, &mut kzip)?;

    Ok(())
}

/// Generate sha256 hex digest of a vector of bytes
fn sha256digest(bytes: &[u8]) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(bytes);
    let bytes = sha256.finalize();
    hex::encode(bytes)
}

/// Add a file with a given path and VName to a kzip and a list of required
/// inputs
///
/// * `path` - The &PathBuf to the file
/// * `vname` - The VName for the file
/// * `zip_writer` - A mutable writer for the kzip
/// * `required_inputs` - A mutable vector that the new
///   CompilationUnit_FileInput will be added to
fn kzip_add_required_input(
    path: &PathBuf,
    vname: VName,
    zip_writer: &mut ZipWriter<File>,
    required_inputs: &mut Vec<CompilationUnit_FileInput>,
) -> Result<()> {
    let mut file =
        File::open(path).with_context(|| format!("Failed to open {}", path.display()))?;
    let mut file_contents: Vec<u8> = Vec::new();

    file.read_to_end(&mut file_contents)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let digest = sha256digest(&file_contents);
    kzip_add_file(format!("root/files/{digest}"), &file_contents, zip_writer)?;

    // Generate FileInput and add it to the list of required inputs
    let mut file_input = CompilationUnit_FileInput::new();
    file_input.set_v_name(vname);

    let mut file_info = FileInfo::new();
    file_info.set_path(path.display().to_string());
    file_info.set_digest(digest);
    file_input.set_info(file_info);

    required_inputs.push(file_input);
    Ok(())
}

/// Add a file to the kzip with the specified name and contents
///
/// * `file_name` - The new file's path inside the zip archive
/// * `file_bytes` - The byte contents of the new file
/// * `zip_writer` - The ZipWriter to be written to
fn kzip_add_file(
    file_name: String,
    file_bytes: &[u8],
    zip_writer: &mut ZipWriter<File>,
) -> Result<()> {
    zip_writer
        .start_file(&file_name, FileOptions::default())
        .with_context(|| format!("Failed to create file in kzip: {file_name}"))?;
    zip_writer
        .write_all(file_bytes)
        .with_context(|| format!("Failed to write file contents to kzip: {file_name}"))?;
    Ok(())
}

/// Create a VName from a slice of [VNameRule]s, a path, and a default corpus
fn create_vname(rules: &mut [VNameRule], path: &str, default_corpus: &str) -> VName {
    for rule in rules {
        if rule.matches(path) {
            return rule.produce_vname(path, default_corpus);
        }
    }
    // This should never happen but if we don't match at all just return an empty
    // vname with the corpus set and log it
    eprintln!("Warning: {path} did not match any VName rules");
    let mut vname = VName::new();
    vname.set_corpus(default_corpus.to_string());
    vname
}
