pub mod init;
pub mod mount;
pub mod ps;
pub mod restart;
pub mod start;
pub mod status;
pub mod stop;

use anyhow::Result;

use crate::cli::{CLI, Commands};

pub async fn execute(cli: CLI) -> Result<()> {
    match cli.command {
        Commands::init {
            master,
            worker,
            token,
            master_addr,
        } => {
            if master {
                init::init_master(cli.config).await
            } else if worker {
                init::init_worker(
                    cli.config,
                    token.expect("token required for worker init"),
                    master_addr.expect("master address required for worker init"),
                )
                .await
            } else {
                anyhow::bail!("specify --master or --worker")
            }
        }
        Commands::start => start::start(cli.config).await,
        Commands::stop => stop::stop(cli.config).await,
        Commands::restart => restart::restart(cli.config).await,
        Commands::status => status::status(cli.config).await,
        Commands::ps => ps::ps(cli.config).await,
        Commands::mount => mount::mount(cli.config).await,
        Commands::umount => mount::umount(cli.config).await,
    }
}
