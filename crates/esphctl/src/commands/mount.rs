use anyhow::Result;
use std::path::PathBuf;

pub async fn mount(_config: Option<PathBuf>) -> Result<()> {
    // TODO: connect to esphd via gRPC UDS
    // TODO: call Mount RPC
    println!("[esph] mount: not yet implemented");
    Ok(())
}

pub async fn umount(_config: Option<PathBuf>) -> Result<()> {
    // TODO: connect to esphd via gRPC UDS
    // TODO: call Unmount RPC
    println!("[esph] umount: not yet implemented");
    Ok(())
}
