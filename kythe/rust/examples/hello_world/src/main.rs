// Copyright 2020 The Kythe Authors. All rights reserved.
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

use colored::*;

static NUM: i32 = 1;

fn main() {
    hello_world();
    let mut x = 1;
    x = 2;
    let z = math::add(x, y);
}

/// This prints "Hello, World!" in blue and green
fn hello_world() {
    println!("{} {}", "Hello,".blue(), "World!".green());
}
