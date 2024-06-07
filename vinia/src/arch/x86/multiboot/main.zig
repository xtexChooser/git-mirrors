const mb = @import("./multiboot.zig");

const MULTIBOOT_FLAGS = mb.MULTIBOOT_PAGE_ALIGN | mb.MULTIBOOT_MEMORY_INFO;

export var multiboot_header align(4) linksection(".multiboot") = mb.Header{
    .flags = MULTIBOOT_FLAGS,
    .checksum = @as(c_uint, @bitCast(-@as(c_int, mb.MULTIBOOT_HEADER_MAGIC + MULTIBOOT_FLAGS))),
};

export var stack: [32 * 1024]u8 align(16) linksection(".bss") = undefined;

export var multiboot_magic: u32 = undefined;
export var multiboot_bootinfo: u32 = undefined;

comptime {
    asm (
        \\.global _start;
        \\.type _start, @function;
        \\_start:
        \\  movl %eax, multiboot_magic
        \\  movl %ebx, multiboot_bootinfo
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

pub fn main() void {
    @panic("test");
    // while (true) {}
}
