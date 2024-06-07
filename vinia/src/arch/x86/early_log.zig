const std = @import("std");
const Level = std.log.Level;

const LogFn = fn (
    comptime message_level: Level,
    comptime scope: @TypeOf(.enum_literal),
    comptime format: []const u8,
    args: anytype,
) void;

pub fn logFn(comptime writers: anytype) LogFn {
    return struct {
        pub fn log(
            comptime message_level: Level,
            comptime scope: @Type(.EnumLiteral),
            comptime format: []const u8,
            args: anytype,
        ) void {
            const level_txt = comptime message_level.asText();
            const prefix2 = if (scope == .default) ": " else "(" ++ @tagName(scope) ++ "): ";
            nosuspend {
                inline for (writers) |writer|
                    writer.print(level_txt ++ prefix2 ++ format ++ "\n", args) catch return;
            }
        }
    }.log;
}

const VgaWriterContext = struct {
    x: u16 = 0,
    y: u16 = 0,
};

pub var vga_writer_context = VgaWriterContext{};

pub const vga_writer = std.io.GenericWriter(*VgaWriterContext, error{}, struct {
    fn write(ctx: *VgaWriterContext, bytes: []const u8) error{}!usize {
        const vga_buffer = @as([*]volatile u16, @ptrFromInt(0xB8000));
        for (bytes) |byte| {
            switch (byte) {
                '\n' => {
                    ctx.x = 0;
                    ctx.y += 1;
                },
                0 => {},
                else => {
                    vga_buffer[ctx.x + (ctx.y * 80)] = 0x70 << 8 | @as(u16, byte);
                    ctx.x += 1;
                    if (ctx.x >= 80) {
                        ctx.x = 0;
                        ctx.y += 1;
                    }
                },
            }
            if (ctx.y >= 24) {
                ctx.y -= 1;
                for (vga_buffer, vga_buffer[80..(80 * 24)]) |*d, s| d.* = s;
            }
        }
        return bytes.len;
    }
}.write){ .context = &vga_writer_context };

pub fn clear_vga_screen() void {
    @memset(@as([*]volatile u16, @ptrFromInt(0xB8000))[0..(80 * 24)], 0);
}
