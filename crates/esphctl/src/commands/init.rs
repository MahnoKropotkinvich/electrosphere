use anyhow::Result;
use std::path::PathBuf;

pub async fn init_master(_config: Option<PathBuf>) -> Result<()> {
    println!("[esph] Initializing master node...");
    // TODO: create XDG directories
    // TODO: initialize RocksDB
    // TODO: generate node_id
    // TODO: connect to Headscale
    // TODO: save LocalContext + ClusterContext
    // TODO: start esphd daemon
    // TODO: output join command
    println!("[esph] Master node ready.");
    println!();
    println!("To add a worker node, run the following command on the target machine:");
    println!();
    println!("  esphctl init --worker --token <TOKEN> --master <ADDR>");
    Ok(())
}

pub async fn init_worker(
    _config: Option<PathBuf>,
    _token: String,
    _master_addr: String,
) -> Result<()> {
    println!("[esph] Initializing worker node...");
    // TODO: create XDG directories
    // TODO: initialize RocksDB
    // TODO: decode join token
    // TODO: join Headscale network
    // TODO: save LocalContext
    // TODO: start esphd daemon
    println!("[esph] Worker node ready.");
    Ok(())
}
