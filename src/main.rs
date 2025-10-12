mod logs;
mod server;
mod world;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logs::setup_logger()?;
    server::start().await?;
    Ok(())
}
