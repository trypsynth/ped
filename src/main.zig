const std = @import("std");

const Pet = struct {
    name: []const u8,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const data_dir = try std.fs.getAppDataDir(allocator, "ped");
    defer allocator.free(data_dir);
    const config_path = try std.fs.path.join(allocator, &[_][]const u8{ data_dir, "config.json" });
    std.debug.print("{s}\n", .{config_path});
}
