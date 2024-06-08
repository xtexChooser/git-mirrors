const std = @import("std");
const builtin = @import("builtin");

pub const mem = @import("./mem.zig");
pub const math = @import("./math.zig");

pub const arch = switch (builtin.cpu.arch) {
    .x86, .x86_64 => @import("arch/x86/root.zig"),
    else => @compileError("Unsupported architecture"),
};
