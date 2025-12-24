const std = @import("std");
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
    }
};

const Pet = struct {
    name: []const u8,
};

pub fn main() !void {
    const allocator = heap.page_allocator;
    try Config.load(allocator);
}
