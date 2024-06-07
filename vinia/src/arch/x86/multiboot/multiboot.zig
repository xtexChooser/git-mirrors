const std = @import("std");
const mem = std.mem;
const assert = std.debug.assert;

/// How many bytes from the start of the file we search for the header.
pub const MULTIBOOT_SEARCH = 8192;
pub const MULTIBOOT_HEADER_ALIGN = 4;

/// The magic field should contain this.
pub const MULTIBOOT_HEADER_MAGIC = @as(c_uint, 0x1BADB002);

/// This should be in %eax.
pub const MULTIBOOT_BOOTLOADER_MAGIC = @as(c_uint, 0x2BADB002);

/// Alignment of multiboot modules.
pub const MULTIBOOT_MOD_ALIGN = 0x00001000;
/// Alignment of the multiboot info structure.
pub const MULTIBOOT_INFO_ALIGN = 0x00000004;
/// Align all boot modules on i386 page (4KB) boundaries.
pub const MULTIBOOT_PAGE_ALIGN = 0x00000001;
/// Must pass memory information to OS.
pub const MULTIBOOT_MEMORY_INFO = 0x00000002;
/// Must pass video information to OS.
pub const MULTIBOOT_VIDEO_MODE = 0x00000004;
/// This flag indicates the use of the address fields in the header.
pub const MULTIBOOT_AOUT_KLUDGE = 0x00010000;

/// is there basic lower/upper memory information?
pub const MULTIBOOT_INFO_MEMORY = 0x00000001;
/// is there a boot device set?
pub const MULTIBOOT_INFO_BOOTDEV = 0x00000002;
/// is the command-line defined?
pub const MULTIBOOT_INFO_CMDLINE = 0x00000004;
/// are there modules to do something with?
pub const MULTIBOOT_INFO_MODS = 0x00000008;
/// is there a symbol table loaded?
pub const MULTIBOOT_INFO_AOUT_SYMS = 0x00000010;
/// is there an ELF section header table?
pub const MULTIBOOT_INFO_ELF_SHDR = 0x00000020;
/// is there a full memory map?
pub const MULTIBOOT_INFO_MEM_MAP = 0x00000040;
/// Is there drive info?
pub const MULTIBOOT_INFO_DRIVE_INFO = 0x00000080;
/// Is there a config table?
pub const MULTIBOOT_INFO_CONFIG_TABLE = 0x00000100;
/// Is there a boot loader name?
pub const MULTIBOOT_INFO_BOOT_LOADER_NAME = 0x00000200;
/// Is there a APM table?
pub const MULTIBOOT_INFO_APM_TABLE = 0x00000400;
/// Is there video information?
pub const MULTIBOOT_INFO_VBE_INFO = 0x00000800;
/// Is there video information?
pub const MULTIBOOT_INFO_FRAMEBUFFER_INFO = 0x00001000;

pub const MULTIBOOT_FRAMEBUFFER_TYPE_INDEXED = 0;
pub const MULTIBOOT_FRAMEBUFFER_TYPE_RGB = 1;
pub const MULTIBOOT_FRAMEBUFFER_TYPE_EGA_TEXT = 2;

pub const MULTIBOOT_MEMORY_AVAILABLE = 1;
pub const MULTIBOOT_MEMORY_RESERVED = 2;
pub const MULTIBOOT_MEMORY_ACPI_RECLAIMABLE = 3;
pub const MULTIBOOT_MEMORY_NVS = 4;
pub const MULTIBOOT_MEMORY_BADRAM = 5;

pub const Header = extern struct {
    magic: c_uint = MULTIBOOT_HEADER_MAGIC,
    flags: c_uint = mem.zeroes(c_uint),
    checksum: c_uint = mem.zeroes(c_uint),
    header_addr: c_uint = mem.zeroes(c_uint),
    load_addr: c_uint = mem.zeroes(c_uint),
    load_end_addr: c_uint = mem.zeroes(c_uint),
    bss_end_addr: c_uint = mem.zeroes(c_uint),
    entry_addr: c_uint = mem.zeroes(c_uint),
    mode_type: c_uint = mem.zeroes(c_uint),
    width: c_uint = mem.zeroes(c_uint),
    height: c_uint = mem.zeroes(c_uint),
    depth: c_uint = mem.zeroes(c_uint),
};

pub const SymbolTable = extern union {
    aout_sym: AoutSymbolTable,
    elf_sec: ElfSectionHeaderTable,
};

pub const AoutSymbolTable = extern struct {
    tabsize: c_uint,
    strsize: c_uint,
    addr: c_uint,
    reserved: c_uint,
};

pub const ElfSectionHeaderTable = extern struct {
    num: c_uint,
    size: c_uint,
    addr: c_uint,
    shndx: c_uint,
};

pub const ColorInfo = extern union {
    indexed: extern struct {
        framebuffer_palette_addr: c_uint = mem.zeroes(c_uint),
        framebuffer_palette_num_colors: c_ushort = mem.zeroes(c_ushort),
    },
    rgb: extern struct {
        framebuffer_red_field_position: u8 = mem.zeroes(u8),
        framebuffer_red_mask_size: u8 = mem.zeroes(u8),
        framebuffer_green_field_position: u8 = mem.zeroes(u8),
        framebuffer_green_mask_size: u8 = mem.zeroes(u8),
        framebuffer_blue_field_position: u8 = mem.zeroes(u8),
        framebuffer_blue_mask_size: u8 = mem.zeroes(u8),
    },
};

pub const Info = extern struct {
    flags: c_uint = mem.zeroes(c_uint),
    mem_lower: c_uint = mem.zeroes(c_uint),
    mem_upper: c_uint = mem.zeroes(c_uint),
    boot_device: c_uint = mem.zeroes(c_uint),
    cmdline: c_uint = mem.zeroes(c_uint),
    mods_count: c_uint = mem.zeroes(c_uint),
    mods_addr: c_uint = mem.zeroes(c_uint),
    syms: SymbolTable = mem.zeroes(SymbolTable),
    mmap_length: c_uint = mem.zeroes(c_uint),
    mmap_addr: [*]MmapEntry = mem.zeroes([*]MmapEntry),
    drives_length: c_uint = mem.zeroes(c_uint),
    drives_addr: c_uint = mem.zeroes(c_uint),
    config_table: c_uint = mem.zeroes(c_uint),
    boot_loader_name: c_uint = mem.zeroes(c_uint),
    apm_table: c_uint = mem.zeroes(c_uint),
    vbe_control_info: c_uint = mem.zeroes(c_uint),
    vbe_mode_info: c_uint = mem.zeroes(c_uint),
    vbe_mode: c_ushort = mem.zeroes(c_ushort),
    vbe_interface_seg: c_ushort = mem.zeroes(c_ushort),
    vbe_interface_off: c_ushort = mem.zeroes(c_ushort),
    vbe_interface_len: c_ushort = mem.zeroes(c_ushort),
    framebuffer_addr: c_ulonglong = mem.zeroes(c_ulonglong),
    framebuffer_pitch: c_uint = mem.zeroes(c_uint),
    framebuffer_width: c_uint = mem.zeroes(c_uint),
    framebuffer_height: c_uint = mem.zeroes(c_uint),
    framebuffer_bpp: u8 = mem.zeroes(u8),
    framebuffer_type: u8 = mem.zeroes(u8),
    color_info: ColorInfo = mem.zeroes(ColorInfo),
};

comptime {
    // multiboot_info.mmap_addr
    assert(@bitSizeOf([*]MmapEntry) == 32);
}

pub const Color = extern struct {
    red: u8 = mem.zeroes(u8),
    green: u8 = mem.zeroes(u8),
    blue: u8 = mem.zeroes(u8),
};

pub const MmapEntry = extern struct {
    size: c_uint align(1) = mem.zeroes(c_uint),
    addr: c_ulonglong align(1) = mem.zeroes(c_ulonglong),
    len: c_ulonglong align(1) = mem.zeroes(c_ulonglong),
    type: c_uint align(1) = mem.zeroes(c_uint),
};

pub const ModuleList = extern struct {
    mod_start: c_uint = mem.zeroes(c_uint),
    mod_end: c_uint = mem.zeroes(c_uint),
    cmdline: c_uint = mem.zeroes(c_uint),
    pad: c_uint = mem.zeroes(c_uint),
};

pub const ApmInfo = extern struct {
    version: c_ushort = mem.zeroes(c_ushort),
    cseg: c_ushort = mem.zeroes(c_ushort),
    offset: c_uint = mem.zeroes(c_uint),
    cseg_16: c_ushort = mem.zeroes(c_ushort),
    dseg: c_ushort = mem.zeroes(c_ushort),
    flags: c_ushort = mem.zeroes(c_ushort),
    cseg_len: c_ushort = mem.zeroes(c_ushort),
    cseg_16_len: c_ushort = mem.zeroes(c_ushort),
    dseg_len: c_ushort = mem.zeroes(c_ushort),
};
