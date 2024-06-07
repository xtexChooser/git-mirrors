const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const pic = b.option(bool, "pic", "Produce Position Independent Code");

    // The vinia module
    const mod = b.addModule("vinia", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = false,
        .link_libcpp = false,
        // @TODO: SSP
        // .stack_protector = true,
        // .stack_check = true,
        .pic = pic,
    });

    // The vinia library
    const lib = b.addStaticLibrary(.{
        .name = "vinia",
        .target = target,
        .optimize = optimize,
    });
    lib.root_module = mod.*;
    b.installArtifact(lib);

    // Bootloaders
    switch (target.result.cpu.arch) {
        .x86_64 => {
            // Multiboot
            const mb_exe = b.addExecutable(.{
                .name = "vinia-multiboot",
                .root_source_file = b.path("src/arch/x86_64/multiboot/main.zig"),
                .target = b.resolveTargetQuery(.{
                    .cpu_arch = std.Target.Cpu.Arch.x86,
                    .os_tag = std.Target.Os.Tag.freestanding,
                    .abi = std.Target.Abi.none,
                    .cpu_model = .{ .explicit = &std.Target.x86.cpu.bonnell },
                    .ofmt = .elf,
                    // .cpu_features_sub = mb_cpu_sub,
                    // .cpu_features_add = mb_cpu_add,
                }),
                .optimize = optimize,
                .single_threaded = true,
                .pic = true,
                .link_libc = false,
                .linkage = .static,
            });
            mb_exe.setLinkerScript(b.path("src/arch/x86_64/multiboot/linker.ld"));
            b.installArtifact(mb_exe);
        },
        else => unreachable,
    }

    // Unit tests
    const unit_tests = b.addTest(.{
        .root_source_file = b.path(""),
        .target = target,
        .optimize = optimize,
    });
    unit_tests.root_module = mod.*;
    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
