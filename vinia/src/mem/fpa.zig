const std = @import("std");

pub fn ForwardPointerAllocator(comptime Context: type, comptime checker: fn (ctx: Context, ptr: *void, size: usize) ?*void) type {
    return struct {
        const Self = @This();

        ctx: Context,
        ptr: *void,
        end: *void,

        pub fn init(ctx: Context, ptr: *void, end: *void) @This() {
            return .{
                .ctx = ctx,
                .ptr = ptr,
                .end = end,
            };
        }

        pub fn allocator(self: *Self) std.mem.Allocator {
            return .{
                .ptr = self,
                .vtable = &.{
                    .alloc = alloc,
                    .resize = resize,
                    .free = free,
                },
            };
        }

        pub fn alloc(
            ctx: *anyopaque,
            len: usize,
            ptr_align: u8,
            ret_addr: usize,
        ) ?[*]u8 {
            const self: *Self = @ptrCast(@alignCast(ctx));
            const alignment = @as(usize, 1) << @as(std.mem.Allocator.Log2Align, @intCast(ptr_align));
            const aligned_len = @max(len, alignment);
            var ptr = self.ptr;
            _ = ret_addr;

            while (true) {
                if ((@intFromPtr(ptr) + aligned_len) >= @intFromPtr(self.end))
                    return null;
                const extra = @intFromPtr(ptr) % alignment;
                if (extra != 0)
                    ptr = @ptrFromInt(@intFromPtr(ptr) - extra + alignment);
                if (checker(self.ctx, ptr, aligned_len)) |end| {
                    // this area can't be used, skip it
                    ptr = @ptrFromInt(@intFromPtr(end) + 1);
                } else {
                    // found a empty area
                    self.ptr = @ptrFromInt(@intFromPtr(ptr) + aligned_len);
                    return @ptrCast(ptr);
                }
            }
        }

        pub fn resize(
            ctx: *anyopaque,
            buf: []u8,
            buf_align: u8,
            new_len: usize,
            ret_addr: usize,
        ) bool {
            _ = ctx;
            _ = buf;
            _ = buf_align;
            _ = new_len;
            _ = ret_addr;
            return false;
        }

        pub fn free(
            ctx: *anyopaque,
            buf: []u8,
            buf_align: u8,
            ret_addr: usize,
        ) void {
            _ = ctx;
            _ = buf;
            _ = buf_align;
            _ = ret_addr;
            return;
        }
    };
}
