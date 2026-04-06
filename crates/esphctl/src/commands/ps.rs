use anyhow::Result;
use std::path::PathBuf;

pub async fn ps(_config: Option<PathBuf>) -> Result<()> {
    // TODO: connect to esphd via gRPC UDS
    // TODO: call GetClusterNodes RPC
    // TODO: display cluster-wide node table
    println!("[esph] ps: not yet implemented");
    Ok(())
}
