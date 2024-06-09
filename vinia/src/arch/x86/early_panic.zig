const std = @import("std");

threadlocal var panic_stage: usize = 0;

pub fn panic(msg: []const u8, stacktrace: ?*std.builtin.StackTrace, ret_addr: ?usize) noreturn {
    @setCold(true);
    _ = stacktrace;

    nosuspend switch (panic_stage) {
        0 => {
            panic_stage = 1;
            const addr = ret_addr orelse @returnAddress();
            print("panic: {s} @ 0x{x}", .{ msg, addr }) catch @panic("panic panic~~~");
        },
        1 => {
            panic_stage = 2;
            print("panic: Panicked during a panic", .{}) catch {};
        },
        else => {},
    };

    while (true)
        asm volatile ("hlt");
}

fn print(comptime fmt: []const u8, args: anytype) !void {
    @setCold(true);

    var buf: [1024]u8 = undefined;
    const str = std.fmt.bufPrint(&buf, fmt, args) catch fmt;

    const vga_buffer = @as([*]volatile u16, @ptrFromInt(0xB8000));
    for (str, 0..) |byte, i|
        vga_buffer[i] = 0xF0 << 8 | @as(u16, byte);

    std.log.err("{s}", .{str});

    // write again to avoid being overrided by log
    for (str, 0..) |byte, i|
        vga_buffer[i] = 0xF0 << 8 | @as(u16, byte);
}
