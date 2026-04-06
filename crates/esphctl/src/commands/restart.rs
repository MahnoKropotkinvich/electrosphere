use anyhow::Result;
use std::path::PathBuf;

pub async fn restart(config: Option<PathBuf>) -> Result<()> {
    super::stop::stop(config.clone()).await?;
    super::start::start(config).await
}
