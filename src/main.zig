const std = @import("std");

const Config = struct {
    fn load(allocator: std.mem.Allocator) !void {
        const data_dir = try std.fs.getAppDataDir(allocator, "ped");
        defer allocator.free(data_dir);
        std.fs.makeDirAbsolute(data_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };
        const config_path = try std.fs.path.join(allocator, &[_][]const u8{ data_dir, "config.json" });
        defer allocator.free(config_path);
        const file = try std.fs.cwd().createFile(config_path, .{});
        defer file.close();
    }
};

const Pet = struct {
    name: []const u8,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    try Config.load(allocator);
}
