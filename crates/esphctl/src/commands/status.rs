use anyhow::Result;
use std::path::PathBuf;

pub async fn status(_config: Option<PathBuf>) -> Result<()> {
    // TODO: connect to esphd via gRPC UDS
    // TODO: call GetStatus RPC
    // TODO: display local node status
    println!("[esph] status: not yet implemented");
    Ok(())
}
