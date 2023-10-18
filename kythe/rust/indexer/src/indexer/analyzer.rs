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

use crate::error::KytheError;
use crate::providers::FileProvider;
use crate::writer::KytheWriter;

use super::docs::process_documentation;
use super::entries::EntryEmitter;
use super::kytheuri::vname_to_kythe_uri;

use analysis_rust_proto::CompilationUnit;
use common_rust_proto::{Link, MarkedSource, MarkedSource_Kind};
use protobuf::{Message, RepeatedField};
use ra_ap_hir::{
    Access, Adt, AsAssocItem, AssocItemContainer, Crate, DefWithBody, FieldSource, GenericDef,
    GenericParam, HasAttrs, HasSource, HirDisplay, InFile, MacroKind, Module, ModuleSource,
    Semantics, StructKind, VariantDef,
};
use ra_ap_hir_def::visibility::Visibility;
use ra_ap_ide::{AnalysisHost, Change, FileId, RootDatabase};
use ra_ap_ide_db::defs::{Definition, IdentClass};
use ra_ap_ide_db::documentation::docs_with_rangemap;
use ra_ap_ide_db::helpers::get_definition;
use ra_ap_load_cargo::ProjectFolders;
use ra_ap_paths::AbsPath;
use ra_ap_project_model::{ProjectJson, ProjectJsonData, ProjectWorkspace};
use ra_ap_syntax::{
    ast::{AstNode, HasName},
    NodeOrToken, SyntaxKind, SyntaxToken, TextRange, TextSize, T,
};
use ra_ap_vfs::{Vfs, VfsPath};
use rustc_hash::FxHashMap;
use storage_rust_proto::*;
use triomphe::Arc;

use std::collections::HashMap;
use std::path::Path;

struct FileRange {
    pub file_id: u32,
    pub text_range: TextRange,
}

/// A data structure to analyze and index CompilationUnit protobufs
pub struct UnitAnalyzer<'a> {
    // The CompilationUnit being analyzed
    unit: &'a CompilationUnit,
    // The storage_rust_proto VName for the CompilationUnit
    unit_storage_vname: VName,
    // The emitter used to  write generated nodes and edges
    emitter: EntryEmitter<'a>,
    // A map between a file name and its Kythe VName
    file_vnames: HashMap<String, VName>,
    // A map between a file name and its sha256 digest
    file_digests: HashMap<String, String>,
    // A file provider
    provider: &'a mut dyn FileProvider,
    /// A map between rust-analyzer FileId and the string source path
    file_id_to_path: HashMap<u32, String>,
    /// A map between rust-analyzer FileId and Kythe VName
    file_id_to_vname: HashMap<u32, VName>,
    /// A map between rust-analyzer Definition and Kythe VName signature
    def_to_signature: HashMap<Definition, String>,
}

impl<'a> UnitAnalyzer<'a> {
    /// Create an instance to assist in analyzing `unit`. Graph information will
    /// be written to the `writer` and source file contents will be read using
    /// `root_dir` as a base directory.
    pub fn new(
        unit: &'a CompilationUnit,
        writer: &'a mut dyn KytheWriter,
        provider: &'a mut dyn FileProvider,
    ) -> Result<Self, KytheError> {
        // Create a HashMap between the file path and the VName which we can retrieve
        // later to emit nodes and create a HashMap between a file path and its digest
        let mut file_vnames = HashMap::new();
        let mut file_digests = HashMap::new();
        let required_inputs = unit.get_required_input();

        // Check if there are no required inputs
        if required_inputs.is_empty() {
            return Err(KytheError::IndexerError(
                "There are no required inputs present in the CompilationUnit".to_owned(),
            ));
        }

        for required_input in unit.get_required_input() {
            let analysis_vname = required_input.get_v_name();
            let path = required_input.get_info().get_path().to_owned();
            let mut storage_vname: VName = analysis_to_storage_vname(analysis_vname);
            // Remove the language and signature fields from the VName
            storage_vname.clear_language();
            storage_vname.clear_signature();
            file_vnames.insert(path.clone(), storage_vname);
            file_digests.insert(path.clone(), required_input.get_info().get_digest().to_string());
        }

        let unit_storage_vname: VName = analysis_to_storage_vname(unit.get_v_name());
        Ok(Self {
            unit,
            unit_storage_vname,
            emitter: EntryEmitter::new(writer),
            file_vnames,
            file_digests,
            provider,
            file_id_to_path: HashMap::new(),
            file_id_to_vname: HashMap::new(),
            def_to_signature: HashMap::new(),
        })
    }

    /// Emits file nodes for all of the source files in a CompilationUnit
    pub fn handle_files(&mut self) -> Result<(), KytheError> {
        // https://kythe.io/docs/schema/#file
        for source_file in self.unit.get_source_file() {
            let vname_result = self.file_vnames.get(source_file);
            // Generated files won't have a file vname returned
            if vname_result.is_none() {
                continue;
            }
            let vname = vname_result.unwrap();

            // Create the file node fact
            self.emitter.emit_fact(vname, "/kythe/node/kind", b"file".to_vec())?;

            // Create language fact
            self.emitter.emit_fact(vname, "/kythe/language", b"rust".to_vec())?;

            // Read the file contents and set it on the fact
            // Returns a FileReadError if we can't read the file
            let file_contents: String =
                if let Some(file_digest) = self.file_digests.get(source_file) {
                    let file_bytes = self.provider.contents(source_file, file_digest)?;
                    String::from_utf8(file_bytes).map_err(|_| {
                        KytheError::IndexerError(format!(
                            "Failed to read file {source_file} as UTF8 string"
                        ))
                    })?
                } else {
                    return Err(KytheError::FileNotFoundError(source_file.to_string()));
                };

            // Create text fact
            self.emitter.emit_fact(vname, "/kythe/text", file_contents.into_bytes())?;
        }
        Ok(())
    }

    pub fn index_crate(&mut self) -> Result<(), KytheError> {
        // Get the Rust project from the and deserialize it
        let rust_project_file = self.get_rust_project_file()?;
        let rust_project_data: ProjectJsonData =
            serde_json::from_str(&rust_project_file).map_err(|e| {
                KytheError::IndexerError(format!("Failed to parse kythe-rust-project.json: {e}"))
            })?;
        let project_root = AbsPath::assert(Path::new("/kythe"));
        let rust_project = ProjectJson::new(project_root, rust_project_data);

        // Create the project workspace from the project
        let extra_env: FxHashMap<String, String> = FxHashMap::default();
        let workspace = ProjectWorkspace::load_inline(rust_project, None, &extra_env, None);

        // Add all required inputs to the VFS and the analysis change and keep track of
        // the file ids that correspond to the root crate's source files
        let mut vfs = Vfs::default();
        let mut analysis_change = Change::new();
        let source_files = self.unit.get_source_file();
        let mut source_file_ids: Vec<u32> = Vec::new();
        for required_input in self.unit.get_required_input() {
            let path = required_input.get_info().get_path().to_owned();
            if path.eq("kythe-rust-project.json") {
                continue;
            }
            let digest = required_input.get_info().get_digest().to_string();
            let file_bytes = self.provider.contents(&path, &digest)?;

            // Add to VFS
            let vfs_path = VfsPath::from(project_root.join(Path::new(&path)));
            vfs.set_file_contents(vfs_path.clone(), Some(file_bytes.clone()));

            // Attempt to add to the analysis change
            let file_id = vfs.file_id(&vfs_path).unwrap();
            let text = String::from_utf8(file_bytes).map_err(|e| {
                KytheError::IndexerError(format!(
                    "Failed to serialize the contents of {path} as a UTF-8 string: {e}"
                ))
            })?;
            analysis_change.change_file(file_id, Some(Arc::from(text)));

            // Add file information to relevant hashmaps
            self.file_id_to_path.insert(file_id.0, path.clone());
            let vname = required_input.get_v_name();
            self.file_id_to_vname.insert(file_id.0, analysis_to_storage_vname(vname));

            // Add the file id to the list of source file ids if this is a source file for
            // the root crate
            if source_files.contains(&path) {
                source_file_ids.push(file_id.0)
            }
        }

        // Generate and set the crate graph
        let (crate_graph, _) = workspace.to_crate_graph(
            &mut |path: &AbsPath| {
                // We already loaded the files so we just have to give it the file id
                vfs.file_id(&VfsPath::from(path.to_path_buf()))
            },
            &extra_env,
        );
        analysis_change.set_crate_graph(crate_graph);

        // Generate and set the roots
        let project_folders = ProjectFolders::new(&[workspace], &[]);
        let source_roots = project_folders.source_root_config.partition(&vfs);
        analysis_change.set_roots(source_roots);

        // Create the analysis host and apply the change
        let mut analysis_host = AnalysisHost::new(None);
        analysis_host.apply_change(analysis_change);

        // Get the rust-analyzer database
        let db = analysis_host.raw_database();

        // Get the root module of the crate being analyzed
        let root_module = get_root_module_in_file_ids(db, &source_file_ids).ok_or_else(|| {
            KytheError::IndexerError(
                "Failed to find root module for crate being indexed".to_string(),
            )
        })?;

        // Emit nodes for all of the modules in the crate
        self.emit_modules(db, root_module)?;

        // Analyze all source files
        let semantics = Semantics::new(db);
        for file_id in source_file_ids {
            let tokens = semantics
                .parse(FileId(file_id))
                .syntax()
                .descendants_with_tokens()
                .filter_map(|x| match x {
                    NodeOrToken::Node(_) => None,
                    NodeOrToken::Token(x) => Some(x),
                })
                .filter(|token| {
                    matches!(
                        token.kind(),
                        T![ident]
                            | T![lifetime_ident]
                            | T![crate]
                            | T![super]
                            | T![self]
                            | T![Self]
                    )
                });
            for token in tokens {
                match self.visit_token(&semantics, db, file_id, token) {
                    // TODO: Emit diagnostic node
                    Err(KytheError::DiagnosticError(_)) => {}
                    Err(e) => return Err(e),
                    _ => {}
                };
            }
        }

        Ok(())
    }

    /// Given the database and the crate's root module, emit nodes and anchors
    /// for all modules in the crate
    fn emit_modules(&mut self, db: &RootDatabase, root_module: Module) -> Result<(), KytheError> {
        // Recurse through child modules to get all of the modules in the crate
        let mut worklist: Vec<Module> = vec![root_module];
        let mut modules = Vec::new();

        while let Some(module) = worklist.pop() {
            modules.push(module);
            worklist.extend(module.children(db));
        }

        let mut module_to_vname: HashMap<Module, VName> = HashMap::new();

        for module in modules {
            let def = Definition::Module(module);
            let def_source = module.definition_source(db);
            let file_id = def_source.file_id.original_file(db);

            // Create the signature for the module
            let mut def_vname = self.gen_base_vname();
            let mut parent_vname: Option<VName> = None;
            if module.crate_root(db) == module {
                // The signature of the root module is just the root path of the crate
                def_vname.set_signature(self.file_id_to_path.get(&file_id.0).unwrap().to_owned());
            } else {
                let parent_module = module.parent(db).unwrap();
                let parent = module_to_vname.get(&parent_module).unwrap();
                parent_vname = Some(parent.to_owned());
                let parent_signature = parent.get_signature();
                let name = module.name(db).unwrap().to_smol_str();
                def_vname.set_signature(format!("{parent_signature}::{name}"));
            }
            module_to_vname.insert(module, def_vname.clone());
            self.def_to_signature.insert(def, def_vname.get_signature().to_string());

            // Emit the facts about the module
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"record".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/subkind", b"module".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/complete", b"definition".to_vec())?;

            // Emit the childof edge to the parent module if there is one
            if let Some(vname) = parent_vname {
                self.emitter.emit_edge(&def_vname, &vname, "/kythe/edge/childof")?;
            }

            // Emit the anchor for the module
            let mut anchor_vname = self.file_id_to_vname.get(&file_id.0).unwrap().clone();
            anchor_vname.set_signature(format!("{}_anchor", def_vname.get_signature()));
            match def_source.value {
                ModuleSource::SourceFile(_) => {
                    // Emit implicit anchor
                    self.emitter.emit_fact(
                        &anchor_vname,
                        "/kythe/node/kind",
                        b"anchor".to_vec(),
                    )?;
                    self.emitter.emit_fact(&anchor_vname, "/kythe/loc/start", b"0".to_vec())?;
                    self.emitter.emit_fact(&anchor_vname, "/kythe/loc/end", b"0".to_vec())?;
                    self.emitter.emit_edge(
                        &anchor_vname,
                        &def_vname,
                        "/kythe/edge/defines/implicit",
                    )?;
                }
                ModuleSource::Module(m) => {
                    if let Some(name) = m.name() {
                        let range = InFile::new(def_source.file_id, name.syntax())
                            .original_file_range_opt(db)
                            .map(|it| it.range);
                        if let Some(range) = range {
                            let start = u32::from(range.start());
                            let end = u32::from(range.end());
                            self.emitter.emit_anchor(&anchor_vname, &def_vname, start, end)?;
                        } else {
                            // TODO: We'll have to emit some diagnostic about
                            // not being able to
                            // find the identifier
                        }
                    }
                }
                // Not sure when a module would be defined in a block expression but we'll ignore it
                // for the time being
                _ => {}
            };

            // Emit module documentation if it is present
            if let Some((doc, range_map)) = docs_with_rangemap(db, &module.attrs(db)) {
                let mut doc_vname = def_vname.clone();
                doc_vname.set_signature(format!("{}::(DOC)", def_vname.get_signature()));
                self.emitter.emit_fact(&doc_vname, "/kythe/node/kind", b"doc".to_vec())?;
                self.emitter.emit_edge(&doc_vname, &def_vname, "/kythe/edge/documents")?;

                // Process the documentation, emit the text, and emit any params present in the
                // text
                let (doc_text, doc_refs) = process_documentation(&def, &doc, &range_map, db);
                self.emitter.emit_fact(&doc_vname, "/kythe/text", doc_text.into())?;
                for (i, doc_ref) in doc_refs.iter().enumerate() {
                    // Emit the doc param referencing the item in the link
                    if let Some(signature) = self.get_signature(db, doc_ref.reference) {
                        let mut ref_vname = self.gen_base_vname();
                        ref_vname.set_signature(signature);
                        let edge_kind = format!("/kythe/edge/param.{i}");
                        self.emitter.emit_edge(&doc_vname, &ref_vname, &edge_kind)?;

                        // If we got a range back for the link in the documentation, emit an anchor
                        // in the docs with a reference.
                        if let Some(range) = doc_ref.range {
                            let range_start = u32::from(range.start());
                            let range_end = u32::from(range.end());
                            let mut ref_anchor = anchor_vname.clone();
                            ref_anchor
                                .set_signature(format!("anchor_{range_start}_to_{range_end}"));

                            self.emitter.emit_fact(
                                &ref_anchor,
                                "/kythe/node/kind",
                                b"anchor".to_vec(),
                            )?;
                            self.emitter.emit_fact(
                                &ref_anchor,
                                "/kythe/loc/start",
                                range_start.to_string().into_bytes().to_vec(),
                            )?;
                            self.emitter.emit_fact(
                                &ref_anchor,
                                "/kythe/loc/end",
                                range_end.to_string().into_bytes().to_vec(),
                            )?;
                            self.emitter.emit_edge(
                                &ref_anchor,
                                &ref_vname,
                                "/kythe/edge/ref/doc",
                            )?;
                        }
                    }
                }
            }

            // Generate and emit MarkedSource
            if let Some(marked_source) = self.gen_marked_source(def, db) {
                self.emitter.emit_fact(&def_vname, "/kythe/code", marked_source)?;
            }
        }

        Ok(())
    }

    /// Get a VName signature for a Definition from the cache or by generating
    /// it
    fn get_signature(&mut self, db: &RootDatabase, def: Definition) -> Option<String> {
        // Check if we already know what the signature is
        if let Some(signature) = self.def_to_signature.get(&def) {
            return Some(signature.to_owned());
        }

        let signature = match def {
            Definition::Adt(adt) => {
                let name = adt.name(db).to_smol_str();
                let module_signature =
                    self.get_signature(db, Definition::Module(adt.module(db)))?;
                match adt {
                    Adt::Enum(_) => Some(format!("{module_signature}::ENUM({name})")),
                    Adt::Struct(_) => Some(format!("{module_signature}::STRUCT({name})")),
                    Adt::Union(_) => Some(format!("{module_signature}::UNION({name})")),
                }
            }
            Definition::Const(const_) => {
                let name = if let Some(name) = const_.name(db) {
                    name.to_smol_str().to_string()
                } else {
                    let range = const_.source(db).unwrap().value.syntax().text_range();
                    let start = u32::from(range.start());
                    let end = u32::from(range.end());
                    format!("{start}-{end}")
                };
                let parent_signature = if let Some(assoc_item) = const_.as_assoc_item(db) {
                    self.get_assoc_item_parent_signature(db, assoc_item.container(db))
                } else {
                    self.get_signature(db, Definition::Module(const_.module(db)))
                }?;
                Some(format!("{parent_signature}::CONST({name})"))
            }
            Definition::Field(field) => {
                let name = field.name(db).to_smol_str();
                let parent_def = field.parent_def(db);
                let parent_signature = match parent_def {
                    VariantDef::Struct(s) => {
                        self.get_signature(db, Definition::Adt(Adt::Struct(s)))
                    }
                    VariantDef::Union(u) => self.get_signature(db, Definition::Adt(Adt::Union(u))),
                    VariantDef::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                }?;
                Some(format!("{parent_signature}::FIELD({name}"))
            }
            Definition::Function(function) => {
                let name = function.name(db).to_smol_str();
                let parent_signature = if let Some(assoc_item) = function.as_assoc_item(db) {
                    self.get_assoc_item_parent_signature(db, assoc_item.container(db))
                } else {
                    self.get_signature(db, Definition::Module(function.module(db)))
                }?;
                Some(format!("{parent_signature}::FUNCTION({name})"))
            }
            Definition::GenericParam(param) => {
                let name = param.name(db).to_smol_str();
                let parent_signature = match param.parent() {
                    GenericDef::Function(f) => self.get_signature(db, Definition::Function(f)),
                    GenericDef::Adt(adt) => self.get_signature(db, Definition::Adt(adt)),
                    GenericDef::Trait(t) => self.get_signature(db, Definition::Trait(t)),
                    GenericDef::TraitAlias(ta) => {
                        self.get_signature(db, Definition::TraitAlias(ta))
                    }
                    GenericDef::TypeAlias(ta) => self.get_signature(db, Definition::TypeAlias(ta)),
                    GenericDef::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                    GenericDef::Const(c) => self.get_signature(db, Definition::Const(c)),
                    _ => None,
                }?;
                Some(format!("{parent_signature}::TVAR({name})"))
            }
            Definition::Label(label) => {
                let name = label.name(db).to_smol_str();
                let range = label.source(db).value.syntax().text_range();
                let start = u32::from(range.start());
                let end = u32::from(range.end());
                let parent_signature = match label.parent(db) {
                    DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                    DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                    DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                    DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                    DefWithBody::InTypeConst(_) => None, // Not sure what this is
                }?;
                Some(format!("{parent_signature}::LABEL({name}|{start}-{end})"))
            }
            Definition::Local(local) => {
                let name = local.name(db).to_smol_str();
                let source = local.primary_source(db);
                let range = source.syntax().text_range();
                let start = u32::from(range.start());
                let end = u32::from(range.end());
                let parent_signature = match local.parent(db) {
                    DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                    DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                    DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                    DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                    DefWithBody::InTypeConst(_) => None, // Not sure what this is
                }?;
                Some(format!("{parent_signature}::LOCAL({name}|{start}-{end})"))
            }
            Definition::Macro(makro) => {
                let name = makro.name(db).to_smol_str();
                let module_signature =
                    self.get_signature(db, Definition::Module(makro.module(db)))?;
                Some(format!("{module_signature}::MACRO({name})"))
            }
            Definition::Module(module) => {
                if module.is_crate_root() {
                    let def_source = module.definition_source(db);
                    let file_id = def_source.file_id.original_file(db);
                    Some(self.file_id_to_path.get(&file_id.0).unwrap().to_owned())
                } else {
                    let parent = module.parent(db).unwrap();
                    let parent_signature = self.get_signature(db, Definition::Module(parent))?;
                    let name = module.name(db).unwrap().to_smol_str();
                    Some(format!("{parent_signature}::{name}"))
                }
            }
            Definition::Static(static_) => {
                let name = static_.name(db).to_smol_str();
                let module_signature =
                    self.get_signature(db, Definition::Module(static_.module(db)))?;
                let range = static_.source(db).unwrap().value.syntax().text_range();
                let start = u32::from(range.start());
                let end = u32::from(range.end());
                Some(format!("{module_signature}::STATIC({name}|{start}-{end})"))
            }
            Definition::Trait(trate) => {
                let name = trate.name(db).to_smol_str();
                let module_signature =
                    self.get_signature(db, Definition::Module(trate.module(db)))?;
                Some(format!("{module_signature}::TRAIT({name})"))
            }
            Definition::TraitAlias(trait_alias) => {
                let name = trait_alias.name(db).to_smol_str();
                let module_signature =
                    self.get_signature(db, Definition::Module(trait_alias.module(db)))?;
                Some(format!("{module_signature}::TRAIT_ALIAS({name})"))
            }
            Definition::TypeAlias(talias) => {
                let name = talias.name(db).to_smol_str();
                let range = talias.source(db).unwrap().value.syntax().text_range();
                let start = u32::from(range.start());
                let end = u32::from(range.end());
                let parent_signature = if let Some(assoc_item) = talias.as_assoc_item(db) {
                    self.get_assoc_item_parent_signature(db, assoc_item.container(db))
                } else {
                    self.get_signature(db, Definition::Module(talias.module(db)))
                }?;
                Some(format!("{parent_signature}::TALIAS({name}|{start}-{end})"))
            }
            Definition::Variant(variant) => {
                let name = variant.name(db).to_smol_str();
                let enum_signature =
                    self.get_signature(db, Definition::Adt(Adt::Enum(variant.parent_enum(db))))?;
                Some(format!("{enum_signature}::VARIANT({name})"))
            }
            _ => None,
        };
        if let Some(sig) = &signature {
            self.def_to_signature.insert(def, sig.clone());
        }
        signature
    }

    /// Attempt to generate the VName for a Definition's semantic parent
    fn get_parent_vname(&mut self, db: &RootDatabase, def: &Definition) -> Option<VName> {
        let parent_signature = match def {
            Definition::Adt(adt) => self.get_signature(db, Definition::Module(adt.module(db))),
            Definition::Const(const_) => {
                if let Some(assoc_item) = const_.as_assoc_item(db) {
                    match assoc_item.container(db) {
                        AssocItemContainer::Trait(t) => {
                            self.get_signature(db, Definition::Trait(t))
                        }
                        AssocItemContainer::Impl(i) => {
                            let self_type = i.self_ty(db);
                            if let Some(adt) = self_type.as_adt() {
                                self.get_signature(db, Definition::Adt(adt))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    self.get_signature(db, Definition::Module(const_.module(db)))
                }
            }
            Definition::Field(field) => {
                let parent_def = field.parent_def(db);
                match parent_def {
                    VariantDef::Struct(s) => {
                        self.get_signature(db, Definition::Adt(Adt::Struct(s)))
                    }
                    VariantDef::Union(u) => self.get_signature(db, Definition::Adt(Adt::Union(u))),
                    VariantDef::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                }
            }
            Definition::Function(function) => {
                if let Some(assoc_item) = function.as_assoc_item(db) {
                    match assoc_item.container(db) {
                        AssocItemContainer::Trait(t) => {
                            self.get_signature(db, Definition::Trait(t))
                        }
                        AssocItemContainer::Impl(i) => {
                            let self_type = i.self_ty(db);
                            if let Some(adt) = self_type.as_adt() {
                                self.get_signature(db, Definition::Adt(adt))
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    self.get_signature(db, Definition::Module(function.module(db)))
                }
            }
            Definition::Label(label) => match label.parent(db) {
                DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                _ => None,
            },
            Definition::Local(local) => match local.parent(db) {
                DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                DefWithBody::InTypeConst(_) => todo!(),
            },
            Definition::Macro(macro_) => {
                self.get_signature(db, Definition::Module(macro_.module(db)))
            }
            Definition::Static(static_) => {
                self.get_signature(db, Definition::Module(static_.module(db)))
            }
            Definition::Trait(trait_) => {
                self.get_signature(db, Definition::Module(trait_.module(db)))
            }
            Definition::TraitAlias(trait_alias) => {
                self.get_signature(db, Definition::Module(trait_alias.module(db)))
            }
            Definition::TypeAlias(talias) => {
                if let Some(assoc_item) = talias.as_assoc_item(db) {
                    match assoc_item.container(db) {
                        AssocItemContainer::Trait(t) => {
                            self.get_signature(db, Definition::Trait(t))
                        }
                        AssocItemContainer::Impl(i) => {
                            let self_type = i.self_ty(db);
                            if let Some(adt) = self_type.as_adt() {
                                self.get_signature(db, Definition::Adt(adt))
                            } else {
                                // TODO: Should probably emit a diagnostic node here
                                None
                            }
                        }
                    }
                } else {
                    self.get_signature(db, Definition::Module(talias.module(db)))
                }
            }
            Definition::Variant(variant) => {
                self.get_signature(db, Definition::Adt(Adt::Enum(variant.parent_enum(db))))
            }
            _ => None,
        }?;
        let mut parent_vname = self.gen_base_vname();
        parent_vname.set_signature(parent_signature);
        Some(parent_vname)
    }

    fn get_assoc_item_parent_signature(
        &mut self,
        db: &RootDatabase,
        aic: AssocItemContainer,
    ) -> Option<String> {
        match aic {
            AssocItemContainer::Trait(t) => self.get_signature(db, Definition::Trait(t)),
            AssocItemContainer::Impl(i) => {
                let module_signature = self.get_signature(db, Definition::Module(i.module(db)))?;
                let impl_range = i.source(db).unwrap().value.syntax().text_range();
                let impl_start = u32::from(impl_range.start());
                let impl_end = u32::from(impl_range.end());
                Some(format!("{module_signature}::IMPL({impl_start}-{impl_end}"))
            }
        }
    }

    fn visit_token(
        &mut self,
        semantics: &Semantics<'_, RootDatabase>,
        db: &RootDatabase,
        file_id: u32,
        token: SyntaxToken,
    ) -> Result<(), KytheError> {
        // Get information about the definition
        let def = get_definition(semantics, token.clone());
        if def.is_none() {
            // In the future we may want to return a diagnostic error but right now we won't
            // have definitions for the standard library so it would just spam
            return Ok(());
        }
        let def = def.unwrap();

        // Immediately return if the definition is for something we don't support yet
        if matches!(
            &def,
            Definition::BuiltinType(_)
                | Definition::SelfType(_)
                | Definition::DeriveHelper(_)
                | Definition::BuiltinAttr(_)
                | Definition::ToolModule(_)
        ) {
            return Ok(());
        }

        // Generate the VName for the node
        let mut def_vname = self.gen_base_vname();
        let def_signature = self.get_signature(db, def).ok_or_else(|| {
            KytheError::DiagnosticError(
                "Unable to generate VName signature for definition".to_string(),
            )
        })?;
        def_vname.set_signature(def_signature);

        // Generate the anchor VName
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        // Determine if this is a definition
        let def_range = get_definition_range(semantics, db, def).ok_or_else(|| {
            KytheError::DiagnosticError("Unable to find range for definition".to_string())
        })?;
        if file_id.eq(&def_range.file_id) && token_range.eq(&def_range.text_range) {
            // If this is a module definition, just immediately return because we emit
            // module definitions in `self.emit_modules()`
            if matches!(&def, Definition::Module(_)) {
                return Ok(());
            }

            // Emit the defines/binding edge between the anchor and the semantic node
            self.emitter.emit_edge(&anchor_vname, &def_vname, "/kythe/edge/defines/binding")?;

            // Collect the relevant facts to emit about the semantic node
            let mut facts: Vec<(&str, &[u8])> = Vec::new();
            match &def {
                Definition::Adt(adt) => match adt {
                    Adt::Enum(_) => {
                        facts.push(("/kythe/node/kind", b"sum"));
                        facts.push(("/kythe/complete", b"definition"));
                        facts.push(("/kythe/subkind", b"enum"));
                    }
                    Adt::Struct(_) => {
                        facts.push(("/kythe/node/kind", b"record"));
                        facts.push(("/kythe/complete", b"definition"));
                        facts.push(("/kythe/subkind", b"struct"));
                    }
                    Adt::Union(_) => {
                        facts.push(("/kythe/node/kind", b"record"));
                        facts.push(("/kythe/complete", b"definition"));
                        facts.push(("/kythe/subkind", b"union"));
                    }
                },
                Definition::Const(_) => {
                    facts.push(("/kythe/node/kind", b"constant"));
                }
                Definition::Field(_) => {
                    facts.push(("/kythe/node/kind", b"variable"));
                    facts.push(("/kythe/complete", b"definition"));
                    facts.push(("/kythe/subkind", b"field"));
                }
                Definition::Function(function) => {
                    facts.push(("/kythe/node/kind", b"function"));
                    if function.has_body(db) {
                        facts.push(("/kythe/complete", b"definition"));
                    } else {
                        facts.push(("/kythe/complete", b"incomplete"));
                    }
                }
                Definition::GenericParam(_) => {
                    facts.push(("/kythe/node/kind", b"tvar"));
                }
                Definition::Label(_) => {
                    facts.push(("/kythe/node/kind", b"variable"));
                    facts.push(("/kythe/complete", b"definition"));
                    facts.push(("/kythe/subkind", b"label"));
                }
                Definition::Local(_) => {
                    facts.push(("/kythe/node/kind", b"variable"));
                    facts.push(("/kythe/subkind", b"local"));
                }
                Definition::Macro(_) => {
                    facts.push(("/kythe/node/kind", b"macro"));
                }
                Definition::Static(static_) => {
                    facts.push(("/kythe/node/kind", b"variable"));
                    if static_.value(db).is_some() {
                        facts.push(("/kythe/complete", b"definition"));
                    } else {
                        facts.push(("/kythe/complete", b"incomplete"));
                    }
                    facts.push(("/kythe/subkind", b"static"));
                }
                Definition::Trait(_) => {
                    facts.push(("/kythe/node/kind", b"interface"));
                }
                Definition::TraitAlias(_) | Definition::TypeAlias(_) => {
                    facts.push(("/kythe/node/kind", b"talias"));
                }
                Definition::Variant(variant) => {
                    match variant.kind(db) {
                        StructKind::Tuple => {
                            facts.push(("/kythe/node/kind", b"record"));
                            facts.push(("/kythe/complete", b"definition"));
                            facts.push(("/kythe/subkind", b"tuplevariant"));
                        }
                        StructKind::Record => {
                            facts.push(("/kythe/node/kind", b"record"));
                            facts.push(("/kythe/complete", b"definition"));
                            facts.push(("/kythe/subkind", b"structvariant"));
                        }
                        StructKind::Unit => facts.push(("/kythe/node/kind", b"constant")),
                    };
                }
                _ => {}
            };

            // Emit all of the facts
            for (fact_name, fact_value) in facts.iter() {
                self.emitter.emit_fact(&def_vname, fact_name, fact_value.to_vec())?;
            }

            // Try to get a parent VName and emit a childof edge
            if let Some(parent_vname) = self.get_parent_vname(db, &def) {
                self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
            }

            // See if there is any documentation
            let docs_option = match def {
                Definition::Adt(adt) => docs_with_rangemap(db, &adt.attrs(db)),
                Definition::Const(const_) => docs_with_rangemap(db, &const_.attrs(db)),
                Definition::Field(field) => docs_with_rangemap(db, &field.attrs(db)),
                Definition::Function(function) => docs_with_rangemap(db, &function.attrs(db)),
                Definition::Macro(macro_) => docs_with_rangemap(db, &macro_.attrs(db)),
                Definition::Static(static_) => docs_with_rangemap(db, &static_.attrs(db)),
                Definition::Trait(trait_) => docs_with_rangemap(db, &trait_.attrs(db)),
                Definition::TraitAlias(trait_alias) => {
                    docs_with_rangemap(db, &trait_alias.attrs(db))
                }
                Definition::TypeAlias(talias) => docs_with_rangemap(db, &talias.attrs(db)),
                Definition::Variant(variant) => docs_with_rangemap(db, &variant.attrs(db)),
                _ => None,
            };
            if let Some((doc, range_map)) = docs_option {
                let mut doc_vname = def_vname.clone();
                doc_vname.set_signature(format!("{}::(DOC)", def_vname.get_signature()));
                self.emitter.emit_fact(&doc_vname, "/kythe/node/kind", b"doc".to_vec())?;
                self.emitter.emit_edge(&doc_vname, &def_vname, "/kythe/edge/documents")?;

                // Process the documentation, emit the text, and emit any params present in the
                // text
                let (doc_text, doc_refs) = process_documentation(&def, &doc, &range_map, db);
                self.emitter.emit_fact(&doc_vname, "/kythe/text", doc_text.into())?;
                for (i, doc_ref) in doc_refs.iter().enumerate() {
                    // Emit the doc param referencing the item in the link
                    if let Some(signature) = self.get_signature(db, doc_ref.reference) {
                        let mut ref_vname = self.gen_base_vname();
                        ref_vname.set_signature(signature);
                        let edge_kind = format!("/kythe/edge/param.{i}");
                        self.emitter.emit_edge(&doc_vname, &ref_vname, &edge_kind)?;

                        // If we got a range back for the link in the documentation, emit an anchor
                        // in the docs with a reference.
                        if let Some(range) = doc_ref.range {
                            let range_start = u32::from(range.start());
                            let range_end = u32::from(range.end());
                            let mut ref_anchor = anchor_vname.clone();
                            ref_anchor
                                .set_signature(format!("anchor_{range_start}_to_{range_end}"));

                            self.emitter.emit_fact(
                                &ref_anchor,
                                "/kythe/node/kind",
                                b"anchor".to_vec(),
                            )?;
                            self.emitter.emit_fact(
                                &ref_anchor,
                                "/kythe/loc/start",
                                range_start.to_string().into_bytes().to_vec(),
                            )?;
                            self.emitter.emit_fact(
                                &ref_anchor,
                                "/kythe/loc/end",
                                range_end.to_string().into_bytes().to_vec(),
                            )?;
                            self.emitter.emit_edge(
                                &ref_anchor,
                                &ref_vname,
                                "/kythe/edge/ref/doc",
                            )?;
                        }
                    }
                }
            }

            // Emit tparam edges if there are any
            let generic_params_opt = match def {
                Definition::Adt(adt) => Some(GenericDef::from(adt).params(db)),
                Definition::Const(const_) => Some(GenericDef::from(const_).params(db)),
                Definition::Function(function) => Some(GenericDef::from(function).params(db)),
                Definition::Trait(trait_) => Some(GenericDef::from(trait_).params(db)),
                Definition::TraitAlias(trait_alias) => {
                    Some(GenericDef::from(trait_alias).params(db))
                }
                Definition::TypeAlias(talias) => Some(GenericDef::from(talias).params(db)),
                Definition::Variant(variant) => Some(GenericDef::from(variant).params(db)),
                _ => None,
            };
            if let Some(generic_params) = generic_params_opt {
                let mut index = 0;
                for param in generic_params.into_iter() {
                    // Skip implicit type params
                    if let GenericParam::TypeParam(type_param) = &param {
                        if type_param.is_implicit(db) {
                            continue;
                        }
                    }

                    // Generate VName for GenericParam
                    let mut tparam_vname = self.gen_base_vname();
                    let tparam_signature_opt =
                        self.get_signature(db, Definition::GenericParam(param));
                    if tparam_signature_opt.is_none() {
                        index += 1;
                        continue;
                    }
                    let tparam_signature = tparam_signature_opt.unwrap();
                    tparam_vname.set_signature(tparam_signature);

                    // Emit the tparam edge
                    let edge_kind = format!("/kythe/edge/tparam.{index}");
                    self.emitter.emit_edge(&def_vname, &tparam_vname, &edge_kind)?;
                    index += 1;
                }
            }

            // Emit param edges if this is a function
            if let Definition::Function(function) = &def {
                let mut index = 0;
                // I hate this code but it's necessary. You can only get the self param through
                // .self_param(). Then, you have to manually get the definition located at the
                // last token of the node (which should be the actual "self" token) because
                // there isn't a helper function to convert the SelfParam to a local. And of
                // course most of these functions return Options and we don't want this to crash
                // the indexer so we end up with 4 nested if statements.
                if let Some(self_param) = function.self_param(db) {
                    if let Some(source) = self_param.source(db) {
                        let self_token = source.value.syntax().last_token().unwrap();
                        let local_def = IdentClass::classify_token(semantics, &self_token)
                            .map(IdentClass::definitions_no_ops);
                        if let Some(&[local_def]) = local_def.as_deref() {
                            let mut param_vname = self.gen_base_vname();
                            if let Some(param_signature) = self.get_signature(db, local_def) {
                                param_vname.set_signature(param_signature);
                                let edge_kind = format!("/kythe/edge/param.{index}");
                                self.emitter.emit_edge(&def_vname, &param_vname, &edge_kind)?;
                                index += 1;
                            }
                        } else {
                            // TODO: Might want to emit a diagnostic node that
                            // we couldn't find a
                            // Definition::Local for the self param
                        }
                    }
                }
                for param in function.params_without_self(db) {
                    // Try to convert the param to a local and create the VName
                    let local = param.as_local(db);
                    if local.is_none() {
                        continue;
                    }
                    let mut param_vname = self.gen_base_vname();
                    let param_signature = self.get_signature(db, Definition::Local(local.unwrap()));
                    if param_signature.is_none() {
                        continue;
                    }
                    param_vname.set_signature(param_signature.unwrap());

                    // Emit the param edge
                    let edge_kind = format!("/kythe/edge/param.{index}");
                    self.emitter.emit_edge(&def_vname, &param_vname, &edge_kind)?;
                    index += 1;
                }
            }

            // Generate and emit MarkedSource
            if let Some(marked_source) = self.gen_marked_source(def, db) {
                self.emitter.emit_fact(&def_vname, "/kythe/code", marked_source)?;
            }
        } else {
            // This is a reference, so emit the corresponding edge
            let edge_kind = if matches!(&def, Definition::Macro(_)) {
                "/kythe/edge/ref/expands"
            } else if matches!(&def, Definition::Function(_)) {
                "/kythe/edge/ref/call"
            } else {
                "/kythe/edge/ref"
            };
            self.emitter.emit_edge(&anchor_vname, &def_vname, edge_kind)?;
        }

        // Emit the anchor facts. We do it here instead of before the if-statement
        // because of the nested if-statement that returns immediately if this is a
        // module definition.
        self.emitter.emit_fact(&anchor_vname, "/kythe/node/kind", b"anchor".to_vec())?;
        self.emitter.emit_fact(
            &anchor_vname,
            "/kythe/loc/start",
            token_range_start.to_string().into_bytes().to_vec(),
        )?;
        self.emitter.emit_fact(
            &anchor_vname,
            "/kythe/loc/end",
            token_range_end.to_string().into_bytes().to_vec(),
        )?;

        Ok(())
    }

    fn gen_base_vname(&self) -> VName {
        let mut vname = VName::new();
        vname.set_corpus(self.unit_storage_vname.get_corpus().to_string());
        vname.set_language("rust".to_string());
        vname
    }

    fn get_rust_project_file(&mut self) -> Result<String, KytheError> {
        for required_input in self.unit.get_required_input() {
            let input_path = required_input.get_info().get_path();

            if input_path.eq("kythe-rust-project.json") {
                let digest = required_input.get_info().get_digest();
                let file_bytes = self.provider.contents(input_path, digest)?;
                let file_string = String::from_utf8_lossy(&file_bytes);
                return Ok(file_string.to_string());
            }
        }

        Err(KytheError::IndexerError(
            "The kythe-rust-project.json file could not be found in the Compilation Unit"
                .to_string(),
        ))
    }

    fn marked_source_identifier(
        &mut self,
        def: Definition,
        db: &RootDatabase,
    ) -> Option<MarkedSource> {
        // Set the name on the identifier
        let name = def.name(db)?;
        let mut identifier = MarkedSource::new();
        identifier.set_kind(MarkedSource_Kind::IDENTIFIER);
        identifier.set_pre_text(name.to_smol_str().to_string());

        // Set the link on the identifier
        let mut vname = self.gen_base_vname();
        vname.set_signature(self.get_signature(db, def)?);
        let mut link = Link::new();
        link.set_definition(RepeatedField::from_vec(vec![vname_to_kythe_uri(&vname)]));
        identifier.set_link(RepeatedField::from_vec(vec![link]));

        Some(identifier)
    }

    fn gen_marked_source(&mut self, def: Definition, db: &RootDatabase) -> Option<Vec<u8>> {
        let ms_children: Vec<MarkedSource> = match &def {
            Definition::Adt(_)
            | Definition::Trait(_)
            | Definition::TraitAlias(_)
            | Definition::TypeAlias(_) => {
                let mut children = vec![];

                // Add pub modifier if this is public
                if let Some(visibility) = def.visibility(db) {
                    if visibility == Visibility::Public {
                        let mut public_modifier = MarkedSource::new();
                        public_modifier.set_kind(MarkedSource_Kind::MODIFIER);
                        public_modifier.set_pre_text("pub ".into());
                        children.push(public_modifier);
                    }
                }

                // Add ADT type or "trait" as a modifier
                let mut modifier = MarkedSource::new();
                let modifier_type = match &def {
                    Definition::Adt(adt) => match &adt {
                        Adt::Enum(_) => "enum",
                        Adt::Struct(_) => "struct",
                        Adt::Union(_) => "union",
                    },
                    Definition::Trait(_) | Definition::TraitAlias(_) => "trait",
                    Definition::TypeAlias(_) => "type",
                    _ => unreachable!(),
                };
                modifier.set_kind(MarkedSource_Kind::MODIFIER);
                modifier.set_pre_text(modifier_type.into());
                modifier.set_post_text(" ".into());
                children.push(modifier);

                // Add identifier
                let identifier = self.marked_source_identifier(def, db)?;
                children.push(identifier);

                // Add type parameters if any are present
                let generic_params = match &def {
                    Definition::Adt(adt) => GenericDef::from(*adt),
                    Definition::Trait(trait_) => GenericDef::from(*trait_),
                    Definition::TraitAlias(trait_alias) => GenericDef::from(*trait_alias),
                    Definition::TypeAlias(type_alias) => GenericDef::from(*type_alias),
                    _ => unreachable!(),
                }
                .params(db);

                if !generic_params.is_empty() {
                    let mut has_non_implicit_params = false;
                    for param in generic_params.into_iter() {
                        if let GenericParam::TypeParam(tparam) = param {
                            if tparam.is_implicit(db) {
                                continue;
                            }
                        }
                        has_non_implicit_params = true;
                        break;
                    }

                    if has_non_implicit_params {
                        let mut tparam_ms = MarkedSource::new();
                        tparam_ms.set_kind(MarkedSource_Kind::PARAMETER_LOOKUP_BY_TPARAM);
                        tparam_ms.set_pre_text("<".into());
                        tparam_ms.set_post_text(">".into());
                        tparam_ms.set_post_child_text(", ".into());
                        children.push(tparam_ms);
                    }
                }

                Some(children)
            }
            Definition::Const(_) | Definition::Static(_) => {
                let mut children = vec![];

                // Add pub modifier if this is public
                if let Some(visibility) = def.visibility(db) {
                    if visibility == Visibility::Public {
                        let mut public_modifier = MarkedSource::new();
                        public_modifier.set_kind(MarkedSource_Kind::MODIFIER);
                        public_modifier.set_pre_text("pub ".into());
                        children.push(public_modifier);
                    }
                }

                // Add const/static modifier
                let mut modifier = MarkedSource::new();
                let modifier_type = match &def {
                    Definition::Const(_) => "const",
                    Definition::Static(_) => "static",
                    _ => unreachable!(),
                };
                modifier.set_kind(MarkedSource_Kind::MODIFIER);
                modifier.set_pre_text(modifier_type.into());
                modifier.set_post_text(" ".into());
                children.push(modifier);

                // Add identifier and type
                let mut ms = MarkedSource::new();
                ms.set_kind(MarkedSource_Kind::BOX);
                ms.set_post_child_text(": ".into());
                let identifier = self.marked_source_identifier(def, db)?;
                let ty = match &def {
                    Definition::Const(konst) => konst.ty(db),
                    Definition::Static(statik) => statik.ty(db),
                    _ => unreachable!(),
                };
                let mut type_ms = MarkedSource::new();
                type_ms.set_kind(MarkedSource_Kind::TYPE);
                type_ms.set_pre_text(format!("{}", ty.display(db)));
                ms.set_child(RepeatedField::from_vec(vec![identifier, type_ms]));
                children.push(ms);

                // Add initializer if possible
                let value_opt = match &def {
                    Definition::Const(konst) => konst.value(db),
                    Definition::Static(statik) => statik.value(db),
                    _ => unreachable!(),
                };
                if let Some(value) = value_opt {
                    let mut initializer = MarkedSource::new();
                    initializer.set_kind(MarkedSource_Kind::INITIALIZER);
                    initializer.set_pre_text(format!("{value}"));
                    children.push(initializer);
                }

                Some(children)
            }
            Definition::Field(field) => {
                let mut children = vec![];

                // Add pub modifier if this is public
                if let Some(visibility) = def.visibility(db) {
                    if visibility == Visibility::Public {
                        let mut public_modifier = MarkedSource::new();
                        public_modifier.set_kind(MarkedSource_Kind::MODIFIER);
                        public_modifier.set_pre_text("pub ".into());
                        children.push(public_modifier);
                    }
                }

                // Add identifier
                let identifier = self.marked_source_identifier(def, db)?;
                children.push(identifier);

                // Add type
                let mut type_ms = MarkedSource::new();
                type_ms.set_kind(MarkedSource_Kind::TYPE);
                type_ms.set_pre_text(": ".into());
                type_ms.set_post_text(format!("{}", field.ty(db).display(db)));
                children.push(type_ms);

                Some(children)
            }
            Definition::Function(function) => {
                let mut children = vec![];

                // Create the modifier, including certain prefixes as necessary
                let mut modifier = String::new();
                if let Some(visibility) = def.visibility(db) {
                    if visibility == Visibility::Public {
                        modifier = format!("{modifier}pub ");
                    }
                }
                if function.is_const(db) {
                    modifier = format!("{modifier}const ");
                }
                if function.is_async(db) {
                    modifier = format!("{modifier}async ");
                }
                if function.is_unsafe_to_call(db) {
                    modifier = format!("{modifier}unsafe ");
                }
                modifier = format!("{modifier}fn ");
                let mut modifier_ms = MarkedSource::new();
                modifier_ms.set_kind(MarkedSource_Kind::MODIFIER);
                modifier_ms.set_pre_text(modifier);
                children.push(modifier_ms);

                // Add the identifier
                let identifier = self.marked_source_identifier(def, db)?;
                children.push(identifier);

                // Add the generic params if present
                let generic_def = GenericDef::from(*function);
                let generic_params = generic_def.params(db);
                if !generic_params.is_empty() {
                    let mut has_non_implicit_params = false;
                    for param in generic_params.into_iter() {
                        if let GenericParam::TypeParam(tparam) = param {
                            if tparam.is_implicit(db) {
                                continue;
                            }
                        }
                        has_non_implicit_params = true;
                        break;
                    }

                    if has_non_implicit_params {
                        let mut tparam_ms = MarkedSource::new();
                        tparam_ms.set_kind(MarkedSource_Kind::PARAMETER_LOOKUP_BY_TPARAM);
                        tparam_ms.set_pre_text("<".into());
                        tparam_ms.set_post_text(">".into());
                        tparam_ms.set_post_child_text(", ".into());
                        children.push(tparam_ms);
                    }
                }

                // Add regular params if present
                let mut param_ms = MarkedSource::new();
                param_ms.set_kind(MarkedSource_Kind::PARAMETER);
                param_ms.set_pre_text("(".into());
                param_ms.set_post_text(")".into());
                if function.num_params(db) > 0 {
                    param_ms.set_kind(MarkedSource_Kind::PARAMETER_LOOKUP_BY_PARAM);
                    param_ms.set_post_child_text(", ".into());
                }
                children.push(param_ms);

                // Add the return type if it isn't the unit type
                let return_type = if let Some(ty) = function.async_ret_type(db) {
                    ty
                } else {
                    function.ret_type(db)
                };
                if !return_type.is_unit() {
                    let mut arrow_ms = MarkedSource::new();
                    arrow_ms.set_kind(MarkedSource_Kind::MODIFIER);
                    arrow_ms.set_pre_text(" -> ".into());
                    children.push(arrow_ms);

                    let mut type_ms = MarkedSource::new();
                    type_ms.set_kind(MarkedSource_Kind::TYPE);
                    type_ms.set_pre_text(format!("{}", return_type.display(db)));
                    children.push(type_ms);
                }

                Some(children)
            }
            Definition::GenericParam(param) => {
                let mut children = vec![];
                if let GenericParam::ConstParam(cparam) = param {
                    // Add const modifier
                    let mut modifier = MarkedSource::new();
                    modifier.set_kind(MarkedSource_Kind::MODIFIER);
                    modifier.set_pre_text("const ".into());
                    children.push(modifier);

                    // Add box with identifier and type
                    let mut ms_box = MarkedSource::new();
                    ms_box.set_kind(MarkedSource_Kind::BOX);
                    let identifier = self.marked_source_identifier(def, db)?;
                    let mut type_ms = MarkedSource::new();
                    type_ms.set_kind(MarkedSource_Kind::TYPE);
                    type_ms.set_pre_text(format!(": {}", cparam.ty(db).display(db)));
                    ms_box.set_child(RepeatedField::from_vec(vec![identifier, type_ms]));
                    children.push(ms_box);
                } else {
                    children.push(self.marked_source_identifier(def, db)?);
                }
                Some(children)
            }
            Definition::Local(local) => {
                let mut children = vec![];
                if let Some(self_param) = local.as_self_param(db) {
                    let modifier = match self_param.access(db) {
                        Access::Shared => "&",
                        Access::Exclusive => "&mut ",
                        Access::Owned => "",
                    };
                    if !modifier.is_empty() {
                        let mut mod_ms = MarkedSource::new();
                        mod_ms.set_kind(MarkedSource_Kind::MODIFIER);
                        mod_ms.set_pre_text(modifier.into());
                        children.push(mod_ms);
                    }

                    let identifier = self.marked_source_identifier(def, db)?;
                    children.push(identifier);
                } else {
                    // Add mut modifier is the local is mutable
                    if local.is_mut(db) {
                        let mut mod_ms = MarkedSource::new();
                        mod_ms.set_kind(MarkedSource_Kind::MODIFIER);
                        mod_ms.set_pre_text("mut ".into());
                        children.push(mod_ms);
                    }

                    // Create box
                    let mut box_ms = MarkedSource::new();
                    box_ms.set_kind(MarkedSource_Kind::BOX);
                    box_ms.set_post_child_text(": ".into());

                    // Add identifier
                    let identifier = self.marked_source_identifier(def, db)?;

                    // Add type
                    let mut type_ms = MarkedSource::new();
                    type_ms.set_kind(MarkedSource_Kind::TYPE);
                    type_ms.set_pre_text(format!("{}", local.ty(db).display(db)));

                    box_ms.set_child(RepeatedField::from_vec(vec![identifier, type_ms]));
                    children.push(box_ms);
                }
                Some(children)
            }
            Definition::Macro(macro_) => {
                if macro_.kind(db) == MacroKind::Declarative {
                    let mut modifier = MarkedSource::new();
                    modifier.set_kind(MarkedSource_Kind::MODIFIER);
                    modifier.set_pre_text("macro_rules! ".into());
                    Some(vec![modifier, self.marked_source_identifier(def, db)?])
                } else {
                    None
                }
            }
            Definition::Module(_) => {
                let mut children = vec![];

                // Add pub modifier if this is public
                if let Some(visibility) = def.visibility(db) {
                    if visibility == Visibility::Public {
                        let mut public_modifier = MarkedSource::new();
                        public_modifier.set_kind(MarkedSource_Kind::MODIFIER);
                        public_modifier.set_pre_text("pub ".into());
                        children.push(public_modifier);
                    }
                }

                let mut modifier = MarkedSource::new();
                modifier.set_kind(MarkedSource_Kind::MODIFIER);
                modifier.set_pre_text("mod ".into());
                children.push(modifier);

                children.push(self.marked_source_identifier(def, db)?);
                Some(children)
            }
            _ => None,
        }?;

        let marked_source = if ms_children.len() == 1 {
            ms_children[0].to_owned()
        } else {
            let mut ms = MarkedSource::new();
            ms.set_kind(MarkedSource_Kind::BOX);
            ms.set_child(RepeatedField::from_vec(ms_children));
            ms
        };
        marked_source.write_to_bytes().ok()
    }
}

/// Convert a VName from analysis_rust_proto to a VName from storage_rust_proto
fn analysis_to_storage_vname(analysis_vname: &analysis_rust_proto::VName) -> VName {
    let mut vname = VName::new();
    vname.set_signature(analysis_vname.get_signature().to_string());
    vname.set_corpus(analysis_vname.get_corpus().to_string());
    vname.set_root(analysis_vname.get_root().to_string());
    vname.set_path(analysis_vname.get_path().to_string());
    vname.set_language(analysis_vname.get_language().to_string());
    vname
}

/// Returns the first root module from the crates in the database where the file
/// id of the root module is present in the provided slice of u32 file ids
fn get_root_module_in_file_ids(db: &RootDatabase, file_ids: &[u32]) -> Option<Module> {
    let root_modules: Vec<Module> =
        Crate::all(db).into_iter().map(|krate| krate.root_module()).collect();
    for module in root_modules {
        let def_source = module.definition_source(db);
        let file_id = def_source.file_id.original_file(db);
        if file_ids.contains(&file_id.0) {
            return Some(module);
        }
    }
    None
}

fn get_definition_range(
    semantics: &Semantics<'_, RootDatabase>,
    db: &RootDatabase,
    def: Definition,
) -> Option<FileRange> {
    match def {
        Definition::Adt(adt) => {
            let source = semantics.source(adt)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Const(const_) => {
            let source = semantics.source(const_)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Field(field) => {
            let source = field.source(db)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node = match source.value {
                FieldSource::Named(f) => {
                    f.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)
                }
                _ => None,
            }?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Function(function) => {
            let source = semantics.source(function)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::GenericParam(param) => match param {
            GenericParam::ConstParam(cparam) => {
                let source = semantics.source(cparam.merge())?;
                let def_file_id = source.file_id.original_file(db);
                let name_node =
                    source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
                Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
            }
            GenericParam::LifetimeParam(lparam) => {
                let source = semantics.source(lparam)?;
                let def_file_id = source.file_id.original_file(db);
                let name_node = source
                    .value
                    .syntax()
                    .children()
                    .find(|it| it.kind() == SyntaxKind::LIFETIME)?;
                Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
            }
            GenericParam::TypeParam(tparam) => {
                let source = semantics.source(tparam.merge())?;
                let def_file_id = source.file_id.original_file(db);
                let name_node =
                    source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
                Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
            }
        },
        Definition::Label(label) => {
            let source = label.source(db);
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::LIFETIME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Local(local) => {
            let source = local.primary_source(db);
            let def_file_id = source.source.file_id.original_file(db);
            let name_node = source.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Macro(macro_) => {
            let source = semantics.source(macro_)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Module(module) => {
            let source = module.definition_source(db);
            let def_file_id = source.file_id.original_file(db);
            let text_range = match source.value {
                ModuleSource::Module(ast_module) => {
                    let name_node =
                        ast_module.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
                    Some(name_node.text_range())
                }
                ModuleSource::SourceFile(_) => {
                    Some(TextRange::new(TextSize::from(0), TextSize::from(0)))
                }
                _ => None,
            }?;
            Some(FileRange { file_id: def_file_id.0, text_range })
        }
        Definition::Static(static_) => {
            let source = semantics.source(static_)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Trait(trait_) => {
            let source = semantics.source(trait_)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::TraitAlias(trait_alias) => {
            let source = semantics.source(trait_alias)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::TypeAlias(talias) => {
            let source = semantics.source(talias)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        Definition::Variant(variant) => {
            let source = semantics.source(variant)?;
            let def_file_id = source.file_id.original_file(db);
            let name_node =
                source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME)?;
            Some(FileRange { file_id: def_file_id.0, text_range: name_node.text_range() })
        }
        _ => None,
    }
}
