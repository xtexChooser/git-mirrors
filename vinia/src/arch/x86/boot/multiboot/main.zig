const std = @import("std");
const elf = std.elf;
const log = std.log.scoped(.multiboot);

const vinia = @import("vinia");
const arch = vinia.arch;
const early_log = arch.early_log;
const ForwardPointerAllocator = vinia.mem.ForwardPointerAllocator;

const mb = @import("./multiboot.zig");

const MULTIBOOT_FLAGS = mb.MULTIBOOT_PAGE_ALIGN | mb.MULTIBOOT_MEMORY_INFO;

export var multiboot_header align(4) linksection(".multiboot") = mb.Header{
    .flags = MULTIBOOT_FLAGS,
    .checksum = @as(c_uint, @bitCast(-@as(c_int, mb.MULTIBOOT_HEADER_MAGIC + MULTIBOOT_FLAGS))),
};

export var stack: [32 * 1024]u8 align(16) linksection(".bss") = undefined;

export var multiboot_magic: u32 = undefined;
export var multiboot_bootinfo_addr: u32 = undefined;

comptime {
    asm (
        \\.global _start;
        \\.type _start, @function;
        \\_start:
        \\  movl %eax, multiboot_magic
        \\  movl %ebx, multiboot_bootinfo_addr
        \\  lea [stack + (32 * 1024)], %esp
        \\  movl %esp, %ebp
        \\  cli
        \\  jmp _start_zig
    );
}

extern fn _start() callconv(.Naked) noreturn;

export fn _start_zig() callconv(.C) noreturn {
    main();

    while (true)
        asm volatile ("hlt");
}

// defined in linker.ld
extern var _ELF_BEGIN_: opaque {};
extern var _ELF_END_: opaque {};

pub const panic = arch.early_panic.panic;

pub const std_options = std.Options{
    .logFn = early_log.logFn(.{
        early_log.vga.writer, early_log.serial.writer,
    }),
};

var multiboot_bootinfo: mb.BootInfo = undefined;

pub const MultibootAllocator = ForwardPointerAllocator(*mb.BootInfo, struct {
    inline fn isOverlapped(from1: *void, size1: usize, from2: usize, size2: usize) ?*void {
        if (size1 == 0 or size2 == 0) return null;
        const end1 = @intFromPtr(from1) + size1;
        const end2 = from2 + size2;
        if (@intFromPtr(from1) <= end2 and from2 <= (end1))
            // two range overlapped
            return @ptrFromInt(@max(end1, end2));
        return null;
    }

    pub fn checker(bootinfo: *mb.BootInfo, ptr: *void, size: usize) ?*void {
        // overlap with the loaded multiboot ELF
        if (isOverlapped(
            ptr,
            size,
            @intFromPtr(&_ELF_BEGIN_),
            @intFromPtr(&_ELF_END_) - @intFromPtr(&_ELF_BEGIN_),
        )) |p| return p;
        // overlap with any boot info
        if (isOverlapped(
            ptr,
            size,
            @intFromPtr(bootinfo),
            @sizeOf(mb.BootInfo),
        )) |p| return p;
        if (bootinfo.flags & mb.MULTIBOOT_INFO_CMDLINE != 0)
            if (isOverlapped(
                ptr,
                size,
                @intCast(bootinfo.cmdline),
                std.mem.len(@as([*:0]const u8, @ptrFromInt(bootinfo.cmdline))),
            )) |p| return p;
        if (bootinfo.flags & mb.MULTIBOOT_INFO_MODS != 0) {
            for (0..bootinfo.mods_count, @as([*]mb.Module, @ptrFromInt(bootinfo.mods_addr))) |_, *mod| {
                if (isOverlapped(
                    ptr,
                    size,
                    mod.mod_start,
                    mod.mod_end - mod.mod_start,
                )) |p| return p;
                if (mod.cmdline != 0)
                    if (isOverlapped(
                        ptr,
                        size,
                        mod.cmdline,
                        std.mem.len(@as([*:0]const u8, @ptrFromInt(mod.cmdline))),
                    )) |p| return p;
            }
        }
        if (bootinfo.flags & mb.MULTIBOOT_INFO_MEM_MAP != 0) {
            var mmap = @as(*mb.MmapEntry, @ptrFromInt(bootinfo.mmap_addr));
            var remaining = bootinfo.mmap_length;
            while (remaining > 0) {
                check: {
                    if (mmap.type == mb.MULTIBOOT_MEMORY_AVAILABLE) break :check;
                    const addr_max = @as(c_ulonglong, @intCast(std.math.maxInt(usize)));
                    if (mmap.addr > addr_max) break :check;
                    if (isOverlapped(
                        ptr,
                        size,
                        @intCast(mmap.addr),
                        @intCast(@min(addr_max, mmap.addr + mmap.len) - mmap.addr),
                    )) |p| return p;
                }

                const entry_size = mmap.size + @sizeOf(@TypeOf(@field(mmap, "size")));
                remaining -= entry_size;
                mmap = @ptrFromInt(@intFromPtr(mmap) + entry_size);
            }
        }
        return null;
    }
}.checker);
pub var multiboot_allocator: ?MultibootAllocator = null;

pub fn main() void {
    if (multiboot_magic != mb.MULTIBOOT_BOOTLOADER_MAGIC)
        @panic("Invalid multiboot bootloader magic");
    @memset(@as([*]volatile u8, @ptrFromInt(0x7b00))[0..0x100], 0);
    early_log.vga.clear();
    early_log.serial.init();

    // copy boot info
    multiboot_bootinfo = @as(*mb.BootInfo, @ptrFromInt(multiboot_bootinfo_addr)).*;
    const bootinfo = &multiboot_bootinfo;

    // init allocator
    const alloc_base = @as(*void, @ptrFromInt(@max(multiboot_bootinfo_addr + @sizeOf(mb.BootInfo), @intFromPtr(&_ELF_END_))));
    const alloc_end = result: {
        if (multiboot_bootinfo.flags & mb.MULTIBOOT_INFO_MEMORY != 0) {
            break :result @as(*void, @ptrFromInt((1024 + multiboot_bootinfo.mem_upper) * 1024));
        } else {
            break :result @as(*void, @ptrFromInt(multiboot_bootinfo_addr + (1024 * 1024 * 256)));
        }
    };
    multiboot_allocator = MultibootAllocator.init(&multiboot_bootinfo, alloc_base, alloc_end);
    const alloc = multiboot_allocator.?.allocator();
    _ = alloc;

    // find core module
    if (bootinfo.flags & mb.MULTIBOOT_INFO_MODS == 0)
        @panic("Modules is required for multiboot booting");
    if (bootinfo.mods_count == 0)
        @panic("Vinia core is not found in multiboot modules");
    const mb_mods = @as([*]mb.Module, @ptrFromInt(bootinfo.mods_addr));
    const mod = if (bootinfo.mods_count == 1)
        mb_mods[0]
    else core_mod: {
        for (0..bootinfo.mods_count, mb_mods) |index, *mod| {
            const mod_cmdline = @as([*:0]const u8, @ptrFromInt(mod.cmdline));
            var iter = std.mem.splitScalar(u8, mod_cmdline[0..std.mem.len(mod_cmdline)], ' ');
            while (iter.next()) |arg| {
                if (std.mem.eql(u8, arg, "vinia.bootloader.core")) {
                    log.info("Using the {d}-th module as vinia core", .{index + 1});
                    break :core_mod mod.*;
                }
            }
        }
        log.err("None of the multiboot modules is marked with 'vinia.bootloader.core', please add this to one of the multiboot modules", .{});
        @panic("Cannot determine the module which is the vinia core");
    };
    const core = @as([*]const u8, @ptrFromInt(mod.mod_start))[0..(mod.mod_end - mod.mod_start)];

    arch.boot.boot() catch |err| {
        std.builtin.panic(@errorName(err), @errorReturnTrace(), null);
    };
    _ = core;

    // var core_buf = std.io.fixedBufferStream(core);

    // const ehdr = std.elf.Header.read(&core_buf) catch @panic("Invalid ELF in vinia core");
    // log.info("{any}", .{ehdr});
}
