use crate::node::NodeRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Local context — stored on every node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalContext {
    pub node_id: String,
    pub role: NodeRole,
    pub cluster_name: String,
    /// Worker records master address; master records its own address.
    pub master_addr: String,
    /// Filesystem mount configuration, if set.
    pub fs_config: Option<FSMountConfig>,
    /// Whether the filesystem is currently mounted.
    pub fs_mounted: bool,
}

/// Filesystem mount configuration snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FSMountConfig {
    pub backend: String,
    pub mount_point: String,
    pub meta_url: String,
    pub fs_name: String,
}

/// Cluster context — stored only on the master node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterContext {
    /// node_id → NodeRecord
    pub nodes: HashMap<String, NodeRecord>,
    /// Issued join tokens.
    pub tokens: Vec<TokenRecord>,
}

impl ClusterContext {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            tokens: Vec::new(),
        }
    }
}

impl Default for ClusterContext {
    fn default() -> Self {
        Self::new()
    }
}

/// A record of a node in the cluster, stored in ClusterContext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRecord {
    pub node_id: String,
    pub name: String,
    pub role: NodeRole,
    pub arch: String,
    pub os: String,
    pub tags: Vec<String>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub online: bool,
}

/// A record of an issued join token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRecord {
    pub token: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub used_by: Option<String>,
}
