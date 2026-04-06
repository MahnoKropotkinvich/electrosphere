use anyhow::Result;
use std::path::PathBuf;

pub async fn stop(_config: Option<PathBuf>) -> Result<()> {
    println!("[esph] Stopping esphd daemon...");
    // TODO: connect to esphd via gRPC UDS
    // TODO: send Shutdown RPC (daemon will flush context + exit)
    // TODO: wait for process exit
    println!("[esph] esphd daemon stopped.");
    Ok(())
}
