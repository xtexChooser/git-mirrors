const std = @import("std");
const mb = @import("./multiboot.zig");
const early_log = @import("vinia-x86").early_log;

const MULTIBOOT_FLAGS = mb.MULTIBOOT_PAGE_ALIGN | mb.MULTIBOOT_MEMORY_INFO;

export var multiboot_header align(4) linksection(".multiboot") = mb.Header{
    .flags = MULTIBOOT_FLAGS,
    .checksum = @as(c_uint, @bitCast(-@as(c_int, mb.MULTIBOOT_HEADER_MAGIC + MULTIBOOT_FLAGS))),
};

export var stack: [32 * 1024]u8 align(16) linksection(".bss") = undefined;

export var multiboot_magic: u32 = undefined;
export var multiboot_bootinfo_addr: u32 = undefined;

comptime {
    asm (
        \\.global _start;
        \\.type _start, @function;
        \\_start:
        \\  movl %eax, multiboot_magic
        \\  movl %ebx, multiboot_bootinfo_addr
        \\  lea [stack + (32 * 1024)], %esp
        \\  movl %esp, %ebp
        \\  cli
        \\  jmp _start_zig
    );
}

extern fn _start() callconv(.Naked) noreturn;

export fn _start_zig() callconv(.C) noreturn {
    main();

    while (true)
        asm volatile ("hlt");
}

pub const panic = @import("vinia-x86").early_panic.panic;

pub const std_options = std.Options{
    .logFn = early_log.logFn(.{
        early_log.vga_writer,
    }),
};

pub fn main() void {
    if (multiboot_magic != mb.MULTIBOOT_BOOTLOADER_MAGIC)
        @panic("Invalid multiboot bootloader magic");
    early_log.clear_vga_screen();

    const bootinfo = @as(*mb.Info, @ptrFromInt(multiboot_bootinfo_addr));
    // bootinfo.
    _ = bootinfo;
    inline for (0..10) |i| {
        std.log.warn("test log {d}", .{i});
    }
}
