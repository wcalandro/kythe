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

// This module is essentially a recreation/copy of sections of rust-analyzer
// code that aren't exposed in the library. The following sources were used:
// https://github.com/rust-lang/rust-analyzer/blob/05b061205179dab9a5cd94ae66d1c0e9b8febe08/crates/ide/src/doc_links.rs
// https://github.com/rust-lang/rust-analyzer/blob/05b061205179dab9a5cd94ae66d1c0e9b8febe08/crates/ide/src/doc_links/intra_doc_links.rs
// If this functionality breaks, those source files should be referenced to make
// any updates. We should consider asking rust-analyzer to make some of that
// code public in the library.

use pulldown_cmark::{BrokenLink, CowStr, Event, Options, Parser, Tag};
use ra_ap_hir::{Documentation, HasAttrs, Namespace};
use ra_ap_hir_def::attr::DocsRangeMap;
use ra_ap_ide::RootDatabase;
use ra_ap_ide_db::defs::Definition;
use ra_ap_syntax::TextRange;
use regex::Regex;

pub struct DocReference {
    pub range: Option<TextRange>,
    pub reference: Definition,
}

pub fn process_documentation(
    def: &Definition,
    docs: &Documentation,
    range_map: &DocsRangeMap,
    db: &RootDatabase,
) -> (String, Vec<DocReference>) {
    let link_defs: Vec<(TextRange, Option<Definition>)> = extract_links_from_docs(docs)
        .into_iter()
        .map(|(range, link, ns)| (range, resolve_doc_path_for_def(db, *def, &link, ns)))
        .collect();

    // If there are no doc links, just sanitize the doc and return
    if link_defs.is_empty() {
        let sanitized_doc =
            docs.as_str().to_string().replace('\\', "\\\\").replace('[', "\\[").replace(']', "\\]");
        return (sanitized_doc, Vec::new());
    }

    let doc_string = docs.as_str().to_string();
    let doc_chars: Vec<char> = doc_string.chars().collect();
    let mut doc_cursor: usize = 0;
    let mut refs = Vec::new();
    let mut new_doc = Vec::new();
    for (range, link_def) in link_defs.into_iter() {
        while doc_cursor < range.start().into() && doc_cursor < doc_string.len() {
            let c = doc_chars[doc_cursor];
            if c == '\\' || c == '[' || c == ']' {
                new_doc.push('\\')
            }
            new_doc.push(c);
            doc_cursor += 1;
        }

        let re = Regex::new(r"\[(?P<title>[^\]]+)\]").unwrap();
        let title = if let Some(captures) =
            re.captures(&doc_string[range.start().into()..range.end().into()])
        {
            captures["title"].to_string()
        } else {
            String::from("unknown")
        };
        if let Some(link_def) = link_def {
            new_doc.extend(format!("[{title}]").chars());
            refs.push(DocReference {
                range: range_map.map(range).map(|r| r.value),
                reference: link_def,
            });
        } else {
            new_doc.extend(title.chars());
        }
        doc_cursor = range.end().into();
    }
    while doc_cursor < doc_string.len() {
        let c = doc_chars[doc_cursor];
        if c == '\\' || c == '[' || c == ']' {
            new_doc.push('\\')
        }
        new_doc.push(c);
        doc_cursor += 1;
    }
    (String::from_iter(new_doc), refs)
}

const MARKDOWN_OPTIONS: Options =
    Options::ENABLE_FOOTNOTES.union(Options::ENABLE_TABLES).union(Options::ENABLE_TASKLISTS);

/// Extracts all links from a given markdown text returning the definition text
/// range, link-text and the namespace if known.
fn extract_links_from_docs(docs: &Documentation) -> Vec<(TextRange, String, Option<Namespace>)> {
    Parser::new_with_broken_link_callback(
        docs.as_str(),
        MARKDOWN_OPTIONS,
        Some(&mut broken_link_clone_cb),
    )
    .into_offset_iter()
    .filter_map(|(event, range)| match event {
        Event::Start(Tag::Link(_, target, _)) => {
            let (link, ns) = parse_intra_doc_link(&target);
            Some((
                TextRange::new(range.start.try_into().ok()?, range.end.try_into().ok()?),
                link.to_string(),
                ns,
            ))
        }
        _ => None,
    })
    .collect()
}

fn resolve_doc_path_for_def(
    db: &RootDatabase,
    def: Definition,
    link: &str,
    ns: Option<Namespace>,
) -> Option<Definition> {
    match def {
        Definition::Module(it) => it.resolve_doc_path(db, link, ns),
        Definition::Function(it) => it.resolve_doc_path(db, link, ns),
        Definition::Adt(it) => it.resolve_doc_path(db, link, ns),
        Definition::Variant(it) => it.resolve_doc_path(db, link, ns),
        Definition::Const(it) => it.resolve_doc_path(db, link, ns),
        Definition::Static(it) => it.resolve_doc_path(db, link, ns),
        Definition::Trait(it) => it.resolve_doc_path(db, link, ns),
        Definition::TraitAlias(it) => it.resolve_doc_path(db, link, ns),
        Definition::TypeAlias(it) => it.resolve_doc_path(db, link, ns),
        Definition::Macro(it) => it.resolve_doc_path(db, link, ns),
        Definition::Field(it) => it.resolve_doc_path(db, link, ns),
        Definition::SelfType(it) => it.resolve_doc_path(db, link, ns),
        Definition::BuiltinAttr(_)
        | Definition::ToolModule(_)
        | Definition::BuiltinType(_)
        | Definition::Local(_)
        | Definition::GenericParam(_)
        | Definition::Label(_)
        | Definition::DeriveHelper(_) => None,
    }
    .map(Definition::from)
}

fn broken_link_clone_cb(link: BrokenLink<'_>) -> Option<(CowStr<'_>, CowStr<'_>)> {
    Some((/* url */ link.reference.clone(), /* title */ link.reference))
}

const TYPES: ([&str; 9], [&str; 0]) =
    (["type", "struct", "enum", "mod", "trait", "union", "module", "prim", "primitive"], []);
const VALUES: ([&str; 8], [&str; 1]) =
    (["value", "function", "fn", "method", "const", "static", "mod", "module"], ["()"]);
const MACROS: ([&str; 2], [&str; 1]) = (["macro", "derive"], ["!"]);

/// Extract the specified namespace from an intra-doc-link if one exists.
///
/// # Examples
///
/// * `struct MyStruct` -> ("MyStruct", `Namespace::Types`)
/// * `panic!` -> ("panic", `Namespace::Macros`)
/// * `fn@from_intra_spec` -> ("from_intra_spec", `Namespace::Values`)
fn parse_intra_doc_link(s: &str) -> (&str, Option<Namespace>) {
    let s = s.trim_matches('`');

    [
        (Namespace::Types, (TYPES.0.iter(), TYPES.1.iter())),
        (Namespace::Values, (VALUES.0.iter(), VALUES.1.iter())),
        (Namespace::Macros, (MACROS.0.iter(), MACROS.1.iter())),
    ]
    .into_iter()
    .find_map(|(ns, (mut prefixes, mut suffixes))| {
        if let Some(prefix) = prefixes.find(|&&prefix| {
            s.starts_with(prefix)
                && s.chars().nth(prefix.len()).map_or(false, |c| c == '@' || c == ' ')
        }) {
            Some((&s[prefix.len() + 1..], ns))
        } else {
            suffixes.find_map(|&suffix| s.strip_suffix(suffix).zip(Some(ns)))
        }
    })
    .map_or((s, None), |(s, ns)| (s, Some(ns)))
}
