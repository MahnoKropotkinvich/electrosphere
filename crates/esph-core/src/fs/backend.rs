use async_trait::async_trait;
use std::path::Path;

use crate::error::Result;

/// Status of the filesystem.
#[derive(Debug, Clone)]
pub struct FSStatus {
    pub mounted: bool,
    pub mount_point: Option<String>,
    pub backend: String,
    pub usage_bytes: Option<u64>,
    pub total_bytes: Option<u64>,
}

/// Abstract filesystem backend.
#[async_trait]
pub trait FSBackend: Send + Sync {
    /// Mount the filesystem at the given path.
    async fn mount(&self, mount_point: &Path) -> Result<()>;

    /// Unmount the filesystem.
    async fn unmount(&self, mount_point: &Path) -> Result<()>;

    /// Get filesystem status.
    async fn status(&self) -> Result<FSStatus>;
}
