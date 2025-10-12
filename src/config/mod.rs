#[derive(Debug, Default)]
pub struct ServerConfig {
    pub max_players: u32,
    pub server_address: String,
    pub server_port: u16,
}

pub fn get_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut config = ServerConfig::default();

    let raw_config = std::fs::read_to_string("./terracotta.toml");
    if raw_config.is_ok() {
        let raw_config = raw_config?;
        let parsed = toml::from_str::<toml::value::Table>(raw_config.as_str())?;

        let max_players = parsed.get("max_players");
        if max_players.is_some() {
            config.max_players = max_players.unwrap().as_integer().unwrap() as u32;
        } else {
            config.max_players = 20;
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
    } else {
        config = ServerConfig {
            max_players: 20,
            server_address: "127.0.0.1".to_string(),
            server_port: 25565,
        };
    }

    log::debug!("{:?}", &config);

    Ok(config)
}
