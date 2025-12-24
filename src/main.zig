const std = @import("std");
const debug = std.debug;
const fs = std.fs;
const heap = std.heap;
const mem = std.mem;

const Config = struct {
    fn load(allocator: mem.Allocator) !void {
        const data_dir = try fs.getAppDataDir(allocator, "ped");
        defer allocator.free(data_dir);
        fs.makeDirAbsolute(data_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };
        const config_path = try fs.path.join(allocator, &[_][]const u8{ data_dir, "config.json" });
        defer allocator.free(config_path);
        const file = try fs.cwd().createFile(config_path, .{});
        defer file.close();
        var file_buf: [256]u8 = undefined;
        var reader = file.reader(&file_buf);
        const content = try reader.interface.allocRemaining(allocator, .unlimited);
        debug.print("got {s}\n", .{content});
    }
};

const Pet = struct {
    name: []const u8,
};

pub fn main() !void {
    const allocator = heap.page_allocator;
    try Config.load(allocator);
}
