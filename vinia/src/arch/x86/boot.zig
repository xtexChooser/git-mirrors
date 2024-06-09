const std = @import("std");
const mem = std.mem;
const Allocator = std.mem.Allocator;
const log = std.log.scoped(.x86_boot);

pub const BootInfo = struct {
    alloc: Allocator,
    core_elf: []const u8,
    bootloader_str: []const u8,
    arch: ?enum { x86, x86_64 } = null,
};

pub fn boot(bootinfo: *BootInfo) !void {
    log.info("bootloader: {s}", .{bootinfo.bootloader_str});

    var elf_buf = std.io.fixedBufferStream(bootinfo.core_elf);
    const elf_hdr = try std.elf.Header.read(&elf_buf);
    if (bootinfo.arch) |arch| {
        _ = arch;
    } else {
        bootinfo.arch = switch (elf_hdr.machine) {
            .@"386" => .x86,
            .X86_64 => .x86_64,
            else => return error.UnsupportedElfMachine,
        };
    }

    log.info("{any}", .{elf_hdr});
}
