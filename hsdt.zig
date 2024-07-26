// SPDX-License-Identifier: Unlicense
// Author: xtex <xtex@xtexx.eu.org>
//
// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <https://unlicense.org>

const std = @import("std");
const AnyReader = std.io.AnyReader;
const AnyWriter = std.io.AnyWriter;

pub const DT_TABLE_MAGIC = @as(u32, 0x54445348);
pub const DT_TABLE_VERSION = @as(u32, 1);

pub const DtTableHeader = struct {
    magic: u32 = DT_TABLE_MAGIC,
    version: u32 = DT_TABLE_VERSION,
    entry_count: u32,

    pub fn read(reader: AnyReader) !@This() {
        const magic = try reader.readInt(u32, .little);
        if (magic != DT_TABLE_MAGIC) {
            return error.InvalidHsdtMagic;
        }
        const version = try reader.readInt(u32, .little);
        if (version != DT_TABLE_VERSION) {
            return error.InvalidHsdtVersion;
        }
        const entry_count = try reader.readInt(u32, .little);
        return .{
            .magic = magic,
            .version = version,
            .entry_count = entry_count,
        };
    }
};

pub const DtEntry = struct {
    board_id: [4]u8,
    dtb: struct { offset: u32, len: u32 },
    /// VRL is for verifying the DTB
    vrl: ?struct { offset: u32, len: u32 },

    pub fn read(reader: AnyReader) !@This() {
        var board_id: [4]u8 = undefined;
        try reader.readNoEof(&board_id);
        // in my reverse project, the following four bytes are always zero
        // and are not used by the fastboot
        // maybe we can ignore this if not zero,
        // but the error is used as a notice
        if (try reader.readInt(u32, .little) != 0) {
            return error.NonZeroUnknown1;
        }
        const dtb_len = try reader.readInt(u32, .little);
        const vrl_len = try reader.readInt(u32, .little);
        const dtb_offset = try reader.readInt(u32, .little);
        const vrl_offset = try reader.readInt(u32, .little);
        // the following 16 bytes are for unknown use
        // they are not zero, but I did not see any usage in the fastboot
        try reader.skipBytes(16, .{});
        // the real gzip DTB has an extra offset of 4096 bytes
        return .{
            .board_id = board_id,
            .dtb = .{
                .offset = dtb_offset + 4096,
                .len = dtb_len,
            },
            .vrl = if (vrl_len == 0) null else .{
                .offset = vrl_offset + 4096,
                .len = vrl_len,
            },
        };
    }
};
