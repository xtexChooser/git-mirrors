const mb = @import("./multiboot.zig");

const MULTIBOOT_FLAGS = mb.MULTIBOOT_PAGE_ALIGN | mb.MULTIBOOT_MEMORY_INFO;

export var multiboot_header align(4) linksection(".multiboot") = mb.Header{
    .flags = MULTIBOOT_FLAGS,
    .checksum = @as(c_uint, @bitCast(-@as(c_int, mb.MULTIBOOT_HEADER_MAGIC + MULTIBOOT_FLAGS))),
};

// export var stack_bytes: [16 * 1024]u8 align(16) linksection(".bss") = undefined;
// const stack_bytes_slice = stack_bytes[0..];

export fn _start() callconv(.Naked) noreturn {
    // @call(.{ .stack = stack_bytes_slice }, kmain, .{});

    while (true)
        asm volatile ("hlt");
}
