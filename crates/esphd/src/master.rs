/// Master-only services.
///
/// - Headscale API interaction (node management, token issuance)
/// - Receive worker heartbeats, update ClusterContext
/// - Respond to ps (ListNodes) queries
pub struct MasterServices {
    // TODO: headscale client, heartbeat receiver, cluster state
}
