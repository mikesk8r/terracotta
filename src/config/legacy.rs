#![allow(dead_code)]

#[derive(Default)]
pub struct LegacyConfig {
    pub accepts_transfers: bool,
    pub allow_flight: bool,
    pub broadcast_console_ops: bool,
    pub broadcast_rcon_ops: bool,
    pub bug_report_link: String,
    pub difficulty: crate::world::Difficulty,
    pub enable_coc: bool,
    pub enable_jmx_monitor: bool,
    pub enable_query: bool,
    pub enable_rcon: bool,
    pub enable_status: bool,
    pub enforce_secure_profile: bool,
    pub enforce_whitelist: bool,
    pub entity_broadcast_range: u8,
    pub force_gamemode: bool,
    pub function_permission_level: u8,
    pub gamemode: crate::world::Gamemode,
    pub generate_structures: bool,
    // TODO: add generator settings here
    pub hardcore: bool,
    pub hide_online_players: bool,
    // TODO: add initial-disabled-packs and initial-enabled-packs
    pub level_name: String,
    pub level_seed: String,
    pub level_type: String,
    pub log_ips: bool,
    pub management_server_enabled: bool,
    pub management_server_host: String,
    pub management_server_port: u16,
    pub management_server_secret: String,
    pub management_server_tls_enabled: bool,
    // TODO: management-server-tls-keystore and its -password
    pub max_chained_neighbor_updates: u32,
    pub max_players: u32,
    pub max_tick_time: u32,
    pub max_world_size: u32,
    pub motd: String,
    pub network_compression_threshold: u16,
    pub online_mode: bool,
    pub op_permission_level: u8,
    pub pause_empty_seeconds: u16,
    pub player_idle_timeout: u16,
    pub prevent_proxy_connections: bool,
    pub query_port: u16,
    pub rate_limit: u16,
    pub rcon_password: String,
    pub rcon_port: u16,
    pub region_file_compression: String,
    pub require_resource_pack: bool,
    // TODO: add resource packs
    pub server_ip: String,
    pub server_port: u16,
    pub simulation_distance: u8,
    pub spawn_protection: u8,
    pub status_heartbeat_interval: u8,
    pub sync_chunk_writes: bool,
    pub view_distance: u8,
    pub whitelist: bool,
}

pub fn get_legacy_config(legacy_config: String) -> LegacyConfig {
    let mut config = LegacyConfig::default();

    for line in legacy_config.lines() {
        if line.starts_with("#") {
            continue;
        }
        let line_split: Vec<&str> = line.trim().split_terminator("=").collect();
        match line_split[0] {
            "max-players" => {
                config.max_players = line_split[1].parse().unwrap();
            }
            "server-ip" => {
                config.server_ip = line_split[1].to_string();
            }
            "server-port" => {
                config.server_port = line_split[1].parse().unwrap();
            }
            _ => {
                log::debug!("Unused value {}!", line_split[0]);
                continue;
            }
        }
    }

    config
}
