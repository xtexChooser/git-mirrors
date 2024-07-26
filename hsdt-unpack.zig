const std = @import("std");
const log = std.log;
const hsdt = @import("./hsdt.zig");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);
    if (args.len == 1) {
        log.err("usage: {s} <file>", .{args[0]});
        return;
    } else if (args.len != 2) {
        log.err("too many arguments: {s}", .{args[2]});
        return;
    }
    const path = args[1];
    log.info("reading HSDT file from: {s}", .{path});
    const basename = std.fs.path.basename(path);

    const file = try std.fs.cwd().openFile(path, .{ .mode = .read_only });
    defer file.close();

    try file.seekTo(0);
    const offset: usize = if (try file.reader().readInt(u32, .little) != hsdt.DT_TABLE_MAGIC)
        // for DTS dumped from system update package (UPDATE.APP file)
        // there are 4096 bytes of junk at the head of file
        4096
    else
        // for DTS dumped from Huawei's Android device
        // /dev/block/bootdevice/by-name/dts
        // the table is not offseted
        0;
    if (offset == 4096) {
        log.info("Detected DTS partition image dumped from system update package", .{});
    } else {
        log.info("Detected DTS partition image dumped from device", .{});
    }

    try file.seekTo(offset);
    const header = try hsdt.DtTableHeader.read(file.reader().any());
    log.info("Totally {} DTB entries", .{header.entry_count});
    const entries = try allocator.alloc(hsdt.DtEntry, header.entry_count);
    defer allocator.free(entries);
    for (entries) |*entry| {
        entry.* = try hsdt.DtEntry.read(file.reader().any());
    }
    for (entries) |entry| {
        const board_id_str = try std.fmt.allocPrint(allocator, "<0x{x:02} 0x{x:02} 0x{x:02} 0x{x:02}>", .{
            entry.board_id[0], entry.board_id[1],
            entry.board_id[2], entry.board_id[3],
        });
        defer allocator.free(board_id_str);
        const filename = try std.fmt.allocPrint(allocator, "{s}_{x:02}{x:02}{x:02}{x:02}", .{
            basename,          entry.board_id[0],
            entry.board_id[1], entry.board_id[2],
            entry.board_id[3],
        });
        defer allocator.free(filename);
        if (entry.vrl) |vrl| {
            log.info("Found DTB for {s}, DTB at 0x{x} ({} bytes), VRL at 0x{x} ({} bytes)", .{
                board_id_str,
                entry.dtb.offset,
                entry.dtb.len,
                vrl.offset,
                vrl.len,
            });
        } else {
            log.info("Found DTB for {s}, DTB at 0x{x} ({} bytes), no VRL", .{
                board_id_str, entry.dtb.offset, entry.dtb.len,
            });
        }

        {
            const dtb_gz_path = try std.fmt.allocPrint(allocator, "{s}.dtb.gz", .{filename});
            defer allocator.free(dtb_gz_path);
            const dtb_gz_file = try std.fs.cwd().createFile(dtb_gz_path, .{
                .read = true,
                .lock = .exclusive,
            });
            defer dtb_gz_file.close();
            _ = try file.copyRangeAll(offset + entry.dtb.offset, dtb_gz_file, 0, entry.dtb.len);

            // gzip decompress
            const dtb_path = try std.fmt.allocPrint(allocator, "{s}.dtb", .{filename});
            defer allocator.free(dtb_path);
            const dtb_file = try std.fs.cwd().createFile(dtb_path, .{ .lock = .exclusive });
            defer dtb_file.close();

            try dtb_gz_file.seekTo(0);
            try std.compress.gzip.decompress(dtb_gz_file.reader(), dtb_file.writer());
        }

        if (entry.vrl) |vrl| {
            const vrl_path = try std.fmt.allocPrint(allocator, "{s}_vrl.bin", .{filename});
            defer allocator.free(vrl_path);
            const vrl_file = try std.fs.cwd().createFile(vrl_path, .{ .lock = .exclusive });
            defer vrl_file.close();
            _ = try file.copyRangeAll(offset + vrl.offset, vrl_file, 0, vrl.len);
        }
    }
}

pub const std_options = std.Options{ .log_level = .info };
