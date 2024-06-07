const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    // @TODO: load version from build.zig.zon
    const version = b.option([]const u8, "version", "Version of Cane") orelse "0.0.1";

    const vinia = b.dependency("vinia", .{ .target = target, .optimize = optimize });
    _ = vinia.artifact("vinia");

    const dist_install_dir = std.Build.InstallDir{ .custom = "dist" };

    // Bootloaders
    switch (target.result.cpu.arch) {
        .x86_64 => {
            // Multiboot
            const vinia_mb = vinia.artifact("vinia-multiboot");

            const gen_grub_cfg = b.addSystemCommand(&.{"scripts/x86_64/gen-iso/grub-cfg.sh"});
            gen_grub_cfg.addFileInput(b.path("scripts/x86_64/gen-iso/grub-cfg.sh"));
            gen_grub_cfg.setEnvironmentVariable("CANE_VERSION", version);
            const grub_cfg_modules = gen_grub_cfg.addOutputFileArg("modules.txt");
            const grub_cfg = gen_grub_cfg.captureStdOut();

            const gen_grub_eltorito = b.addSystemCommand(&.{"scripts/x86_64/gen-iso/eltorito.sh"});
            gen_grub_eltorito.addFileInput(b.path("scripts/x86_64/gen-iso/eltorito.sh"));
            const grub_eltorito = gen_grub_eltorito.addOutputFileArg("grub.img");
            gen_grub_eltorito.addFileArg(grub_cfg_modules);

            const gen_iso_script = b.addSystemCommand(&.{"scripts/x86_64/gen-iso/script.sh"});
            gen_iso_script.addFileInput(b.path("scripts/x86_64/gen-iso/script.sh"));
            gen_iso_script.setEnvironmentVariable("CANE_VERSION", version);
            gen_iso_script.addPrefixedFileArg("GRUB_CFG=", grub_cfg);
            gen_iso_script.addPrefixedFileArg("GRUB_ELTORITO=", grub_eltorito);
            // @TODO: https://github.com/ziglang/zig/pull/20211
            gen_iso_script.addPrefixedArtifactArg("VINIA_MULTIBOOT=", vinia_mb);
            const iso_script = gen_iso_script.captureStdOut();

            const gen_iso = b.addSystemCommand(&.{"scripts/x86_64/gen-iso/iso.sh"});
            gen_iso.addFileInput(b.path("scripts/x86_64/gen-iso/iso.sh"));
            const iso = gen_iso.addOutputFileArg("cane.iso");
            gen_iso.addFileArg(iso_script);

            const install_iso = b.addInstallFileWithDir(iso, dist_install_dir, "x86_64/cane.iso");
            b.getInstallStep().dependOn(&install_iso.step);
        },
        else => unreachable,
    }

    // b.addSystemCommand([].{""});

    // b.installArtifact(exe);

    // const run_cmd = b.addRunArtifact(exe);
    // run_cmd.step.dependOn(b.getInstallStep());
    // if (b.args) |args| {
    //     run_cmd.addArgs(args);
    // }

    const test_step = b.step("testall", "Run unit tests");
    test_step.dependOn(&(vinia.builder.top_level_steps.get("test") orelse unreachable).*.step);
}
