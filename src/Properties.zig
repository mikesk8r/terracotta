//! server.properties

const Properties = @This();

@"accepts-transfers": bool,
@"allow-flight": bool,
difficulty: enum {
    peaceful,
    easy,
    normal,
    hard,
},
gamemode: enum {
    survival,
    creative,
    spectator,
    adventure,
},
@"server-port": u16,

fn fromBytes(data: []const u8) !Properties {
    const to_return = Properties {};

    for (data) |char| {
        // in a comment and should skip to next line
        var skip = false;
        // should we write to the value buffer?
        var write_to_value = false;
        var key_buffer: []u8 = .{};
        var value_buffer: []u8 = .{};

        if (char == '\n') {
            skip = false;
            const field = @field(to_return, key_buffer);
            field = @as(@TypeOf(field), value_buffer);

            continue;
        }

        if (skip) {
            continue;
        }

        // are we reading a comment?
        if (char == '#') {
            skip = true;
            continue;
        }

        if (char == '=') {
            write_to_value = true;
            continue;
        }

        if (char != ' ') {
            if (write_to_value) {
                value_buffer += char;
            } else {
                key_buffer += char;
            }
        }
    }
}
