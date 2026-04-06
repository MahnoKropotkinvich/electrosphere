use serde::{Deserialize, Serialize};
use std::fmt;

/// Node role in the cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeRole {
    Master,
    Worker,
}

impl fmt::Display for NodeRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeRole::Master => write!(f, "master"),
            NodeRole::Worker => write!(f, "worker"),
        }
    }
}

/// CPU architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Arch {
    aarch64,
    x86_64,
    riscv64,
}

impl Arch {
    /// Detect the architecture of the current machine.
    pub fn current() -> Self {
        match std::env::consts::ARCH {
            "aarch64" => Arch::aarch64,
            "x86_64" => Arch::x86_64,
            "riscv64" => Arch::riscv64,
            _ => Arch::x86_64, // fallback
        }
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arch::aarch64 => write!(f, "aarch64"),
            Arch::x86_64 => write!(f, "x86_64"),
            Arch::riscv64 => write!(f, "riscv64"),
        }
    }
}

/// Operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum OS {
    Linux,
    macOS,
    FreeBSD,
}

impl OS {
    /// Detect the OS of the current machine.
    pub fn current() -> Self {
        match std::env::consts::OS {
            "linux" => OS::Linux,
            "macos" => OS::macOS,
            "freebsd" => OS::FreeBSD,
            _ => OS::Linux, // fallback
        }
    }
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OS::Linux => write!(f, "linux"),
            OS::macOS => write!(f, "macos"),
            OS::FreeBSD => write!(f, "freebsd"),
        }
    }
}

/// Information about a node in the cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub node_id: String,
    pub name: String,
    pub role: NodeRole,
    pub arch: Arch,
    pub os: OS,
    pub tags: Vec<String>,
    pub online: bool,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}
