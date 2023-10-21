use color_eyre::Result;
use site::run_server;

#[tokio::main]
async fn main() -> Result<()> {
    run_server().await?;
    Ok(())
}
