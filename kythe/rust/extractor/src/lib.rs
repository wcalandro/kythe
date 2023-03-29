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

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod vname_util;

#[derive(Deserialize)]
pub struct ExtractionInfo {
    /// The arguments from the Rust compilation action if they were available
    /// during extraction. This is always [None] for dependencies.
    pub arguments: Option<Vec<String>>,

    /// Configuration flags for rustc
    pub cfg: Vec<String>,

    /// The type of crate
    pub crate_type: String,

    /// Direct dependencies
    pub deps: Vec<ExtractionInfo>,

    /// The Rust edition
    pub edition: String,

    /// The crate's name
    pub name: String,

    /// An optional value for the OUT_DIR environment variable
    #[serde(deserialize_with = "empty_string_is_none")]
    pub out_dir_path: Option<String>,

    /// The path to the output of the action if it was available during
    /// extraction. This is always [None] for dependencies.
    pub output: Option<String>,

    /// A list of file paths that are under the root module. May contain
    /// non-Rust source files needed for compilation
    pub relevant_srcs: Vec<PathBuf>,

    /// The path to the root file for the crate
    pub root: PathBuf,

    /// The target architecture
    pub target: String,
}

fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
}

/// A dependency for a crate in a Rust project
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RustProjectCrateDep {
    /// The index of the crate in the Rust project's list of crates
    #[serde(rename = "crate")]
    pub krate: usize,

    /// The name the crate is imported as. Should be the same as the crate's
    /// original name unless it is aliased.
    pub name: String,
}

/// A source location definition for a crate in a Rust project
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RustProjectCrateSource {
    /// The list of paths to include. Each path is recursive.
    pub include_dirs: Vec<PathBuf>,

    /// The list of paths to exclude. Each path is recursive.
    pub exclude_dirs: Vec<PathBuf>,
}

/// A crate in a Rust project
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RustProjectCrate {
    /// The crate's name
    pub display_name: String,

    /// The path to the root file for the crate
    pub root_module: PathBuf,

    /// The Rust edition
    pub edition: String,

    /// A list of the crate's direct dependencies
    pub deps: Vec<RustProjectCrateDep>,

    /// Whether the crate is part of the active workspace. For Kythe's use case
    /// this should always be set to false because it enables performance
    /// improvements as it makes rust-analyzer assume the code won't change.
    pub is_workspace_member: bool,

    /// An optional definition for the locations of source files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<RustProjectCrateSource>,

    /// Configuration flags for rustc
    pub cfg: Vec<String>,

    /// The target architecture
    pub target: String,

    /// An optional map of environment variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// Whether the crate is a procedural macro
    pub is_proc_macro: bool,

    /// If the crate is a procedural macro, an optional path to the built dylib
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proc_macro_dylib_path: Option<String>,
}

impl From<&ExtractionInfo> for RustProjectCrate {
    /// Creates a [RustProjectCrate] from an [ExtractionInfo]. Does not iterate
    /// through dependencies so `deps` will always be empty, it is the
    /// responsibility of the caller to set this field.
    fn from(info: &ExtractionInfo) -> Self {
        let source: Option<RustProjectCrateSource>;
        let env: Option<HashMap<String, String>>;
        if let Some(out_dir_path) = &info.out_dir_path {
            let root_parent = info.root.parent().unwrap_or(Path::new("/")).to_path_buf();
            let include_dirs = vec![root_parent, PathBuf::from(out_dir_path)];
            source = Some(RustProjectCrateSource { include_dirs, exclude_dirs: Vec::new() });

            env = Some(HashMap::from([(String::from("OUT_DIR"), out_dir_path.to_string())]));
        } else {
            source = None;
            env = None;
        };

        Self {
            display_name: info.name.clone(),
            root_module: info.root.clone(),
            edition: info.edition.clone(),
            deps: Vec::new(),
            is_workspace_member: false,
            source,
            cfg: info.cfg.clone(),
            target: info.target.clone(),
            env,
            is_proc_macro: info.crate_type.eq("proc-macro"),
            proc_macro_dylib_path: None,
        }
    }
}

/// The structure of a rust-project.json file for rust-analyzer
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RustProject {
    /// The path to the source files for the Rust sysroot
    pub sysroot_src: Option<String>,

    /// The crates in the project
    pub crates: Vec<RustProjectCrate>,
}

impl From<&ExtractionInfo> for RustProject {
    /// Creates a [RustProject] from an instance of [ExtractionInfo]
    fn from(info: &ExtractionInfo) -> Self {
        let mut deps: Vec<RustProjectCrateDep> = Vec::new();
        let mut crates: Vec<RustProjectCrate> = Vec::new();
        for dep_info in &info.deps {
            deps.push(RustProjectCrateDep { krate: deps.len(), name: dep_info.name.clone() });
            crates.push(RustProjectCrate::from(dep_info));
        }

        let mut main_crate = RustProjectCrate::from(info);
        main_crate.deps = deps;
        main_crate.is_workspace_member = true;
        crates.push(main_crate);

        Self { sysroot_src: None, crates }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_crate_from_extraction_info() {
        let info = ExtractionInfo {
            arguments: None,
            cfg: vec!["feature=\"surprise\"".to_string()],
            crate_type: "proc-macro".to_string(),
            deps: vec![],
            edition: "2021".to_string(),
            name: "kythe_test".to_string(),
            out_dir_path: None,
            output: None,
            relevant_srcs: vec![PathBuf::from("src/main.rs"), PathBuf::from("src/util.rs")],
            root: PathBuf::from("src/main.rs"),
            target: "x86_64-unknown-linux-gnu".to_string(),
        };
        let expected = RustProjectCrate {
            display_name: "kythe_test".to_string(),
            root_module: PathBuf::from("src/main.rs"),
            edition: "2021".to_string(),
            deps: vec![],
            is_workspace_member: false,
            source: None,
            cfg: vec!["feature=\"surprise\"".to_string()],
            target: "x86_64-unknown-linux-gnu".to_string(),
            env: None,
            is_proc_macro: true,
            proc_macro_dylib_path: None,
        };

        let result = RustProjectCrate::from(&info);
        assert_eq!(expected, result);
    }

    #[test]
    fn create_crate_from_extraction_info_with_out_dir_path() {
        let info = ExtractionInfo {
            arguments: None,
            cfg: vec!["feature=\"surprise\"".to_string()],
            crate_type: "rlib".to_string(),
            deps: vec![],
            edition: "2021".to_string(),
            name: "kythe_test".to_string(),
            out_dir_path: Some("bazel-out/kythe_test_build_script.out_dir".to_string()),
            output: None,
            relevant_srcs: vec![PathBuf::from("src/main.rs"), PathBuf::from("src/util.rs")],
            root: PathBuf::from("src/main.rs"),
            target: "x86_64-unknown-linux-gnu".to_string(),
        };
        let expected = RustProjectCrate {
            display_name: "kythe_test".to_string(),
            root_module: PathBuf::from("src/main.rs"),
            edition: "2021".to_string(),
            deps: vec![],
            is_workspace_member: false,
            source: Some(RustProjectCrateSource {
                include_dirs: vec![
                    PathBuf::from("src/"),
                    PathBuf::from("bazel-out/kythe_test_build_script.out_dir/"),
                ],
                exclude_dirs: vec![],
            }),
            cfg: vec!["feature=\"surprise\"".to_string()],
            target: "x86_64-unknown-linux-gnu".to_string(),
            env: Some(HashMap::from([(
                "OUT_DIR".to_string(),
                "bazel-out/kythe_test_build_script.out_dir".to_string(),
            )])),
            is_proc_macro: false,
            proc_macro_dylib_path: None,
        };

        let result = RustProjectCrate::from(&info);
        assert_eq!(expected, result);
    }

    #[test]
    fn create_project_from_extraction_info() {
        let dep = ExtractionInfo {
            arguments: None,
            cfg: vec![],
            crate_type: "rlib".to_string(),
            deps: vec![],
            edition: "2021".to_string(),
            name: "kythe_test_dep".to_string(),
            out_dir_path: None,
            output: None,
            relevant_srcs: vec![PathBuf::from("src/lib.rs")],
            root: PathBuf::from("src/lib.rs"),
            target: "x86_64-unknown-linux-gnu".to_string(),
        };
        let info = ExtractionInfo {
            arguments: None,
            cfg: vec![],
            crate_type: "rlib".to_string(),
            deps: vec![dep],
            edition: "2021".to_string(),
            name: "kythe_test".to_string(),
            out_dir_path: None,
            output: None,
            relevant_srcs: vec![PathBuf::from("src/bin/main.rs")],
            root: PathBuf::from("src/bin/main.rs"),
            target: "x86_64-unknown-linux-gnu".to_string(),
        };

        let expected_crate_1 = RustProjectCrate {
            display_name: "kythe_test_dep".to_string(),
            root_module: PathBuf::from("src/lib.rs"),
            edition: "2021".to_string(),
            deps: vec![],
            is_workspace_member: false,
            source: None,
            cfg: vec![],
            target: "x86_64-unknown-linux-gnu".to_string(),
            env: None,
            is_proc_macro: false,
            proc_macro_dylib_path: None,
        };
        let expected_crate_2 = RustProjectCrate {
            display_name: "kythe_test".to_string(),
            root_module: PathBuf::from("src/bin/main.rs"),
            edition: "2021".to_string(),
            deps: vec![RustProjectCrateDep { krate: 0, name: "kythe_test_dep".to_string() }],
            is_workspace_member: true,
            source: None,
            cfg: vec![],
            target: "x86_64-unknown-linux-gnu".to_string(),
            env: None,
            is_proc_macro: false,
            proc_macro_dylib_path: None,
        };
        let expected =
            RustProject { sysroot_src: None, crates: vec![expected_crate_1, expected_crate_2] };

        let result = RustProject::from(&info);
        assert_eq!(expected, result);
    }
}
