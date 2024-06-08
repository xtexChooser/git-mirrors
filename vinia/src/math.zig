const std = @import("std");
const builtin = std.builtin;
const testing = std.testing;

/// Sign-extend a integer
/// See also: https://bottomupcs.com/ch02s02.html#sign_extension
pub inline fn signExtend(
    to_bits: comptime_int,
    value: anytype,
) @Type(.{ .Int = .{ .signedness = .unsigned, .bits = to_bits } }) {
    const from_bits = @typeInfo(@TypeOf(value)).Int.bits;
    return @bitCast(@as(
        @Type(.{ .Int = .{ .signedness = .signed, .bits = to_bits } }),
        @as(@Type(.{ .Int = .{ .signedness = .signed, .bits = from_bits } }), @bitCast(value)),
    ));
}

test signExtend {
    try testing.expectEqual(~@as(u64, 9), signExtend(64, ~@as(u32, 9)));
    try testing.expectEqual(~@as(u128, 12345), signExtend(128, ~@as(u128, 12345)));
    try testing.expectEqual(~@as(u256, 12345), signExtend(256, ~@as(u32, 12345)));
    try testing.expectEqual(@as(u64, 9), signExtend(64, @as(u32, 9)));
    try testing.expectEqual(@as(u128, 12345), signExtend(128, @as(u128, 12345)));
    try testing.expectEqual(@as(u256, 12345), signExtend(256, @as(u32, 12345)));
}
