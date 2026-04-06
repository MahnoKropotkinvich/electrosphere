use esph_core::context::LocalContext;

use crate::store::{EsphStore, Result};

const LOCAL_CONTEXT_KEY: &str = "local/context";

impl EsphStore {
    /// Load the local context from RocksDB.
    pub fn load_local_context(&self) -> Result<Option<LocalContext>> {
        self.get_json(LOCAL_CONTEXT_KEY)
    }

    /// Save the local context to RocksDB.
    pub fn save_local_context(&self, ctx: &LocalContext) -> Result<()> {
        self.put_json(LOCAL_CONTEXT_KEY, ctx)
    }
}
