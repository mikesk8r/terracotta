use std::sync::Arc;

use tokio::sync::Mutex;

mod config;
mod logs;
mod server;
mod world;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config()?;
    logs::setup_logger()?;
    let server = Arc::new(Mutex::new(server::ServerState::default()));
    let server_clone = server.clone();
    tokio::spawn(async move {
        let _ = server::start(config, &server_clone).await;
    });
    tokio::spawn(async move {
        world::begin(&server).await;
    })
    .await?;
    Ok(())
}
