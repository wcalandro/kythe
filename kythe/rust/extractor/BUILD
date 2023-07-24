load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_clippy", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "extractor_lib",
    srcs = glob(
        include = ["src/**/*.rs"],
        exclude = ["src/bin/**"],
    ),
    edition = "2021",
    deps = [
        "//kythe/proto:analysis_rust_proto",
        "@crate_index//:anyhow",
        "@crate_index//:regex",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
    ],
)

rust_test(
    name = "extractor_lib_test",
    crate = ":extractor_lib",
)

rust_binary(
    name = "extractor",
    srcs = glob(["src/bin/**/*.rs"]),
    edition = "2021",
    deps = [
        ":extractor_lib",
        "//kythe/proto:analysis_rust_proto",
        "@crate_index//:anyhow",
        "@crate_index//:clap",
        "@crate_index//:glob",
        "@crate_index//:hex",
        "@crate_index//:protobuf",
        "@crate_index//:serde_json",
        "@crate_index//:sha2",
        "@crate_index//:zip",
    ],
)

# rust_clippy(
#     name = "clippy",
#     deps = [
#         ":extractor",
#         ":extractor_lib",
#     ],
# )

bzl_library(
    name = "aspect_bzl",
    srcs = ["aspect.bzl"],
)
