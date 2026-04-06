use thiserror::Error;

#[derive(Error, Debug)]
pub enum EsphError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("network error: {0}")]
    Network(String),

    #[error("filesystem error: {0}")]
    Filesystem(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("daemon not running")]
    DaemonNotRunning,

    #[error("not a master node")]
    NotMaster,

    #[error("node not found: {0}")]
    NodeNotFound(String),

    #[error("io error: {0}")]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, EsphError>;
