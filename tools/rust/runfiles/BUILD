load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "runfiles",
    srcs = ["lib.rs"],
    edition = "2021",
)

rust_test(
    name = "runfiles_test",
    crate = ":runfiles",
    data = [
        ":testfile.txt",
    ],
)
