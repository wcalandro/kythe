# Copyright 2023 The Kythe Authors. All rights reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Implements a rule for testing the Rust indexer

Extracts and indexes the source files for the test, then runs the entries
through the verifier
"""

load("@bazel_skylib//lib:paths.bzl", "paths")
load(
    "//tools/build_rules/verifier_test:verifier_test.bzl",
    "KytheEntries",
    "verifier_test",
)

def _rust_extract_impl(ctx):
    # Copy out_dir_files to out_dir
    all_out_dir_files = []
    for f in ctx.files.out_dir_files:
        out = ctx.actions.declare_file("%s_out_dir/%s" % (ctx.label.name, f.basename))
        all_out_dir_files.append(out)
        ctx.actions.run_shell(
            outputs = [out],
            inputs = depset([f]),
            arguments = [f.path, out.path],
            command = "cp $1 $2",
        )

    # Determine the crate root
    crate_root_path = ""
    if len(ctx.files.srcs) == 1:
        crate_root_path = ctx.files.srcs[0].path
    else:
        for src in ctx.files.srcs:
            if paths.basename(src.path) == "main.rs":
                crate_root_path = src.path
                break
        if crate_root_path == "":
            fail("Could not determine root path for crate")

    # Generate extraction info file to be used by the extractor
    crate = dict()
    crate["name"] = ctx.attr.crate_name
    crate["root"] = crate_root_path
    crate["edition"] = "2021"
    crate["target"] = ctx.toolchains[Label("@rules_rust//rust:toolchain")].target_triple.str
    crate["crate_type"] = "bin"
    crate["deps"] = []
    crate["cfg"] = ["test", "debug_assertions"]
    crate["relevant_srcs"] = [src.path for src in ctx.files.srcs]

    if len(all_out_dir_files) > 0:
        crate["out_dir_path"] = all_out_dir_files[0].dirname
    else:
        crate["out_dir_path"] = ""

    extraction_info_file = ctx.actions.declare_file(ctx.label.name + ".rust_extraction_info.json")
    ctx.actions.write(
        output = extraction_info_file,
        content = json.encode(crate),
    )

    # Generate the kzip
    runfiles = ctx.files.srcs + all_out_dir_files + [extraction_info_file, ctx.file._vnames_config_file]
    output = ctx.outputs.kzip
    ctx.actions.run(
        mnemonic = "RustExtract",
        executable = ctx.executable._extractor,
        arguments = [
            "-corpus=test_corpus",
            "-extraction_info=%s" % extraction_info_file.path,
            "-output=%s" % output.path,
            "-vnames_config=%s" % ctx.file._vnames_config_file.path,
        ],
        inputs = runfiles,
        outputs = [output],
    )

    # buildifier: disable=rule-impl-return
    return struct(kzip = output)

# Generate a kzip with the compilations captured from a single Go library or
# binary rule.
rust_extract = rule(
    _rust_extract_impl,
    attrs = {
        "srcs": attr.label_list(
            mandatory = True,
            allow_files = [".rs"],
        ),
        "out_dir_files": attr.label_list(
            mandatory = True,
            allow_files = [".rs"],
        ),
        "crate_name": attr.string(
            default = "test_crate",
        ),
        "_extractor": attr.label(
            default = Label("//kythe/rust/extract_rust_kzip"),
            allow_files = True,
            executable = True,
            cfg = "exec",
        ),
        "_vnames_config_file": attr.label(
            default = Label("//external:vnames_config"),
            allow_single_file = True,
        ),
    },
    outputs = {"kzip": "%{name}.kzip"},
    toolchains = [str(Label("@rules_rust//rust:toolchain"))],
)

def _rust_entries_impl(ctx):
    kzip = ctx.attr.kzip.kzip
    indexer = ctx.executable._indexer
    iargs = [indexer.path]
    output = ctx.outputs.entries
    inputs = [kzip]

    if ctx.attr.include_sysroot:
        rust_analyzer_toolchain = ctx.toolchains[Label("@rules_rust//rust/rust_analyzer:toolchain_type")]
        sysroot = rust_analyzer_toolchain.rustc_srcs.label.workspace_root
        sysroot_src = sysroot + "/" + rust_analyzer_toolchain.rustc_srcs.label.package + "/library"
        iargs += ["--sysroot={}".format(sysroot), "--sysroot_src={}".format(sysroot_src)]
        inputs += rust_analyzer_toolchain.rustc_srcs.files.to_list()

    iargs += [kzip.path, "| gzip >" + output.path]

    cmds = ["set -e", "set -o pipefail", " ".join(iargs), ""]
    ctx.actions.run_shell(
        mnemonic = "RustIndexer",
        command = "\n".join(cmds),
        outputs = [output],
        inputs = inputs,
        tools = [indexer],
    )
    return [KytheEntries(compressed = depset([output]), files = depset())]

# Run the Kythe indexer on the output that results from a go_extract rule.
rust_entries = rule(
    _rust_entries_impl,
    attrs = {
        # The kzip to pass to the Rust indexer
        "kzip": attr.label(
            providers = ["kzip"],
            mandatory = True,
        ),
        # Whether to pass the Rust sysroot to the indexer
        "include_sysroot": attr.bool(
            default = False,
        ),
        # The location of the Rust indexer binary.
        "_indexer": attr.label(
            default = Label("//kythe/rust/indexer:bazel_indexer"),
            executable = True,
            cfg = "exec",
        ),
    },
    outputs = {"entries": "%{name}.entries.gz"},
    toolchains = [str(Label("@rules_rust//rust/rust_analyzer:toolchain_type"))],
)

def _rust_indexer(
        name,
        srcs,
        out_dir_files = [],
        include_sysroot = False):
    kzip = name + "_units"
    rust_extract(
        name = kzip,
        srcs = srcs,
        out_dir_files = out_dir_files,
    )
    entries = name + "_entries"
    rust_entries(
        name = entries,
        kzip = ":" + kzip,
        include_sysroot = include_sysroot,
    )
    return entries

# buildifier: disable=function-docstring-return
def rust_indexer_test(
        name,
        srcs,
        out_dir_files = [],
        size = None,
        tags = None,
        log_entries = False,
        has_marked_source = False,
        resolve_code_facts = False,
        allow_duplicates = False,
        include_sysroot = False):
    """
    Runs a Rust verifier test on the source files

    Args:
      name: Rule name
      srcs: A list of Rust source files to index and verify
      out_dir_files: A list of files to include in $OUT_DIR
      size: The size to pass to the verifier_test macro
      tags: The tags to pass to the verifier_test macro
      log_entries: Enable to make the verifier log all indexer entries
      has_marked_source: Enable if the test uses Marked Source
      resolve_code_facts: Enable to resolve Marked Source nodes
      allow_duplicates: Enable to make the verifier ignore duplicate entries
      include_sysroot: Whether to pass the sysroot and sysroot_src to the indexer
    """

    # Generate entries using the Rust indexer
    entries = _rust_indexer(
        name = name,
        srcs = srcs,
        out_dir_files = out_dir_files,
        include_sysroot = include_sysroot,
    )

    opts = ["--use_file_nodes", "--show_goals", "--check_for_singletons"]
    if log_entries:
        opts.append("--show_protos")
    if allow_duplicates:
        opts.append("--ignore_dups")
    if has_marked_source:
        opts.append("--convert_marked_source")
    return verifier_test(
        name = name,
        size = size,
        opts = opts,
        tags = tags,
        resolve_code_facts = resolve_code_facts,
        deps = [":" + entries],
    )
