use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "esphctl")]
#[command(about = "Electrosphere cluster management CLI")]
#[command(version)]
pub struct CLI {
    /// Path to configuration file
    #[arg(short = 'c', long = "config", global = true)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
pub enum Commands {
    /// Initialize this node (first-time setup)
    init {
        /// Initialize as master node
        #[arg(long, conflicts_with_all = ["worker"])]
        master: bool,

        /// Initialize as worker node
        #[arg(long, conflicts_with_all = ["master"])]
        worker: bool,

        /// Join token (required for --worker)
        #[arg(long, required_if_eq("worker", "true"))]
        token: Option<String>,

        /// Master address (required for --worker)
        #[arg(long, required_if_eq("worker", "true"))]
        master_addr: Option<String>,
    },

    /// Start the esphd daemon
    start,

    /// Stop the esphd daemon
    stop,

    /// Restart the esphd daemon
    restart,

    /// Show local node status
    status,

    /// Show cluster-wide node status
    ps,

    /// Mount the distributed filesystem
    mount,

    /// Unmount the distributed filesystem
    #[command(alias = "unmount")]
    umount,
}
