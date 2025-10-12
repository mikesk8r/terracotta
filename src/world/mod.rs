#![allow(dead_code)]

use std::sync::Arc;

use tokio::sync::Mutex;

pub mod player;

#[derive(Default)]
pub enum Difficulty {
    Peaceful,
    #[default]
    Easy,
    Normal,
    Hard,
}

#[derive(Default)]
pub enum Gamemode {
    #[default]
    Survival,
    Creative,
    Spectator,
    Adventure,
}

pub async fn begin(_server: &Arc<Mutex<crate::server::ServerState>>) {
    #[allow(unused_variables)]
    let mut tick: u128 = 0;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        tick += 1;
    }
}
