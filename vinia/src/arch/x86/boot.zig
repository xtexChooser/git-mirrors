const std = @import("std");
const mem = std.mem;
const Allocator = std.mem.Allocator;
const log = std.log.scoped(.x86_boot);
const desc = @import("./desc.zig");

pub const gdt = @import("./boot/gdt.zig");
pub const interrupt = @import("./boot/interrupt.zig");

pub const boot_allocator: fn () mem.Allocator = @field(@import("root"), "boot_allocator");

pub const BootInfo = struct {
    alloc: Allocator,
    core_elf: []const u8,
    bootloader_str: []const u8,
    arch: ?enum { x86, x86_64 } = null,
};

/// Things to be done by bootloader before calling this:
///   - Enter protect mode
///   - Load boot GDT (`gdt.load_gdt`)
///   - Setup IDT
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
