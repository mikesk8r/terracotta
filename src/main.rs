mod config;
mod logs;
mod server;
mod world;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::get_config()?;
    logs::setup_logger()?;
    server::start(config).await?;
    Ok(())
}
