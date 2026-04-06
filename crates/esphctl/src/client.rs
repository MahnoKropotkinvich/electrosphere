/// gRPC client for connecting to the local esphd daemon.
///
/// Communicates via Unix domain socket at ~/.local/share/esph/esphd.sock.

pub struct DaemonClient {
    // TODO: tonic gRPC client channel over UDS
}

impl DaemonClient {
    /// Connect to the local esphd daemon via Unix socket.
    pub async fn connect(_socket_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: implement UDS connection via tonic + tower
        Ok(Self {})
    }
}
