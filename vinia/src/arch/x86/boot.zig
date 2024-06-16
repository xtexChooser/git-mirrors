const std = @import("std");
const mem = std.mem;
const Allocator = std.mem.Allocator;
const log = std.log.scoped(.x86_boot);
const is64 = @import("./root.zig").is64;
const desc = @import("./desc.zig");


pub const BootInfo = struct {
    alloc: Allocator,
    core_elf: []const u8,
    bootloader_str: []const u8,
    arch: ?enum { x86, x86_64 } = null,
};

/// Things to be done by bootloader before calling this:
///   - Enter protect mode
///   - Load boot GDT (`boot_gdt`)
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

pub const GDT_ENTRY_NUM = 3;
pub const GDT_ENTRY_DATA = 1;
pub const GDT_ENTRY_CODE = 2;

export const gdt_table = if (@hasDecl(@import("root"), "gdt_table"))
    @import("root").gdt_table
else
    [GDT_ENTRY_NUM]desc.SegmentDescriptor{
        undefined,
        .{
            .limit0 = 0xffff,
            .base0 = 0,
            .ty = .{ .data_seg = .{ .write = true } },
            .priv_level = 0,
            .limit1 = 0xf,
            .long_mode_code = false,
            .base2 = 0,
        },
        .{
            .limit0 = 0xffff,
            .base0 = 0,
            .ty = .{ .code_seg = .{ .read = true } },
            .priv_level = 0,
            .limit1 = 0xf,
            .long_mode_code = false,
            .base2 = 0,
        },
    };

pub fn load_gdt() void {
    desc.loadGdt(&gdt_table);
    desc.switchDataSeg(.{ .priv_level = 0, .table = .gdt, .index = GDT_ENTRY_DATA });
    desc.switchCodeSeg(.{ .priv_level = 0, .table = .gdt, .index = GDT_ENTRY_CODE });
}
