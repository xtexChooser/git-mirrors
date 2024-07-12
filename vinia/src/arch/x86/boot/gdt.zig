const std = @import("std");
const desc = @import("../desc.zig");

pub const GDT_ENTRY_NUM = 3;
pub const GDT_ENTRY_DATA = 1;
pub const GDT_ENTRY_CODE = 2;

pub const gdt_table = if (@hasDecl(@import("root"), "gdt_table"))
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
