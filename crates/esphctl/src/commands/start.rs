use anyhow::Result;
use std::path::PathBuf;

pub async fn start(_config: Option<PathBuf>) -> Result<()> {
    println!("[esph] Starting esphd daemon...");
    // TODO: check if already running (PID file)
    // TODO: fork/exec esphd process, or instruct user to use systemd/launchd
    // TODO: wait for daemon to be ready (connect to UDS)
    println!("[esph] esphd daemon started.");
    Ok(())
}
