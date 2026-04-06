use async_trait::async_trait;
use esph_core::fs::backend::{FSBackend, FSStatus};
use std::path::Path;

/// JuiceFS filesystem backend.
///
/// Manages JuiceFS mount/unmount by invoking the `juicefs` CLI.
pub struct JuiceFSManager {
    pub meta_url: String,
    pub fs_name: String,
}

#[async_trait]
impl FSBackend for JuiceFSManager {
    async fn mount(&self, _mount_point: &Path) -> esph_core::error::Result<()> {
        // TODO: invoke `juicefs mount`
        Ok(())
    }

    async fn unmount(&self, _mount_point: &Path) -> esph_core::error::Result<()> {
        // TODO: invoke `juicefs umount`
        Ok(())
    }

    async fn status(&self) -> esph_core::error::Result<FSStatus> {
        // TODO: check mount status
        Ok(FSStatus {
            mounted: false,
            mount_point: None,
            backend: "juicefs".to_string(),
            usage_bytes: None,
            total_bytes: None,
        })
    }
}
