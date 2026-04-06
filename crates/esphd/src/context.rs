/// Context manager for esphd.
///
/// Holds in-memory LocalContext (and ClusterContext for master),
/// handles periodic async flush and forced flush on shutdown.

pub struct ContextManager {
    // TODO: EsphStore, RwLock<LocalContext>, Option<RwLock<ClusterContext>>
    // TODO: flush interval configuration
}
