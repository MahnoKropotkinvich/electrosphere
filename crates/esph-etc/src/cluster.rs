use esph_core::context::ClusterContext;

use crate::store::{EsphStore, Result};

const CLUSTER_CONTEXT_KEY: &str = "cluster/context";

impl EsphStore {
    /// Load the cluster context from RocksDB (master only).
    pub fn load_cluster_context(&self) -> Result<Option<ClusterContext>> {
        self.get_json(CLUSTER_CONTEXT_KEY)
    }

    /// Save the cluster context to RocksDB (master only).
    pub fn save_cluster_context(&self, ctx: &ClusterContext) -> Result<()> {
        self.put_json(CLUSTER_CONTEXT_KEY, ctx)
    }
}
