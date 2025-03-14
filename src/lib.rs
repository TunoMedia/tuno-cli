pub mod config;
pub mod server;

use anyhow::Result;

pub async fn run() -> Result<()> {
    let config = config::load_config()?;

    let server = server::TunoServer::new(
        config.server.host,
        config.server.port,
        config.server.cert_dir
    );

    server.run().await?;
    
    Ok(())
}