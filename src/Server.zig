const std = @import("std");

const log = std.log.scoped(.net);

const Server = @This();

// TODO:
//  - chunkloading
//  - dimensions
//  - time
//  - etc...

allocator: std.heap.DebugAllocator(.{}) = .init,
listener: std.Io.net.Server,
connected_clients: std.Io.Group = .init,
server_running: bool = true,

pub fn start(io: std.Io) !void {
    log.info("starting server", .{});
    const address = try std.Io.net.IpAddress.parse("127.0.0.1", 25565);
    var listener = try address.listen(io, .{});
    defer listener.deinit(io);

    var server = Server { .listener = listener };
    while (server.server_running) {
        const stream = server.listener.accept(io) catch |err| {
            std.log.err("couldn't accept socket: {}\n", .{ err });
            continue;
        };

        log.info("connecting to new client", .{});
        server.connected_clients.async(io, handleConnection, .{ stream, io });
    }
}

pub fn deinit(self: *Server) void {
    self.server_running = false;
    self.connected_clients.cancel(self.io);
}

fn handleConnection(stream: std.Io.net.Stream, io: std.Io) error{Canceled}!void {
    defer stream.close(io);

    log.info("connection from {}", .{ stream.socket.address });

    var writer = stream.writer(io, &.{});

    // TODO: implement java edition protocol
    // see https://minecraft.wiki/w/Java_Edition_protocol
    writer.interface.writeAll("Hello, world!") catch |err| {
        log.err("couldn't write to interface! {}", .{ err });
    };
}
