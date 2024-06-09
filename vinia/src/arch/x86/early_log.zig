const std = @import("std");
const log = std.log;
const Level = std.log.Level;

const asm_ = @import("./assembly.zig");
const outb = asm_.outb;
const inb = asm_.inb;

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
            nosuspend inline for (writers) |writer|
                writer.print(level_txt ++ prefix2 ++ format ++ "\n", args) catch return;
        }
    }.log;
}

pub const vga = struct {
    const Context = struct {
        x: u16,
        y: u16,
    };

    pub const context = @as(*Context, @ptrFromInt(0x7b00));

    pub const writer = std.io.GenericWriter(*Context, error{}, struct {
        fn write(ctx: *Context, bytes: []const u8) error{}!usize {
            const buf = @as([*]volatile u16, @ptrFromInt(0xB8000));
            for (bytes) |byte| {
                switch (byte) {
                    '\n' => {
                        ctx.x = 0;
                        ctx.y += 1;
                    },
                    0 => {},
                    else => {
                        buf[ctx.x + (ctx.y * 80)] = 0x07 << 8 | @as(u16, byte);
                        ctx.x += 1;
                        if (ctx.x >= 80) {
                            ctx.x = 0;
                            ctx.y += 1;
                        }
                    },
                }
                if (ctx.y >= 24) {
                    ctx.y -= 1;
                    for (buf, buf[80..(80 * 24)]) |*d, s| d.* = s;
                }
            }
            return bytes.len;
        }
    }.write){ .context = context };

    pub fn clear() void {
        @memset(@as([*]volatile u16, @ptrFromInt(0xB8000))[0..(80 * 24)], 0);
        context.x = 0;
        context.y = 0;
    }
};

pub const serial = struct {
    const Context = struct {
        port: u16,
    };

    pub const context = @as(*Context, @ptrFromInt(0x7b00 + @sizeOf(vga.Context)));

    pub const writer = std.io.GenericWriter(*Context, error{}, struct {
        fn write(ctx: *Context, bytes: []const u8) error{}!usize {
            if (ctx.port != 0) {
                for (bytes) |byte| {
                    if (byte == '\n') {
                        serial.write(ctx.port, '\r');
                    }
                    serial.write(ctx.port, byte);
                }
            }
            return bytes.len;
        }
    }.write){ .context = context };

    pub fn test_port(port: u16) bool {
        const old = inb(port + 4);
        defer outb(port + 4, old);

        outb(port + 4, 0x10);
        if ((inb(port + 6) & 0xF0) != 0x00) return false;

        outb(port + 4, 0x1F);
        if ((inb(port + 6) & 0xF0) != 0xF0) return false;

        outb(port + 4, 0x1E);
        outb(port + 0, 0xAE);
        if (inb(port + 0) != 0xAE) return false;

        return true;
    }

    pub fn init() void {
        if (context.port == 0) {
            const bda_ports = @as([*]volatile u16, @ptrFromInt(0x0400));
            inline for ([_]u16{
                0x3F8,        0x2F8,        0x3E8,        0x2E8,
                0x5F8,        0x4F8,        0x5E8,        0x4E8,
                bda_ports[0], bda_ports[1], bda_ports[2], bda_ports[3],
            }) |port| {
                if (test_port(port)) {
                    log.info("found UART serial port at 0x{X}", .{port});
                    context.port = port;
                    break;
                }
            }
        }

        if (context.port != 0) {
            const port = context.port;
            // disable interrupts
            outb(port + 1, 0);
            // set baudrate to 38400
            outb(port + 3, 0x80);
            outb(port + 0, 0x03);
            outb(port + 1, 0x00);
            // set to 8N1
            outb(port + 3, 0x03);
            // disable FIFO
            outb(port + 2, 0);
        }
    }

    pub fn write(port: u16, byte: u8) void {
        // wait until the THRE is set
        while ((inb(port + 5) & 0x20) == 0) {}
        // write to the buffer
        outb(port, byte);
    }
};
