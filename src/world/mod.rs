#![allow(dead_code)]

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
