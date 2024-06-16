pub const ioasm = @import("./ioasm.zig");
pub const early_panic = @import("./early_panic.zig");
pub const early_log = @import("./early_log.zig");
pub const boot = @import("./boot.zig");
pub const desc = @import("./desc.zig");

pub const RingLevel = u2;

pub const is64 = @import("builtin").cpu.arch == .x86_64;
