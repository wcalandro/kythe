load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_clippy", "rust_library", "rust_test")
load("//tools/build_rules/verifier_test:rust_indexer_test.bzl", "rust_indexer_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "kythe_rust_indexer",
    srcs = glob(
        include = ["src/**/*.rs"],
        exclude = ["src/bin/**"],
    ),
    edition = "2021",
    deps = [
        "//kythe/proto:analysis_rust_proto",
        "//kythe/proto:common_rust_proto",
        "//kythe/proto:storage_rust_proto",
        "@crate_index//:base64",
        "@crate_index//:glob",
        "@crate_index//:hex",
        "@crate_index//:path-absolutize",
        "@crate_index//:protobuf",
        "@crate_index//:pulldown-cmark",
        "@crate_index//:quick-error",
        "@crate_index//:ra_ap_hir",
        "@crate_index//:ra_ap_hir_def",
        "@crate_index//:ra_ap_ide",
        "@crate_index//:ra_ap_ide_db",
        "@crate_index//:ra_ap_load-cargo",
        "@crate_index//:ra_ap_paths",
        "@crate_index//:ra_ap_project_model",
        "@crate_index//:ra_ap_syntax",
        "@crate_index//:ra_ap_vfs",
        "@crate_index//:regex",
        "@crate_index//:rustc-hash",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
        "@crate_index//:sha2",
        "@crate_index//:triomphe",
        "@crate_index//:zip",
    ],
)

rust_test(
    name = "kythe_rust_indexer_tests",
    crate = ":kythe_rust_indexer",
)

rust_test(
    name = "integration_tests",
    srcs = glob(["tests/*.rs"]),
    data = [
        "tests/testkzip.kzip",
    ],
    edition = "2021",
    deps = [
        ":kythe_rust_indexer",
        "//kythe/proto:analysis_rust_proto",
        "//kythe/proto:storage_rust_proto",
        "@crate_index//:hex",
        "@crate_index//:protobuf",
        "@crate_index//:sha2",
        "@rules_rust//tools/runfiles",
    ],
)

rust_binary(
    name = "bazel_indexer",
    srcs = glob(
        include = ["src/bin/bazel/*.rs"],
    ),
    crate_root = ":src/bin/bazel/main.rs",
    edition = "2021",
    deps = [
        ":kythe_rust_indexer",
        "@crate_index//:anyhow",
        "@crate_index//:clap",
    ],
)

rust_binary(
    name = "proxy_indexer",
    srcs = glob(
        include = ["src/bin/proxy/*.rs"],
    ),
    crate_root = ":src/bin/proxy/main.rs",
    edition = "2021",
    deps = [
        ":kythe_rust_indexer",
        "//kythe/proto:analysis_rust_proto",
        "@crate_index//:anyhow",
        "@crate_index//:base64",
        "@crate_index//:clap",
        "@crate_index//:protobuf",
        "@crate_index//:serde_json",
    ],
)

rust_clippy(
    name = "clippy",
    deps = [
        ":bazel_indexer",
        ":kythe_rust_indexer",
        ":proxy_indexer",
    ],
)

rust_indexer_test(
    name = "adt_test",
    srcs = ["testdata/adt.rs"],
)

rust_indexer_test(
    name = "const_test",
    srcs = ["testdata/const.rs"],
)

rust_indexer_test(
    name = "documentation_test",
    srcs = ["testdata/docs.rs"],
)

rust_indexer_test(
    name = "doc_links_test",
    srcs = ["testdata/doc_links.rs"],
)

rust_indexer_test(
    name = "field_test",
    srcs = ["testdata/fields.rs"],
)

rust_indexer_test(
    name = "function_test",
    srcs = ["testdata/functions.rs"],
)

rust_indexer_test(
    name = "macro_test",
    srcs = ["testdata/macros.rs"],
)

rust_indexer_test(
    name = "label_test",
    srcs = ["testdata/labels.rs"],
)

rust_indexer_test(
    name = "local_test",
    srcs = ["testdata/locals.rs"],
)

rust_indexer_test(
    name = "markedsource_test",
    srcs = ["testdata/markedsource.rs"],
    has_marked_source = True,
    include_sysroot = True,
    resolve_code_facts = True,
)

rust_indexer_test(
    name = "modules_test",
    srcs = glob(["testdata/modules/*"]),
)

rust_indexer_test(
    name = "static_test",
    srcs = ["testdata/static.rs"],
)

rust_indexer_test(
    name = "talias_test",
    srcs = ["testdata/talias.rs"],
)

rust_indexer_test(
    name = "trait_test",
    srcs = ["testdata/traits.rs"],
)

rust_indexer_test(
    name = "variant_test",
    srcs = ["testdata/variant.rs"],
)
