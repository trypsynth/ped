const std = @import("std");

const Config = struct {
    fn load(allocator: std.mem.Allocator) !void {
        const config_path = try getConfigPath(allocator);
        defer allocator.free(config_path);
        const file = try std.fs.cwd().createFile(config_path, .{});
        defer file.close();
        var file_buf: [256]u8 = undefined;
        var reader = file.reader(&file_buf);
        const content = try reader.interface.allocRemaining(allocator, .unlimited);
        std.debug.print("got {s}\n", .{content});
    }

    fn getConfigPath(allocator: std.mem.Allocator) ![]const u8 {
        const data_dir = try std.fs.getAppDataDir(allocator, "ped");
        defer allocator.free(data_dir);
        std.fs.makeDirAbsolute(data_dir) catch |err| switch (err) {
            error.PathAlreadyExists => {},
            else => return err,
        };
        const path = try std.fs.path.join(allocator, &[_][]const u8{ data_dir, "config.json" });
        return path;
    }
};

const Pet = struct {
    name: []const u8,
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    try Config.load(allocator);
}
