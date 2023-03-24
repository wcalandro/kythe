load("@rules_rust//rust:rust_common.bzl", "CrateInfo")

def _rust_extractor_aspect_impl(target, ctx):
    crate_info = target[CrateInfo]
    (crate, runfiles) = _process_basic_crate_info(ctx, crate_info)

    deps = []
    for dep in crate_info.deps.to_list():
        if dep.crate_info != None and dep.crate_info.root != crate_info.root:
            (dep_crate, dep_runfiles) = _process_basic_crate_info(ctx, dep.crate_info)
            deps.append(dep_crate)
            runfiles.extend(dep_runfiles)
    crate["deps"] = deps

    for action in target.actions:
        if action.mnemonic == "Rustc":
            crate["arguments"] = action.argv
            outputs_list = action.outputs.to_list()
            if len(outputs_list) > 0:
                crate["output"] = outputs_list[0].path

    # TODO: Handle aliases?
    # TODO: See if we need to get to populate proc_macro_dylib_path

    extraction_info_file = ctx.actions.declare_file(ctx.label.name + ".rust_extraction_info.json")
    ctx.actions.write(
        output = extraction_info_file,
        content = json.encode(crate),
    )

    kzip = ctx.actions.declare_file(ctx.label.name + ".rust.kzip")
    runfiles.append(extraction_info_file)
    runfiles.append(ctx.file._vnames_config)
    ctx.actions.run(
        inputs = runfiles,
        tools = [ctx.executable._rust_extractor],
        outputs = [kzip],
        mnemonic = "ExtractRustAspect",
        executable = ctx.executable._rust_extractor,
        arguments = [
            "--extraction_info=%s" % extraction_info_file.path,
            "--output=%s" % kzip.path,
            "--vnames_config=%s" % ctx.file._vnames_config.path,
        ],
    )

    return [OutputGroupInfo(kzips = [kzip], rust_extraction_info = [extraction_info_file])]

def _process_basic_crate_info(ctx, crate_info):
    crate = dict()
    crate["name"] = crate_info.name
    crate["root"] = crate_info.root.path
    crate["edition"] = crate_info.edition
    crate["target"] = ctx.toolchains[Label("@rules_rust//rust:toolchain")].target_triple.str
    crate["crate_type"] = crate_info.type
    crate["is_external"] = crate_info.root.path.startswith("external/")
    crate["deps"] = []

    cfg = ["test", "debug_assertions"]
    if hasattr(ctx.rule.attr, "crate_features"):
        cfg += ['feature="{}"'.format(f) for f in ctx.rule.attr.crate_features]
    if hasattr(ctx.rule.attr, "rustc_flags"):
        cfg += [f[6:] for f in ctx.rule.attr.rustc_flags if f.startswith("--cfg ") or f.startswith("--cfg=")]
    crate["cfg"] = cfg

    relevant_srcs = []
    runfiles = []
    for f in crate_info.srcs.to_list():
        if f.path.startswith(crate_info.root.dirname):
            relevant_srcs.append(f.path)
            runfiles.append(f)
    crate["relevant_srcs"] = relevant_srcs

    out_dir_path = ""
    for dep in crate_info.deps.to_list():
        if dep.build_info != None:
            out_dir_path = dep.build_info.out_dir.path
            runfiles.append(dep.build_info.out_dir)

    crate["out_dir_path"] = out_dir_path

    return (crate, runfiles)

extract_rust_aspect = aspect(
    implementation = _rust_extractor_aspect_impl,
    toolchains = [str(Label("@rules_rust//rust:toolchain"))],
    incompatible_use_toolchain_transition = True,
    doc = "The extraction aspect for Rust",
    required_providers = [CrateInfo],
    attrs = {
        "_rust_extractor": attr.label(
            default = Label("//kythe/rust/extractor"),
            executable = True,
            cfg = "exec",
        ),
        "_vnames_config": attr.label(
            default = Label("//external:vnames_config"),
            allow_single_file = True,
        ),
    },
)
