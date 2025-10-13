mod legacy;

#[derive(Debug, Default)]
pub struct ServerConfig {
    pub max_players: u32,
    pub motd: String,
    pub server_address: String,
    pub server_port: u16,
}

impl ServerConfig {
    pub fn from_legacy(legacy_config: legacy::LegacyConfig) -> Self {
        let mut config = ServerConfig::default();

        config.max_players = legacy_config.max_players;
        config.server_address = legacy_config.server_ip;
        config.server_port = legacy_config.server_port;

        config
    }
}

pub fn get_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut config = ServerConfig::default();

    let raw_config = std::fs::read_to_string("./terracotta.toml");
    let legacy_config = std::fs::read_to_string("./server.properties");
    if raw_config.is_ok() {
        let raw_config = raw_config?;
        let parsed = toml::from_str::<toml::value::Table>(raw_config.as_str())?;

        let max_players = parsed.get("max_players");
        if max_players.is_some() {
            config.max_players = max_players.unwrap().as_integer().unwrap() as u32;
        } else {
            config.max_players = 20;
        }

        let motd = parsed.get("motd");
        if motd.is_some() {
            config.motd = motd.unwrap().to_string();
        } else {
            config.motd = "A Minecraft server".to_string();
        }

        let server_address = parsed.get("server_address");
        if server_address.is_some() {
            config.server_address = server_address.unwrap().to_string();
        } else {
            config.server_address = "127.0.0.1".to_string();
        }

        let server_port = parsed.get("server_port");
        if server_port.is_some() {
            config.server_port = server_port.unwrap().as_integer().unwrap() as u16;
        } else {
            config.server_port = 25565;
        }
    } else if legacy_config.is_ok() {
        let legacy_config = legacy_config?;
        let parsed = legacy::get_legacy_config(legacy_config)?;
        config = ServerConfig::from_legacy(parsed);
    } else {
        config = ServerConfig {
            max_players: 20,
            motd: "A Minecraft server".to_string(),
            server_address: "127.0.0.1".to_string(),
            server_port: 25565,
        };
    }

    log::debug!("{:?}", &config);

    Ok(config)
}
