def _impl(ctx):
    ctx.actions.run(
        inputs = [ctx.file.schema],
        outputs = [ctx.outputs.out],
        arguments = ["-g", ctx.attr.gen, "-s", ctx.file.schema.path, "-O", ctx.outputs.out.path],
        progress_message = "Generate %s from %s" % (ctx.outputs.out.short_path, ctx.file.schema.short_path),
        executable = ctx.executable.generator,
    )

generate_flatdata = rule(
    implementation = _impl,
    attrs = {
        "generator": attr.label(
            default = Label("@flatdata//:generator"),
            allow_files = True,
            executable = True,
            cfg = "target",
        ),
        "schema": attr.label(
            allow_single_file = True,
        ),
        "gen": attr.string(
            doc = "the target language to generate (rust, cpp, go or python)",
            mandatory = True,
        ),
        "out": attr.output(mandatory = True),
    },
)
