use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Top-level configuration, loaded from config.toml.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EsphConfig {
    #[serde(default)]
    pub cluster: ClusterConfig,
    #[serde(default)]
    pub daemon: DaemonConfig,
    #[serde(default)]
    pub filesystem: FilesystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterConfig {
    /// Cluster name.
    #[serde(default)]
    pub name: String,
    /// Node role: "master" or "worker".
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// TCP port for inter-node gRPC communication.
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            listen_port: default_listen_port(),
        }
    }
}

fn default_listen_port() -> u16 {
    9090
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilesystemConfig {
    #[serde(default = "default_fs_backend")]
    pub backend: String,
    #[serde(default = "default_mount_point")]
    pub mount_point: String,
    #[serde(default)]
    pub juicefs: JuiceFSConfig,
}

fn default_fs_backend() -> String {
    "juicefs".to_string()
}

fn default_mount_point() -> String {
    "/mnt/esph".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JuiceFSConfig {
    #[serde(default)]
    pub meta_url: String,
    #[serde(default)]
    pub fs_name: String,
}

/// Standard paths following XDG conventions.
pub struct EsphPaths {
    /// ~/.config/esph/
    pub config_dir: PathBuf,
    /// ~/.local/share/esph/
    pub data_dir: PathBuf,
}

impl EsphPaths {
    /// Resolve XDG-compliant paths, respecting `XDG_CONFIG_HOME` and `XDG_DATA_HOME`.
    pub fn resolve() -> Self {
        let config_dir = std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("esph");

        let data_dir = std::env::var("XDG_DATA_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".local").join("share")))
            .unwrap_or_else(|| PathBuf::from(".local/share"))
            .join("esph");

        Self {
            config_dir,
            data_dir,
        }
    }

    /// Path to config.toml
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Path to the RocksDB directory
    pub fn db_dir(&self) -> PathBuf {
        self.data_dir.join("db")
    }

    /// Path to the daemon Unix socket
    pub fn daemon_socket(&self) -> PathBuf {
        self.data_dir.join("esphd.sock")
    }

    /// Path to the daemon PID file
    pub fn daemon_pid_file(&self) -> PathBuf {
        self.data_dir.join("esphd.pid")
    }
}

impl EsphConfig {
    /// Load configuration from a TOML file.
    pub fn load(path: &Path) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            crate::error::EsphError::Config(format!("failed to read {}: {}", path.display(), e))
        })?;
        toml::from_str(&content).map_err(|e| {
            crate::error::EsphError::Config(format!("failed to parse {}: {}", path.display(), e))
        })
    }

    /// Create a default configuration.
    pub fn default_config() -> Self {
        Self {
            cluster: ClusterConfig::default(),
            daemon: DaemonConfig::default(),
            filesystem: FilesystemConfig::default(),
        }
    }
}
