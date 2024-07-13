const std = @import("std");
const assert = std.debug.assert;
const isX64 = @import("./root.zig").isX64;
const RingLevel = @import("./root.zig").RingLevel;

pub const SegmentDescriptor = packed struct(u64) {
    limit0: u16,
    base0: u24,
    ty: SegmentType,
    desc_type: DescriptorType = .code_data,
    priv_level: RingLevel,
    present: bool = true,
    limit1: u4,
    unused: u1 = undefined,
    long_mode_code: bool,
    default: packed union {
        code_seg: DefaultOpSize,
        stack_seg: DefaultSpSize,
        expand_down: ExpandDownBound,
    } = .{ .code_seg = .@"32bits" },
    granularity: Granularity = .@"4k",
    base2: u8,
};

pub const Granularity = enum(u1) {
    byte = 0,
    @"4k" = 1,
};

pub const DescriptorType = enum(u1) {
    system = 0,
    code_data = 1,
};

pub const DefaultOpSize = enum(u1) {
    @"16bits" = 0,
    @"32bits" = 1,
};

pub const DefaultSpSize = enum(u1) {
    @"16bits" = 0,
    @"32bits" = 1,
};

pub const ExpandDownBound = enum(u1) {
    @"4GiB" = 0,
    @"64KiB" = 1,
};

pub const SegmentType = packed union {
    data_seg: DataSegmentType,
    code_seg: CodeSegmentType,
};

pub const DataSegmentType = packed struct(u4) {
    accessed: bool = false,
    write: bool,
    expand_down: bool = false,
    _: u1 = 0,
};

pub const CodeSegmentType = packed struct(u4) {
    accessed: bool = false,
    read: bool,
    conforming: bool = false,
    _: u1 = 1,
};

pub const Pointer = packed struct(if (isX64) u80 else u48) {
    limit: u16,
    base: if (isX64) u64 else u32,
};

pub inline fn loadGdtr(pointer: *const Pointer) void {
    if (isX64) {
        asm volatile ("lgdtq (%[gdt])"
            :
            : [gdt] "r{eax}" (@intFromPtr(pointer)),
        );
    } else {
        asm volatile ("lgdtl (%[gdt])"
            :
            : [gdt] "r{eax}" (@intFromPtr(pointer)),
        );
    }
}

pub inline fn loadGdt(desc: []const SegmentDescriptor) void {
    if (desc.len == 0) @panic("GDT table is empty");
    const ptr = Pointer{
        .limit = @sizeOf(SegmentDescriptor) * desc.len - 1,
        .base = @intFromPtr(desc.ptr),
    };
    loadGdtr(&ptr);
}

pub const SegmentSelector = packed struct(u16) {
    priv_level: RingLevel,
    table: enum(u1) { gdt = 0, ldt = 1 },
    index: u13,
};

pub inline fn switchDataSeg(seg: SegmentSelector) void {
    asm volatile (
        \\movw %[seg], %%ds
        \\movw %[seg], %%es
        \\movw %[seg], %%fs
        \\movw %[seg], %%gs
        \\movw %[seg], %%ss
        :
        : [seg] "r{bx}" (seg),
    );
    @fence(.seq_cst);
}

pub inline fn switchCodeSeg(seg: SegmentSelector) void {
    asm volatile (
        \\ljmp %[seg], $Lcs_switch
        \\Lcs_switch:
        :
        : [seg] "Nr{bx}" (seg),
    );
    @fence(.seq_cst);
}

// only interrupt-gate is needed by us and therefore implemented.
pub const InterruptDescriptor = if (isX64) InterruptDescriptor64 else InterruptDescriptor32;

const InterruptDescriptor64 = packed struct(u128) {
    offset0: u16,
    segment: SegmentSelector,
    ist: u2 = 0,
    constant0: u5 = 0,
    ty: u4 = 0b0111,
    constant1: u1 = 0,
    priv_level: RingLevel,
    present: bool = true,
    offset1: u48,
    reserved: u32 = 0,
};

const InterruptDescriptor32 = packed struct(u64) {
    offset0: u16,
    segment: SegmentSelector,
    reserved: u5 = 0,
    constant0: u8 = 0b01110000,
    priv_level: RingLevel,
    present: bool = true,
    offset1: u16,
};

pub inline fn loadIdtr(pointer: *const Pointer) void {
    if (isX64) {
        asm volatile ("lidtq (%[idt])"
            :
            : [idt] "r{eax}" (@intFromPtr(pointer)),
        );
    } else {
        asm volatile ("lidtl (%[idt])"
            :
            : [idt] "r{eax}" (@intFromPtr(pointer)),
        );
    }
}

pub inline fn loadIdt(desc: []const InterruptDescriptor) void {
    if (desc.len == 0) @panic("IDT table is empty");
    const ptr = Pointer{
        .limit = @sizeOf(InterruptDescriptor) * desc.len - 1,
        .base = @intFromPtr(desc.ptr),
    };
    loadIdtr(&ptr);
}
