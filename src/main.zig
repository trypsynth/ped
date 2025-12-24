const std = @import("std");

const Pet = struct {
    name: []const u8,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const data_dir = try std.fs.getAppDataDir(allocator, "ped");
    std.debug.print("{s}\n", .{data_dir});
}
