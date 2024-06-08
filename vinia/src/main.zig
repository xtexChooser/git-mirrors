pub usingnamespace @import("./root.zig");

pub export fn _start() callconv(.C) void {
    while (true)
        asm volatile ("hlt");
}
