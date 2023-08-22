workspace(name = "io_kythe")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("//:version.bzl", "MAX_VERSION", "MIN_VERSION", "check_version")

# Check that the user has a version between our minimum supported version of
# Bazel and our maximum supported version of Bazel.
check_version(MIN_VERSION, MAX_VERSION)

load("//:setup.bzl", "kythe_rule_repositories", "remote_java_repository")

kythe_rule_repositories()

###
# BEGIN rules_ts setup
# loads are sensitive to intervening calls, so they need to happen at the
# top-level and not in e.g. a _ts_dependencies() function.
load("@aspect_rules_ts//ts:repositories.bzl", "rules_ts_dependencies")

rules_ts_dependencies(
    ts_version_from = "//:package.json",
)

load("@aspect_rules_jasmine//jasmine:dependencies.bzl", "rules_jasmine_dependencies")

# Fetch dependencies which users need as well
rules_jasmine_dependencies()

# Fetch and register node, if you haven't already
load("@rules_nodejs//nodejs:repositories.bzl", "DEFAULT_NODE_VERSION", "nodejs_register_toolchains")

nodejs_register_toolchains(
    name = "node",
    node_version = DEFAULT_NODE_VERSION,
)

load("@aspect_rules_js//npm:npm_import.bzl", "npm_translate_lock")

npm_translate_lock(
    name = "npm",
    pnpm_lock = "//:pnpm-lock.yaml",
)

load("@npm//:repositories.bzl", "npm_repositories")

npm_repositories()

load("@aspect_rules_jasmine//jasmine:repositories.bzl", "jasmine_repositories")

jasmine_repositories(name = "jasmine")

load("@jasmine//:npm_repositories.bzl", jasmine_npm_repositories = "npm_repositories")

jasmine_npm_repositories()

# END rules_ts setup
###

# gazelle:repository_macro external.bzl%_go_dependencies
load("//:external.bzl", "kythe_dependencies")

kythe_dependencies()

load("//tools/build_rules/external_tools:external_tools_configure.bzl", "external_tools_configure")

external_tools_configure()

load("@maven//:compat.bzl", "compat_repositories")

compat_repositories()

load("@maven//:defs.bzl", "pinned_maven_install")

pinned_maven_install()

load(
    "@bazelruby_rules_ruby//ruby:defs.bzl",
    "ruby_bundle",
)

ruby_bundle(
    name = "website_bundle",
    bundler_version = "2.1.4",
    gemfile = "//kythe/web/site:Gemfile",
    gemfile_lock = "//kythe/web/site:Gemfile.lock",
)

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

# Run `CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index` after updating
rust_analyzer_version = "=0.0.165"

crates_repository(
    name = "crate_index",
    annotations = {
        "protobuf-codegen": [crate.annotation(gen_binaries = True)],
    },
    cargo_lockfile = "//:Cargo.Bazel.lock",
    lockfile = "//:cargo-bazel-lock.json",
    packages = {
        "anyhow": crate.spec(
            version = "=1.0.68",
        ),
        "base64": crate.spec(
            version = "=0.13.0",
        ),
        "clap": crate.spec(
            features = ["derive"],
            version = "=3.2.23",
        ),
        "glob": crate.spec(
            version = "0.3.0",
        ),
        "hex": crate.spec(
            version = "0.4.3",
        ),
        "pulldown-cmark": crate.spec(
            version = "=0.9.3",
        ),
        "quick-error": crate.spec(
            version = "2.0.1",
        ),
        "ra_ap_hir": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_hir_def": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_ide": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_ide_db": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_paths": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_project_model": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_syntax": crate.spec(
            version = rust_analyzer_version,
        ),
        "ra_ap_vfs": crate.spec(
            version = rust_analyzer_version,
        ),
        "regex": crate.spec(
            version = "1.5.6",
        ),
        "rustc-hash": crate.spec(
            version = "=1.1.0",
        ),
        "serde": crate.spec(
            features = ["derive"],
            version = "=1.0.171",
        ),
        "serde_json": crate.spec(
            version = "1.0",
        ),
        "triomphe": crate.spec(
            version = "=0.1.9",
        ),
        "sha2": crate.spec(
            version = "0.10.2",
        ),
        "zip": crate.spec(
            version = "0.5.11",
        ),
        # Dependencies for our Rust protobuf toolchain
        "protobuf": crate.spec(
            features = ["with-bytes"],
            version = "=2.28.0",
        ),
        "protobuf-codegen": crate.spec(
            version = "=2.28.0",
        ),
    },
    rust_version = "nightly/2023-03-16",
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# Register our Rust protobuf toolchain from the BUILD file
register_toolchains(
    ":rust_proto_toolchain",
)

# Bazel does not yet ship with JDK-19 compatible JDK, so configure our own.
remote_java_repository(
    name = "remotejdk19_linux",
    prefix = "remotejdk",
    sha256 = "2ac8cd9e7e1e30c8fba107164a2ded9fad698326899564af4b1254815adfaa8a",
    strip_prefix = "zulu19.30.11-ca-jdk19.0.1-linux_x64",
    target_compatible_with = [
        "@platforms//os:linux",
    ],
    urls = [
        "https://mirror.bazel.build/cdn.azul.com/zulu/bin/zulu19.30.11-ca-jdk19.0.1-linux_x64.tar.gz",
        "https://cdn.azul.com/zulu/bin/zulu19.30.11-ca-jdk19.0.1-linux_x64.tar.gz",
    ],
    version = "19",
)

register_toolchains("//buildenv/java:all")
