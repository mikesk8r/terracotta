const std = @import("std");

const Server = @import("Server.zig");

pub fn main(init: std.process.Init) !void {
    try Server.start(init.io);
}
