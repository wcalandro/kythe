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

pub mod analyzer;
mod docs;
pub mod entries;
mod kytheuri;

use crate::error::KytheError;
use crate::providers::FileProvider;
use crate::writer::KytheWriter;

use analysis_rust_proto::*;
use analyzer::UnitAnalyzer;
use glob::glob;
use path_absolutize::*;
use ra_ap_paths::AbsPathBuf;
use rustc_hash::FxHashMap;

use std::path::PathBuf;

/// A data structure for indexing CompilationUnits
pub struct KytheIndexer<'a> {
    writer: &'a mut dyn KytheWriter,
    sysroot: Option<String>,
    sysroot_src: Option<String>,
    sysroot_src_files: Option<FxHashMap<AbsPathBuf, String>>,
    max_parallelism: Option<u8>,
}

impl<'a> KytheIndexer<'a> {
    /// Create a new instance of the KytheIndexer
    pub fn new(
        writer: &'a mut dyn KytheWriter,
        sysroot: Option<PathBuf>,
        sysroot_src: Option<PathBuf>,
        max_parallelism: Option<u8>,
    ) -> Self {
        // Absolutize sysroot paths and load all sysroot_src_files
        let sysroot_absolute =
            sysroot.map(|s| s.absolutize().unwrap().to_str().unwrap().to_string());
        let sysroot_src_absolute =
            sysroot_src.map(|s| s.absolutize().unwrap().to_str().unwrap().to_string());
        let sysroot_src_files = if let Some(path) = &sysroot_src_absolute {
            let mut map = FxHashMap::default();
            for entry in glob(&format!("{path}/**/*.rs")).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        if let Ok(contents) = std::fs::read_to_string(&path) {
                            map.insert(AbsPathBuf::assert(path), contents);
                        }
                    }
                    Err(_) => continue,
                }
            }
            Some(map)
        } else {
            None
        };
        Self {
            writer,
            sysroot: sysroot_absolute,
            sysroot_src: sysroot_src_absolute,
            sysroot_src_files,
            max_parallelism,
        }
    }

    /// Accepts a CompilationUnit and the directory for analysis files and
    /// indexes the CompilationUnit
    pub fn index_cu(
        &mut self,
        unit: &CompilationUnit,
        provider: &mut dyn FileProvider,
    ) -> Result<(), KytheError> {
        let mut generator = UnitAnalyzer::new(
            unit,
            self.writer,
            provider,
            self.sysroot.clone(),
            self.sysroot_src.clone(),
            self.sysroot_src_files.clone(),
            self.max_parallelism,
        )?;

        generator.handle_files()?;
        generator.index_crate()?;

        // We must flush the writer each time to ensure that all entries get written
        self.writer.flush()?;
        Ok(())
    }
}
