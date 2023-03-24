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

use super::entries::EntryEmitter;

use analysis_rust_proto::CompilationUnit;
use ra_ap_hir::{
    Adt, AsAssocItem, AssocItemContainer, Const, Crate, DefWithBody, Field, FieldSource, Function,
    HasSource, InFile, Label, Local, Macro, Module, ModuleSource, Semantics, Static, StructKind,
    Trait, TypeAlias, Variant, VariantDef,
};
use ra_ap_ide::{AnalysisHost, Change, RootDatabase, SourceRoot};
use ra_ap_ide_db::defs::{Definition, IdentClass};
use ra_ap_paths::AbsPath;
use ra_ap_project_model::{ProjectJson, ProjectJsonData, ProjectWorkspace};
use ra_ap_syntax::{
    ast::{AstNode, HasName},
    NodeOrToken, SyntaxKind, SyntaxToken, T,
};
use ra_ap_vfs::{file_set::FileSetConfigBuilder, Vfs, VfsPath};
use rustc_hash::FxHashMap;
use storage_rust_proto::*;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

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
        let rust_project = ProjectJson::new(project_root.clone(), rust_project_data);

        // Create the project workspace from the project
        let extra_env: FxHashMap<String, String> = FxHashMap::default();
        let workspace =
            ProjectWorkspace::load_inline(rust_project, None, &extra_env).map_err(|e| {
                KytheError::IndexerError(format!("Failed to create Rust project workspace: {e}"))
            })?;

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
            analysis_change.change_file(file_id, Some(Arc::new(text)));

            // Add file information to relevant hashmaps
            self.file_id_to_path.insert(file_id.0, path.clone());
            let vname = required_input.get_v_name();
            self.file_id_to_vname.insert(file_id.0, analysis_to_storage_vname(&vname));

            // Add the file id to the list of source file ids if this is a source file for
            // the root crate
            if source_files.contains(&path) {
                source_file_ids.push(file_id.0)
            }
        }

        // Generate and set the crate graph
        let crate_graph = workspace.to_crate_graph(
            &mut |_, _| Err("proc_macro_client is disabled".to_string()),
            &mut |path: &AbsPath| {
                let source_path =
                    path.strip_prefix(&project_root).unwrap().as_ref().display().to_string();
                if let Some(file_digest) = self.file_digests.get(&source_path) {
                    let file_bytes = self.provider.contents(&source_path, file_digest).ok();
                    let vfs_path = VfsPath::from(path.to_path_buf());
                    vfs.set_file_contents(vfs_path.clone(), file_bytes);
                    vfs.file_id(&vfs_path)
                } else {
                    None
                }
            },
            &extra_env,
        );
        analysis_change.set_crate_graph(crate_graph);

        // Generate and set the roots
        let mut fsc_builder = FileSetConfigBuilder::default();
        let mut local_filesets = Vec::new();
        let workspace_roots = workspace.to_roots();
        for root in workspace_roots {
            let mut paths = Vec::new();
            for path in root.include {
                paths.push(VfsPath::from(path));
            }
            if root.is_local {
                local_filesets.push(fsc_builder.len());
            }
            fsc_builder.add_file_set(paths);
        }
        let fsc = fsc_builder.build();
        let source_roots: Vec<SourceRoot> = fsc
            .partition(&vfs)
            .iter()
            .enumerate()
            .map(|(idx, file_set)| {
                let is_local = local_filesets.contains(&idx);
                if is_local {
                    SourceRoot::new_local(file_set.clone())
                } else {
                    SourceRoot::new_library(file_set.clone())
                }
            })
            .collect();
        analysis_change.set_roots(source_roots);

        // Create the analysis host and apply the change
        let mut analysis_host = AnalysisHost::new(None);
        analysis_host.apply_change(analysis_change);

        // Get the rust-analyzer database
        let db = analysis_host.raw_database();

        // Get the root module of the crate being analyzed
        let root_module = get_root_module_in_file_ids(&db, &source_file_ids).ok_or_else(|| {
            KytheError::IndexerError(
                "Failed to find root module for crate being indexed".to_string(),
            )
        })?;
        let krate = root_module.krate();

        // Emit nodes for all of the modules in the crate
        self.emit_modules(&db, root_module.clone())?;

        // Analyze all source files
        let semantics = Semantics::new(db.clone());
        for file_id in source_file_ids {
            let tokens = semantics
                .parse(ra_ap_ide::FileId(file_id))
                .syntax()
                .descendants_with_tokens()
                .filter_map(|x| match x {
                    NodeOrToken::Node(_) => None,
                    NodeOrToken::Token(x) => Some(x),
                })
                .filter(|token| {
                    matches!(
                        token.kind(),
                        SyntaxKind::IDENT
                            | SyntaxKind::LIFETIME_IDENT
                            | SyntaxKind::CRATE_KW
                            | SyntaxKind::SUPER_KW
                            | T![self]
                            | T![Self]
                    )
                });

            for token in tokens {
                if let Some(def) = get_definition(&semantics, token.clone()) {
                    match def {
                        Definition::Adt(adt) => {
                            self.visit_adt_ident(&db, &semantics, file_id, &token, &adt)?
                        }
                        Definition::Const(konst) => {
                            self.visit_const_ident(&db, &semantics, file_id, &token, &konst)?
                        }
                        Definition::Field(field) => {
                            self.visit_field_ident(&db, file_id, &token, &field)?
                        }
                        Definition::Function(function) => {
                            self.visit_function_ident(&db, &semantics, file_id, &token, &function)?
                        }
                        Definition::Label(label) => {
                            self.visit_label_ident(&db, file_id, &token, &label)?
                        }
                        Definition::Local(local) => {
                            self.visit_local_ident(&db, file_id, &token, &local)?
                        }
                        Definition::Macro(makro) => {
                            self.visit_macro_ident(&db, &semantics, file_id, &token, &makro)?
                        }
                        Definition::Module(module) => {
                            self.visit_module_ident(&db, file_id, &token, &module, &krate)?
                        }
                        Definition::Static(statik) => {
                            self.visit_static_ident(&db, &semantics, file_id, &token, &statik)?
                        }
                        Definition::Trait(trate) => {
                            self.visit_trait_ident(&db, &semantics, file_id, &token, &trate)?
                        }
                        Definition::TypeAlias(talias) => {
                            self.visit_talias_ident(&db, &semantics, file_id, &token, &talias)?
                        }
                        Definition::Variant(variant) => {
                            self.visit_variant_ident(&db, &semantics, file_id, &token, &variant)?
                        }
                        _ => {}
                    }
                }
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
            module_to_vname.insert(module.clone(), def_vname.clone());
            self.def_to_signature
                .insert(Definition::Module(module.clone()), def_vname.get_signature().to_string());

            // Emit the facts about the module
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"record".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/subkind", b"module".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/complete", b"definition".to_vec())?;

            // Emit the childof edge to the parent module if there is one
            if let Some(vname) = parent_vname {
                self.emitter.emit_edge(&def_vname, &vname, "/kythe/edge/childof")?;
            }

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
                ModuleSource::Module(module) => {
                    let name = module.name().unwrap();
                    let range = InFile::new(def_source.file_id.clone(), name.syntax())
                        .original_file_range_opt(db)
                        .map(|it| it.range);
                    if let Some(range) = range {
                        let start = u32::from(range.start());
                        let end = u32::from(range.end());
                        self.emitter.emit_anchor(&anchor_vname, &def_vname, start, end)?;
                    } else {
                        // TODO: We'll have to emit some diagnostic about not
                        // being able to find the
                        // identifier
                    }
                }
                // Not sure when a module would be defined in a block expression but we'll ignore it
                // for the time being
                _ => {}
            };
        }

        Ok(())
    }

    /// Visits an identifier for a user-defined type. If this is the definition,
    /// emits node information and an anchor. If this is a reference, emits
    /// a reference.
    fn visit_adt_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        adt: &Adt,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = semantics.source(adt.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the parent module
        let mut module_vname = self.gen_base_vname();
        let module_signature = self.get_signature(db, Definition::Module(adt.module(db)));
        module_vname.set_signature(module_signature.clone());

        // Create the VName for the const
        let mut def_vname = self.gen_base_vname();
        let signature = self.get_signature(db, Definition::Adt(adt.clone()));
        def_vname.set_signature(signature);

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            let mut facts: Vec<(&str, &[u8])> = Vec::new();
            match adt {
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
            }
            for (fact_name, fact_value) in facts.iter() {
                self.emitter.emit_fact(&def_vname, fact_name, fact_value.to_vec())?;
            }
            self.emitter.emit_edge(&def_vname, &module_vname, "/kythe/edge/childof")?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a constant. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_const_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        const_: &Const,
    ) -> Result<(), KytheError> {
        // If the constant has no name, it cannot be referenced so we just don't do
        // anything. We should probably still emit a definition but I won't deal with it
        // for now.
        if const_.name(db).is_none() {
            return Ok(());
        }

        // Get the definition range
        let source = semantics.source(const_.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the const
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Const(const_.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"constant".to_vec())?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;

            // Try to generate a VName signature for the const's parent node
            let parent_signature = if let Some(assoc_item) = const_.as_assoc_item(db) {
                match assoc_item.container(db) {
                    AssocItemContainer::Trait(t) => {
                        Some(self.get_signature(db, Definition::Trait(t)))
                    }
                    AssocItemContainer::Impl(i) => {
                        let self_type = i.self_ty(db);
                        if let Some(adt) = self_type.as_adt() {
                            Some(self.get_signature(db, Definition::Adt(adt)))
                        } else {
                            // TODO: Should probably emit a diagnostic node here
                            None
                        }
                    }
                }
            } else {
                Some(self.get_signature(db, Definition::Module(const_.module(db))))
            };
            // If we successfully generated a signature for the parent, emit a childof edge
            // to it
            if let Some(parent_signature) = parent_signature {
                let mut parent_vname = self.gen_base_vname();
                parent_vname.set_signature(parent_signature);
                self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
            }
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a field. If this is the definition,
    /// emits node information and an anchor. If this is a reference, emits
    /// a reference.
    fn visit_field_ident(
        &mut self,
        db: &RootDatabase,
        file_id: u32,
        token: &SyntaxToken,
        field: &Field,
    ) -> Result<(), KytheError> {
        let field_name = field.name(db).to_smol_str();

        // Get the definition range
        let source = field.source(db).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = match source.value {
            FieldSource::Named(f) => f.syntax().children().find(|it| it.kind() == SyntaxKind::NAME),
            _ => None,
        };
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the parent
        let mut parent_vname = self.gen_base_vname();
        let parent_def = field.parent_def(db);
        let parent_signature = match parent_def {
            VariantDef::Struct(s) => {
                self.get_signature(db, Definition::Adt(Adt::Struct(s.clone())))
            }
            VariantDef::Union(u) => self.get_signature(db, Definition::Adt(Adt::Union(u.clone()))),
            VariantDef::Variant(v) => self.get_signature(db, Definition::Variant(v.clone())),
        };
        parent_vname.set_signature(parent_signature.clone());

        // Create the VName for the const
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(format!("{parent_signature}::FIELD({field_name}"));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"variable".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/complete", b"definition".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/subkind", b"field".to_vec())?;
            self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a function. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_function_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        function: &Function,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = semantics.source(function.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the macro
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Function(function.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"function".to_vec())?;
            let complete_value =
                if function.has_body(db) { b"definition".to_vec() } else { b"incomplete".to_vec() };
            self.emitter.emit_fact(&def_vname, "/kythe/complete", complete_value)?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;

            // Try to generate a VName signature for the function's parent node
            let parent_signature = if let Some(assoc_item) = function.as_assoc_item(db) {
                match assoc_item.container(db) {
                    AssocItemContainer::Trait(t) => {
                        Some(self.get_signature(db, Definition::Trait(t)))
                    }
                    AssocItemContainer::Impl(i) => {
                        let self_type = i.self_ty(db);
                        if let Some(adt) = self_type.as_adt() {
                            Some(self.get_signature(db, Definition::Adt(adt)))
                        } else {
                            // TODO: Should probably emit a diagnostic node here
                            None
                        }
                    }
                }
            } else {
                Some(self.get_signature(db, Definition::Module(function.module(db))))
            };
            // If we successfully generated a signature for the parent, emit a childof edge
            // to it
            if let Some(parent_signature) = parent_signature {
                let mut parent_vname = self.gen_base_vname();
                parent_vname.set_signature(parent_signature);
                self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
            }
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a label. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_label_ident(
        &mut self,
        db: &RootDatabase,
        file_id: u32,
        token: &SyntaxToken,
        label: &Label,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = label.source(db);
        let source_file_id = source.file_id.original_file(db);
        let name_node =
            source.value.syntax().children().find(|it| it.kind() == SyntaxKind::LIFETIME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the label
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Label(label.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"variable".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/complete", b"definition".to_vec())?;
            self.emitter.emit_fact(&def_vname, "/kythe/subkind", b"label".to_vec())?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;

            // Create the parent VName and emit a childof node
            let mut parent_vname = self.gen_base_vname();
            let parent_signature = match label.parent(db) {
                DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
            };
            parent_vname.set_signature(parent_signature);
            self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a local. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_local_ident(
        &mut self,
        db: &RootDatabase,
        file_id: u32,
        token: &SyntaxToken,
        local: &Local,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = local.source(db);
        let source_file_id = source.file_id.original_file(db);
        let name_node = if source.value.is_left() {
            source.value.unwrap_left().syntax().children().find(|it| it.kind() == SyntaxKind::NAME)
        } else {
            source.value.unwrap_right().syntax().children().find(|it| it.kind() == SyntaxKind::NAME)
        };
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the label
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Local(local.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"variable".to_vec())?;
            // TODO: Determine whether this is a declaration or a definition
            self.emitter.emit_fact(&def_vname, "/kythe/subkind", b"local".to_vec())?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;

            // Create the parent VName and emit a childof node
            let mut parent_vname = self.gen_base_vname();
            let parent_signature = match local.parent(db) {
                DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
            };
            parent_vname.set_signature(parent_signature);
            self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a macro. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_macro_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        makro: &Macro,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = semantics.source(makro.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the parent module
        let mut module_vname = self.gen_base_vname();
        module_vname.set_signature(self.get_signature(db, Definition::Module(makro.module(db))));

        // Create the VName for the macro
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Macro(makro.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"macro".to_vec())?;
            self.emitter.emit_edge(&def_vname, &module_vname, "/kythe/edge/childof")?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        } else {
            // Emit an anchor and a ref/expands
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
            self.emitter.emit_edge(&anchor_vname, &def_vname, "/kythe/edge/ref/expands")?
        }

        Ok(())
    }

    /// Visits a module identifier in and, if it is a reference, emits the
    /// information to the Kythe graph
    fn visit_module_ident(
        &mut self,
        db: &RootDatabase,
        file_id: u32,
        token: &SyntaxToken,
        module: &Module,
        root_crate: &Crate,
    ) -> Result<(), KytheError> {
        let range = token.text_range();
        let def_source = module.definition_source(db);

        // If the module is part of the crate being analyzed and is declared as a `mod
        // foo {}` rather than as a whole file, we need to determine if this
        // identifier is a reference or if it is part of the module definition.
        // Otherwise we know it's a reference.
        let is_reference = if module.krate().eq(root_crate)
            && matches!(def_source.value, ModuleSource::Module(_))
        {
            if let ModuleSource::Module(ast_module) = def_source.value {
                let name = ast_module.name().unwrap();
                let def_range = InFile::new(def_source.file_id.clone(), name.syntax())
                    .original_file_range_opt(db)
                    .map(|it| it.range);
                if let Some(def_range) = def_range {
                    // If the range matches, this identifier is part of the definition and is
                    // therefore not a reference. Module definitions are emitted in `emit_modules`
                    // so we return None.
                    if range.eq(&def_range) { false } else { true }
                } else {
                    // TODO: Probably print something about not being able to find the range?
                    false
                }
            } else {
                // This is guaranteed to never happen since we already check if the value
                // matches but the compiler complains if we don't use an `if let`.
                false
            }
        } else {
            true
        };

        if is_reference {
            let start = u32::from(range.start());
            let end = u32::from(range.end());

            let mut target_vname = self.gen_base_vname();
            target_vname.set_signature(self.get_signature(db, Definition::Module(module.clone())));

            let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
            anchor_vname.set_language("rust".to_string());
            anchor_vname.set_signature(format!("anchor_{start}_to_{end}"));

            self.emitter.emit_reference(&anchor_vname, &target_vname, start, end)?;
        }

        Ok(())
    }

    /// Visits an identifier for a static. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_static_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        statik: &Static,
    ) -> Result<(), KytheError> {
        let static_name = statik.name(db).to_smol_str();

        // Get the definition range
        let source = semantics.source(statik.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the parent module
        let mut module_vname = self.gen_base_vname();
        let module_signature = self.get_signature(db, Definition::Module(statik.module(db)));
        module_vname.set_signature(module_signature.clone());

        // Create the VName for the static
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(format!(
            "{module_signature}::STATIC({static_name}|{}-{})",
            u32::from(definition_range.start()),
            u32::from(definition_range.end())
        ));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"variable".to_vec())?;
            if statik.value(db).is_some() {
                self.emitter.emit_fact(&def_vname, "/kythe/complete", b"definition".to_vec())?;
            } else {
                self.emitter.emit_fact(&def_vname, "/kythe/complete", b"incomplete".to_vec())?;
            }
            self.emitter.emit_edge(&def_vname, &module_vname, "/kythe/edge/childof")?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a type alias. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_talias_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        talias: &TypeAlias,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = semantics.source(talias.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the static
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::TypeAlias(talias.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"talias".to_vec())?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;

            // Try to generate a VName signature for the type alias' parent node
            let parent_signature = if let Some(assoc_item) = talias.as_assoc_item(db) {
                match assoc_item.container(db) {
                    AssocItemContainer::Trait(t) => {
                        Some(self.get_signature(db, Definition::Trait(t)))
                    }
                    AssocItemContainer::Impl(i) => {
                        let self_type = i.self_ty(db);
                        if let Some(adt) = self_type.as_adt() {
                            Some(self.get_signature(db, Definition::Adt(adt)))
                        } else {
                            // TODO: Should probably emit a diagnostic node here
                            None
                        }
                    }
                }
            } else {
                Some(self.get_signature(db, Definition::Module(talias.module(db))))
            };
            // If we successfully generated a signature for the parent, emit a childof edge
            // to it
            if let Some(parent_signature) = parent_signature {
                let mut parent_vname = self.gen_base_vname();
                parent_vname.set_signature(parent_signature);
                self.emitter.emit_edge(&def_vname, &parent_vname, "/kythe/edge/childof")?;
            }
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for a trait. If this is the definition, emits
    /// node information and an anchor. If this is a reference, emits a
    /// reference.
    fn visit_trait_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        trate: &Trait,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = semantics.source(trate.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the parent module
        let mut module_vname = self.gen_base_vname();
        module_vname.set_signature(self.get_signature(db, Definition::Module(trate.module(db))));

        // Create the VName for the trait
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Trait(trate.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            self.emitter.emit_fact(&def_vname, "/kythe/node/kind", b"interface".to_vec())?;
            self.emitter.emit_edge(&def_vname, &module_vname, "/kythe/edge/childof")?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Visits an identifier for an enum variant. If this is the definition,
    /// emits node information and an anchor. If this is a reference, emits
    /// a reference.
    fn visit_variant_ident(
        &mut self,
        db: &RootDatabase,
        semantics: &Semantics<'_, RootDatabase>,
        file_id: u32,
        token: &SyntaxToken,
        variant: &Variant,
    ) -> Result<(), KytheError> {
        // Get the definition range
        let source = semantics.source(variant.clone()).unwrap();
        let source_file_id = source.file_id.original_file(db);
        let name_node = source.value.syntax().children().find(|it| it.kind() == SyntaxKind::NAME);
        if name_node.is_none() {
            // TODO: Emit some diagnostic about not being about to find the name
            return Ok(());
        }
        let definition_range = name_node.unwrap().text_range();

        // Create the VName for the parent enum
        let mut enum_vname = self.gen_base_vname();
        let enum_signature =
            self.get_signature(db, Definition::Adt(Adt::Enum(variant.parent_enum(db))));
        enum_vname.set_signature(enum_signature.clone());

        // Create the VName for the const
        let mut def_vname = self.gen_base_vname();
        def_vname.set_signature(self.get_signature(db, Definition::Variant(variant.clone())));

        // Create the VName for the anchor
        let token_range = token.text_range();
        let token_range_start = u32::from(token_range.start());
        let token_range_end = u32::from(token_range.end());
        let mut anchor_vname = self.file_id_to_vname.get(&file_id).unwrap().clone();
        anchor_vname.set_language("rust".to_string());
        anchor_vname.set_signature(format!("anchor_{token_range_start}_to_{token_range_end}"));

        if file_id.eq(&source_file_id.0) && token_range.eq(&definition_range) {
            // This is a definition, so emit the corresponding nodes
            let mut facts: Vec<(&str, &[u8])> = Vec::new();
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
            for (fact_name, fact_value) in facts.iter() {
                self.emitter.emit_fact(&def_vname, fact_name, fact_value.to_vec())?;
            }
            self.emitter.emit_edge(&def_vname, &enum_vname, "/kythe/edge/childof")?;
            self.emitter.emit_anchor(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        } else {
            // Emit a reference
            self.emitter.emit_reference(
                &anchor_vname,
                &def_vname,
                token_range_start,
                token_range_end,
            )?;
        }

        Ok(())
    }

    /// Get a VName signature for a Definition from the cache or by generating
    /// it
    fn get_signature(&mut self, db: &RootDatabase, def: Definition) -> String {
        // Check if we already know what the signature is
        if let Some(signature) = self.def_to_signature.get(&def) {
            return signature.to_owned();
        }

        let signature = match def {
            Definition::Adt(adt) => {
                let name = adt.name(db).to_smol_str();
                let module_signature = self.get_signature(db, Definition::Module(adt.module(db)));
                match adt {
                    Adt::Enum(_) => format!("{module_signature}::ENUM({name}"),
                    Adt::Struct(_) => format!("{module_signature}::STRUCT({name})"),
                    Adt::Union(_) => format!("{module_signature}::UNION({name})"),
                }
            }
            Definition::Const(const_) => {
                let name = if let Some(name) = const_.name(db) {
                    name.to_string()
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
                };
                format!("{parent_signature}::CONST({name})")
            }
            Definition::Function(function) => {
                let name = function.name(db).to_smol_str();
                let parent_signature = if let Some(assoc_item) = function.as_assoc_item(db) {
                    self.get_assoc_item_parent_signature(db, assoc_item.container(db))
                } else {
                    self.get_signature(db, Definition::Module(function.module(db)))
                };
                format!("{parent_signature}::FUNCTION({name})")
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
                };
                format!("{parent_signature}::LABEL({name}|{start}-{end})")
            }
            Definition::Local(local) => {
                let name = local.name(db).to_smol_str();
                let source = local.source(db).value;
                let range = if source.is_left() {
                    source.unwrap_left().syntax().text_range()
                } else {
                    source.unwrap_right().syntax().text_range()
                };
                let start = u32::from(range.start());
                let end = u32::from(range.end());
                let parent_signature = match local.parent(db) {
                    DefWithBody::Function(f) => self.get_signature(db, Definition::Function(f)),
                    DefWithBody::Static(s) => self.get_signature(db, Definition::Static(s)),
                    DefWithBody::Const(c) => self.get_signature(db, Definition::Const(c)),
                    DefWithBody::Variant(v) => self.get_signature(db, Definition::Variant(v)),
                };
                format!("{parent_signature}::LABEL({name}|{start}-{end})")
            }
            Definition::Macro(makro) => {
                let name = makro.name(db).to_smol_str();
                let module_signature = self.get_signature(db, Definition::Module(makro.module(db)));
                format!("{module_signature}::MACRO({name})")
            }
            Definition::Module(module) => {
                if module.is_crate_root(db) {
                    let def_source = module.definition_source(db);
                    let file_id = def_source.file_id.original_file(db);
                    self.file_id_to_path.get(&file_id.0).unwrap().to_owned()
                } else {
                    let parent = module.parent(db).unwrap();
                    let parent_signature =
                        self.get_signature(db, Definition::Module(parent.clone()));
                    let name = module.name(db).unwrap().to_smol_str();
                    format!("{parent_signature}::{name}")
                }
            }
            Definition::Trait(trate) => {
                let name = trate.name(db).to_smol_str();
                let module_signature = self.get_signature(db, Definition::Module(trate.module(db)));
                format!("{module_signature}::TRAIT({name})")
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
                };
                format!("{parent_signature}::TALIAS({name}|{start}-{end})")
            }
            Definition::Variant(variant) => {
                let name = variant.name(db).to_smol_str();
                let enum_signature =
                    self.get_signature(db, Definition::Adt(Adt::Enum(variant.parent_enum(db))));
                format!("{enum_signature}::VARIANT({name})")
            }
            _ => "".to_string(),
        };
        if !signature.is_empty() {
            self.def_to_signature.insert(def.clone(), signature.clone());
        }
        signature
    }

    fn get_assoc_item_parent_signature(
        &mut self,
        db: &RootDatabase,
        aic: AssocItemContainer,
    ) -> String {
        match aic {
            AssocItemContainer::Trait(t) => self.get_signature(db, Definition::Trait(t)),
            AssocItemContainer::Impl(i) => {
                let module_signature = self.get_signature(db, Definition::Module(i.module(db)));
                let impl_range = i.source(db).unwrap().value.syntax().text_range();
                let impl_start = u32::from(impl_range.start());
                let impl_end = u32::from(impl_range.end());
                format!("{module_signature}::IMPL({impl_start}-{impl_end}")
            }
        }
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
        Crate::all(db).into_iter().map(|krate| krate.root_module(db)).collect();
    for module in root_modules {
        let def_source = module.definition_source(db);
        let file_id = def_source.file_id.original_file(db);
        if file_ids.contains(&file_id.0) {
            return Some(module);
        }
    }
    None
}

/// Attempt to get a token's definition from the semantic database
fn get_definition(sema: &Semantics<'_, RootDatabase>, token: SyntaxToken) -> Option<Definition> {
    for token in sema.descend_into_macros(token) {
        let def = IdentClass::classify_token(sema, &token).map(IdentClass::definitions_no_ops);
        if let Some(&[x]) = def.as_deref() {
            return Some(x);
        }
    }
    None
}
