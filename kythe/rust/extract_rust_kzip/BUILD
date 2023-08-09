load("//tools:build_rules/shims.bzl", "go_binary")

package(default_visibility = ["//kythe:default_visibility"])

go_binary(
    name = "extract_rust_kzip",
    srcs = ["extract_rust_kzip.go"],
    deps = [
        "//kythe/go/extractors/bazel",
        "//kythe/go/util/vnameutil",
        "//kythe/proto:analysis_go_proto",
        "//kythe/proto:storage_go_proto",
    ],
)
