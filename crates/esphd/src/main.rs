mod context;
mod master;
mod server;
mod worker;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("esphd starting...");

    // TODO: parse arguments (config path, foreground mode, etc.)
    // TODO: load context from RocksDB
    // TODO: start gRPC server on UDS
    // TODO: start periodic context flush
    // TODO: start worker/master services based on role
    // TODO: await shutdown signal

    tracing::info!("esphd stopped.");
    Ok(())
}
