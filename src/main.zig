const std = @import("std");

const Config = struct {
    name: ?[]const u8 = null,

    fn load(allocator: std.mem.Allocator) !Config {
        const config_path = try getConfigPath(allocator);
        defer allocator.free(config_path);
        const file = std.fs.openFileAbsolute(config_path, .{ .mode = .read_only }) catch |err| switch (err) {
            error.FileNotFound => return .{},
            else => return err,
        };
        defer file.close();
        var file_buf: [256]u8 = undefined;
        var reader = file.reader(&file_buf);
        const content = try reader.interface.allocRemaining(allocator, .unlimited);
        defer allocator.free(content);
        if (content.len == 0) return .{};
        const parsed = try std.json.parseFromSlice(Config, allocator, content, .{});
        defer parsed.deinit();
        var config = parsed.value;
        if (config.name) |name| {
            config.name = try allocator.dupe(u8, name);
        }
        return config;
    }

    fn save(self: Config, allocator: std.mem.Allocator) !void {
        const config_path = try getConfigPath(allocator);
        defer allocator.free(config_path);
        const file = try std.fs.createFileAbsolute(config_path, .{ .truncate = true });
        defer file.close();
        var writer_buf: [256]u8 = undefined;
        var writer = file.writer(&writer_buf);
        try std.json.Stringify.value(self, .{}, &writer.interface);
        try writer.interface.writeByte('\n');
        try writer.interface.flush();
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

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var config = try Config.load(allocator);
    defer if (config.name) |name| allocator.free(name);
    if (config.name) |name| {
        var stdout_buf: [256]u8 = undefined;
        var stdout = std.fs.File.stdout().writer(&stdout_buf);
        try stdout.interface.print("{s}\n", .{name});
        try stdout.interface.flush();
        return;
    }
    const name = try promptForName(allocator);
    defer allocator.free(name);
    config.name = name;
    try config.save(allocator);
}

fn promptForName(allocator: std.mem.Allocator) ![]u8 {
    var stdin_buf: [64]u8 = undefined;
    var stdout_buf: [256]u8 = undefined;
    var stdin = std.fs.File.stdin().reader(&stdin_buf);
    var stdout = std.fs.File.stdout().writer(&stdout_buf);
    try stdout.interface.print("Name: ", .{});
    try stdout.interface.flush();
    const line_opt = try stdin.interface.takeDelimiter('\n');
    if (line_opt == null) return error.EndOfStream;
    const line = line_opt.?;
    const trimmed = std.mem.trim(u8, line, " \t\r\n");
    if (trimmed.len == 0) return error.EmptyName;
    return try allocator.dupe(u8, trimmed);
}
