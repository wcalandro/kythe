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

use std::fmt::Write;
use storage_rust_proto::VName;

/// Converts a VName to a Kythe URI
pub fn vname_to_kythe_uri(vname: &VName) -> String {
    let mut uri = String::from("kythe:");
    if !vname.get_corpus().is_empty() {
        let escaped = escape_string(vname.get_corpus(), false);
        write!(uri, "//{escaped}").unwrap();
    }
    if !vname.get_language().is_empty() {
        let escaped = escape_string(vname.get_language(), true);
        write!(uri, "?lang={escaped}").unwrap();
    }
    if !vname.get_path().is_empty() {
        let escaped = escape_string(vname.get_path(), false);
        write!(uri, "?path={escaped}").unwrap();
    }
    if !vname.get_root().is_empty() {
        let escaped = escape_string(vname.get_root(), false);
        write!(uri, "?root={escaped}").unwrap();
    }
    if !vname.get_signature().is_empty() {
        let escaped = escape_string(vname.get_signature(), true);
        write!(uri, "#{escaped}").unwrap();
    }
    uri
}

fn escape_string(input: &str, escape_slash: bool) -> String {
    let mut escaped = String::new();
    for c in input.chars() {
        if should_escape(c, escape_slash) {
            write!(escaped, "%{:X}", c as i8).unwrap();
        } else {
            write!(escaped, "{c}").unwrap();
        }
    }
    escaped
}

/// Determine's whether a character should be escaped using Kythe's specific
/// escaping rules
fn should_escape(c: char, escape_slash: bool) -> bool {
    if c.is_alphanumeric() {
        return false;
    }
    match c {
        '-' | '.' | '_' | '~' => false,
        '/' => escape_slash,
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vname_to_kythe_uri() {
        let mut vname = VName::new();
        vname.set_corpus("google3".into());
        vname.set_language("rust".into());
        vname.set_signature(
            "third_party/rust/sha2/v0_10/src/lib.rs::core_api::STRUCT(Sha256VarCore)".into(),
        );
        assert_eq!(
            vname_to_kythe_uri(&vname),
            "kythe://google3?lang=rust#third_party%2Frust%2Fsha2%2Fv0_10%2Fsrc%2Flib.rs%3A%3Acore_api%3A%3ASTRUCT%28Sha256VarCore%29"
        );
    }
}
