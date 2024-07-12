const std = @import("std");
const desc = @import("../desc.zig");
const isX64 = @import("../root.zig").isX64;
const GDT_ENTRY_CODE = @import("./gdt.zig").GDT_ENTRY_CODE;

const ioasm = @import("../ioasm.zig");
const outb = ioasm.outb;
const inb = ioasm.inb;

const interrupts_with_errcode = &[_]u8{ 8, 10, 11, 12, 13, 14, 17, 21 };

/// The early-boot IDT table.
/// Must be initialized with `init_idt_table` before use.
pub var idt_table: [256]desc.InterruptDescriptor = undefined;

comptime {
    for (0..256) |i| {
        const isr_name = std.fmt.comptimePrint("isr{d}", .{i});
        if (std.mem.indexOfScalar(u8, interrupts_with_errcode, i) != null) {
            asm (std.fmt.comptimePrint(
                    \\ .global {s}
                    \\ .type {s}, @function
                    \\ .align 8
                    \\ {s}:
                    \\   push ${d}
                    \\   jmp isr_handler
                , .{ isr_name, isr_name, isr_name, i }));
        } else {
            asm (std.fmt.comptimePrint(
                    \\ .global {s}
                    \\ .type {s}, @function
                    \\ .align 8
                    \\ {s}:
                    \\   push $0 // stub error code
                    \\   push ${d}
                    \\   jmp isr_handler
                , .{ isr_name, isr_name, isr_name, i }));
        }
    }

    if (isX64) {
        asm (
            \\ .global isr_handler
            \\ .type isr_handler, @function
            \\ .align 8
            \\ isr_handler:
            \\   cli
            \\   pushq %rax
            \\   pushq %rcx
            \\   pushq %rdx
            \\   pushq %rbx
            \\   pushq %rbp
            \\   pushq %rsi
            \\   pushq %rdi
            \\   pushq %r8
            \\   pushq %r9
            \\   pushq %r10
            \\   pushq %r11
            \\   pushq %r12
            \\   pushq %r13
            \\   pushq %r14
            \\   pushq %r15
            \\   mov %rsp, %rdi
            \\   call interruptHandler
            \\   popq %r15
            \\   popq %r14
            \\   popq %r13
            \\   popq %r12
            \\   popq %r11
            \\   popq %r10
            \\   popq %r9
            \\   popq %r8
            \\   popq %rdi
            \\   popq %rsi
            \\   popq %rbp
            \\   popq %rbx
            \\   popq %rdx
            \\   popq %rcx
            \\   popq %rax
            \\   add $16, %rsp // drop error code and interrupt number
            \\   sti
            \\   iretq
        );
    } else {
        asm (
            \\ .global isr_handler
            \\ .type isr_handler, @function
            \\ .align 8
            \\ isr_handler:
            \\   cli
            \\   pushal
            \\   pushl %esp
            \\   call interruptHandler
            \\   popal
            \\   add $8, %esp // drop error code and interrupt number
            \\   sti
            \\   iretl
        );
    }
}

fn init_idt_table() void {
    inline for (&idt_table, 0..) |*entry, i| {
        const isr = @extern(*const fn () callconv(.Interrupt) void, .{
            .name = std.fmt.comptimePrint("isr{d}", .{i}),
            .linkage = .strong,
        });
        const offset = @intFromPtr(isr);
        entry.* = desc.InterruptDescriptor{
            .offset0 = @truncate(offset),
            .offset1 = @intCast(offset >> 16),
            .segment = .{ .priv_level = 0, .table = .gdt, .index = GDT_ENTRY_CODE },
            .priv_level = 3,
        };
    }
}

fn disable_pic() void {
    outb(0x21, 0xff);
    outb(0xA1, 0xff);
}

pub fn load_idt() void {
    init_idt_table();
    disable_pic();
    desc.loadIdt(&idt_table);
    asm volatile ("sti");
}

pub const InterruptContext = if (isX64) InterruptContext64 else InterruptContext32;

pub const InterruptContext64 = packed struct {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    interrupt: u64,
    error_code: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
};

pub const InterruptContext32 = packed struct {
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    handler_esp: u32,
    ebp: u32,
    esi: u32,
    edi: u32,
    interrupt: u32,
    error_code: u32,
    origin_esp: u32,
    cs: u32,
    eflags: u32,
    // the following 2 fields are only available when a priv-level change occurs
    esp: u32,
    ss: u32,
};

export fn interruptHandler(ctx: *InterruptContext) callconv(.C) void {
    std.log.debug("interrupt trigered, {any}", .{ctx.*});
    if (ctx.interrupt != 3) @panic("ISR triggered");
}
