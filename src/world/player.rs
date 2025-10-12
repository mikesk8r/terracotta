#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ConnectionState {
    #[default]
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
}

#[derive(Clone, Default)]
pub struct Player {
    pub name: String,
    pub uuid: [u8; 16],
    pub state: ConnectionState,
    pub entity_id: u32,
}
