// Copyright 2021 The Kythe Authors. All rights reserved.
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

use analysis_rust_proto::VName;
use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// The String forms of the vname field patterns
#[derive(Deserialize)]
pub struct RawRulePatterns {
    pub corpus: Option<String>,
    pub root: Option<String>,
    pub path: Option<String>,
}

/// The String form of a rule
#[derive(Deserialize)]
pub struct RawRule {
    pub pattern: String,
    pub vname: RawRulePatterns,
}

impl RawRule {
    /// Process a RawRule into a VNameRule
    pub fn process(&self) -> VNameRule {
        // Enclose the raw pattern in an explicitly-anchored non-capturing group
        // to mirror the expected RE2::FullMatch semantics.
        let r = Regex::new(format!(r"\A(?:{})\z", &self.pattern).as_ref()).unwrap();

        let corpus_pattern = Self::convert_vname_pattern(&self.vname.corpus);
        let root_pattern = Self::convert_vname_pattern(&self.vname.root);
        let path_pattern = Self::convert_vname_pattern(&self.vname.path);

        VNameRule { pattern: r, corpus_pattern, root_pattern, path_pattern }
    }

    // Convert the rule to regex-compatible syntax
    fn convert_vname_pattern(pattern: &Option<String>) -> Option<String> {
        let index_regex = Regex::new(r"@(?P<index>\d+)@").unwrap();
        pattern.as_ref().map(|pattern| index_regex.replace_all(pattern, "$${$index}").into_owned())
    }
}

/// A processed form of a RawRule
pub struct VNameRule {
    pub pattern: Regex,
    pub corpus_pattern: Option<String>,
    pub root_pattern: Option<String>,
    pub path_pattern: Option<String>,
}

impl VNameRule {
    /// Processed vname rules defined in files
    pub fn parse_vname_rules(config_path: &PathBuf) -> Result<Vec<Self>> {
        let mut processed_rules = Vec::new();
        let file = File::open(config_path)
            .with_context(|| format!("Failed to open file: {}", config_path.to_string_lossy()))?;
        let reader = BufReader::new(file);
        let rules: Vec<RawRule> = serde_json::from_reader(reader)
            .with_context(|| format!("Failed to parse file: {}", config_path.to_string_lossy()))?;

        for rule in rules {
            processed_rules.push(rule.process());
        }

        Ok(processed_rules)
    }

    /// Check if the path matches the rule's pattern
    pub fn matches(&self, path: &str) -> bool {
        self.pattern.is_match(path)
    }

    /// Produces a vname from the path based on the rule
    pub fn produce_vname(&mut self, path: &str, default_corpus: &str) -> VName {
        let mut vname = VName::new();
        if let Some(corpus_pattern) = &self.corpus_pattern {
            vname.set_corpus(self.pattern.replace(path, corpus_pattern).to_string());
        } else {
            // If the rule doesn't define a corpus, just use the default (set by
            // KYTHE_CORPUS)
            vname.set_corpus(default_corpus.to_string());
        }
        if let Some(root_pattern) = &self.root_pattern {
            vname.set_root(self.pattern.replace(path, root_pattern).to_string());
        }
        if let Some(path_pattern) = &self.path_pattern {
            vname.set_path(self.pattern.replace(path, path_pattern).to_string());
        }
        vname
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_raw_rule_to_vname_rule() {
        let raw_rule = RawRule {
            pattern: r"(Test\d+)+".to_string(),
            vname: RawRulePatterns {
                corpus: Some("corpus".to_string()),
                root: Some("@1@@2@".to_string()),
                path: None,
            },
        };
        let vname_rule = raw_rule.process();
        assert_eq!(vname_rule.corpus_pattern, Some("corpus".to_string()));
        assert_eq!(vname_rule.root_pattern, Some("${1}${2}".to_string()));
        assert_eq!(vname_rule.path_pattern, None);
    }

    #[test]
    fn test_vname_translation() {
        let mut vname_rule_1 = VNameRule {
            pattern: regex::Regex::new("external/([^/]+)/(.*\\.rs)$")
                .expect("Couldn't parse test regex"),
            corpus_pattern: Some("rust_extractor".to_string()),
            root_pattern: Some("${1}".to_string()),
            path_pattern: Some("${2}".to_string()),
        };
        let path = "external/rust_test/vname_rules/lib.rs";
        assert!(vname_rule_1.matches(path));

        let vname_1 = vname_rule_1.produce_vname(path, "");
        assert_eq!(vname_1.get_corpus(), "rust_extractor".to_string());
        assert_eq!(vname_1.get_root(), "rust_test".to_string());
        assert_eq!(vname_1.get_path(), "vname_rules/lib.rs".to_string());

        let mut vname_rule_2 = VNameRule {
            pattern: regex::Regex::new("external/([^/]+)/(.*\\.rs)$")
                .expect("Couldn't parse test regex"),
            corpus_pattern: None,
            root_pattern: Some("${1}".to_string()),
            path_pattern: Some("${2}".to_string()),
        };
        let path = "external/rust_test/vname_rules/lib.rs";
        assert!(vname_rule_2.matches(path));

        let vname_2 = vname_rule_2.produce_vname(path, "default");
        assert_eq!(vname_2.get_corpus(), "default".to_string());
        assert_eq!(vname_2.get_root(), "rust_test".to_string());
        assert_eq!(vname_2.get_path(), "vname_rules/lib.rs".to_string());
    }

    #[test]
    fn test_patterns_are_anchored() {
        let rule = RawRule {
            pattern: "/absolute/path".to_string(),
            vname: RawRulePatterns { corpus: None, root: None, path: None },
        }
        .process();
        assert!(rule.matches("/absolute/path"));
        assert!(!rule.matches("sub/absolute/path"));
    }
}
