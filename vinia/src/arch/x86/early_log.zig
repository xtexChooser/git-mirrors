const std = @import("std");
const log = std.log;
const Level = std.log.Level;

const assembly = @import("./assembly.zig");
const outb = assembly.outb;
const inb = assembly.inb;

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

pub const vga_writer_context = @as(*VgaWriterContext, @ptrFromInt(0x7b00));

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
}.write){ .context = vga_writer_context };

pub fn clear_vga_screen() void {
    @memset(@as([*]volatile u16, @ptrFromInt(0xB8000))[0..(80 * 24)], 0);
}

const SerialWriterContext = struct {
    port: ?u16,
};

pub const serial_writer_context = @as(*SerialWriterContext, @ptrFromInt(0x7b00 + @sizeOf(VgaWriterContext)));

pub const serial_writer = std.io.GenericWriter(*SerialWriterContext, error{}, struct {
    fn write(ctx: *SerialWriterContext, bytes: []const u8) error{}!usize {
        if (ctx.port) |port| {
            for (bytes) |byte| {
                if (byte == '\n') {
                    write_to_serial(port, '\r');
                }
                write_to_serial(port, byte);
            }
        }
        return bytes.len;
    }
}.write){ .context = serial_writer_context };

pub fn test_serial_port(port: u16) bool {
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

pub fn init_serial_port() void {
    if (serial_writer_context.port == null) {
        const bda_ports = @as([*]volatile u16, @ptrFromInt(0x0400));
        inline for ([_]u16{
            0x3F8,        0x2F8,        0x3E8,        0x2E8,
            0x5F8,        0x4F8,        0x5E8,        0x4E8,
            bda_ports[0], bda_ports[1], bda_ports[2], bda_ports[3],
        }) |port| {
            if (test_serial_port(port)) {
                serial_writer_context.port = port;
                log.info("found UART serial port at 0x{X}", .{port});
                break;
            }
        }
    }

    if (serial_writer_context.port) |port| {
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

pub fn write_to_serial(port: u16, byte: u8) void {
    // wait until the THRE is set
    while ((inb(port + 5) & 0x20) == 0) {}
    // write to the buffer
    outb(port, byte);
}
