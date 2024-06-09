const std = @import("std");

pub fn build(b: *std.Build) !void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const pic = b.option(bool, "pic", "Produce Position Independent Code") orelse true;

    // The vinia module
    const vinia = b.addModule("vinia", .{
        .root_source_file = b.path("src/root.zig"),
        // should be initialized when used
        .target = null,
        .optimize = optimize,
        .link_libc = false,
        .link_libcpp = false,
        // @TODO: SSP
        // .stack_protector = true,
        // .stack_check = true,
        .pic = pic,
    });

    // The vinia executable
    const exe = b.addExecutable(.{
        .name = "vinia",
        .target = target,
        .optimize = optimize,
        .pic = pic,
    });
    exe.pie = pic;
    exe.root_module = vinia.*;
    exe.root_module.root_source_file = b.path("src/main.zig");
    exe.root_module.resolved_target = target;
    b.installArtifact(exe);

    // Bootloaders
    switch (target.result.cpu.arch) {
        .x86_64, .x86 => {
            // Multiboot
            const mb_exe = b.addExecutable(.{
                .name = "vinia-multiboot",
                .root_source_file = b.path("src/arch/x86/boot/multiboot/main.zig"),
                .target = b.resolveTargetQuery(.{
                    .cpu_arch = std.Target.Cpu.Arch.x86,
                    .os_tag = std.Target.Os.Tag.freestanding,
                    .abi = std.Target.Abi.none,
                    .cpu_model = .baseline,
                    .ofmt = .elf,
                    .cpu_features_sub = std.Target.x86.featureSet(&[_]std.Target.x86.Feature{
                        .mmx,  .sse,   .sse2,
                        .sse3, .mmx,   .avx,
                        .avx2, .ssse3,
                    }),
                }),
                .optimize = optimize,
                .single_threaded = true,
                .pic = false,
                .link_libc = false,
                .linkage = .static,
            });
            mb_exe.pie = false;
            mb_exe.setLinkerScript(b.path("src/arch/x86/boot/multiboot/linker.ld"));
            const mb_vinia = try b.allocator.create(std.Build.Module);
            mb_vinia.* = vinia.*;
            mb_vinia.pic = false;
            mb_vinia.single_threaded = true;
            mb_vinia.resolved_target = mb_exe.root_module.resolved_target;
            mb_exe.root_module.addImport("vinia", mb_vinia);
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
    unit_tests.root_module = vinia.*;
    const run_unit_tests = b.addRunArtifact(unit_tests);
    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
