const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    // @TODO: load version from build.zig.zon
    const version = b.option([]const u8, "version", "Version of Cane") orelse "0.0.1";

    const vinia = b.dependency("vinia", .{ .target = target, .optimize = optimize });
    const vinia_core = vinia.artifact("vinia");
    b.installArtifact(vinia_core);

    const dist_install_dir = std.Build.InstallDir{ .custom = "dist" };

    const test_step = b.step("testall", "Run unit tests");
    test_step.dependOn(&(vinia.builder.top_level_steps.get("test") orelse unreachable).*.step);

    // Bootloaders
    switch (target.result.cpu.arch) {
        .x86_64, .x86 => {
            const vinia_mb = vinia.artifact("vinia-multiboot");
            b.installArtifact(vinia_mb);

            // GRUB ISO
            const gen_grub_cfg = b.addSystemCommand(&.{"scripts/x86/gen-iso/grub-cfg.sh"});
            gen_grub_cfg.addFileInput(b.path("scripts/x86/gen-iso/grub-cfg.sh"));
            gen_grub_cfg.setEnvironmentVariable("CANE_VERSION", version);
            const grub_cfg_modules = gen_grub_cfg.addOutputFileArg("modules.txt");
            const grub_cfg = gen_grub_cfg.captureStdOut();

            const gen_grub_eltorito = b.addSystemCommand(&.{"scripts/x86/gen-iso/eltorito.sh"});
            gen_grub_eltorito.addFileInput(b.path("scripts/x86/gen-iso/eltorito.sh"));
            const grub_eltorito = gen_grub_eltorito.addOutputFileArg("grub.img");
            gen_grub_eltorito.addFileArg(grub_cfg_modules);

            const gen_iso_script = b.addSystemCommand(&.{"scripts/x86/gen-iso/script.sh"});
            gen_iso_script.addFileInput(b.path("scripts/x86/gen-iso/script.sh"));
            gen_iso_script.setEnvironmentVariable("CANE_VERSION", version);
            const iso_script = gen_iso_script.captureStdOut();

            const gen_iso = b.addSystemCommand(&.{"scripts/x86/gen-iso/iso.sh"});
            gen_iso.addFileInput(b.path("scripts/x86/gen-iso/iso.sh"));
            const iso = gen_iso.addOutputFileArg("cane.iso");
            gen_iso.addFileArg(iso_script);

            gen_iso_script.addPrefixedFileArg("GRUB_CFG=", grub_cfg);
            gen_iso.addFileInput(grub_cfg);
            gen_iso_script.addPrefixedFileArg("GRUB_ELTORITO=", grub_eltorito);
            gen_iso.addFileInput(grub_eltorito);
            // @TODO: https://github.com/ziglang/zig/pull/20211
            gen_iso_script.addPrefixedArtifactArg("VINIA_MULTIBOOT=", vinia_mb);
            gen_iso_script.addPrefixedArtifactArg("VINIA=", vinia_core);

            const install_iso = b.addInstallFileWithDir(iso, dist_install_dir, "cane.iso");
            b.getInstallStep().dependOn(&install_iso.step);

            // Run QEMU
            const run_qemu = b.addSystemCommand(&.{b.fmt("qemu-system-{s}", .{if (target.result.cpu.arch == .x86) "i386" else "x86_64"})});
            run_qemu.addArgs(&.{
                "-name",    "Cane",
                "-uuid",    "aea208ce-c780-44bb-b825-0b31d84c86f1",
                // Memory
                "-m",       "512M",
                // Debugging
                "-chardev", "socket,path=qemugdb,server=on,wait=off,id=gdb0",
                "-gdb",     "chardev:gdb0",
            });
            run_qemu.disable_zig_progress = false;
            run_qemu.addArg("-cdrom");
            run_qemu.addFileArg(iso);
            if (b.args) |args| {
                run_qemu.addArgs(args);
            }
            const run_qemu_step = b.step("qemu", "Run QEMU");
            run_qemu_step.dependOn(&run_qemu.step);
        },
        else => unreachable,
    }
}
