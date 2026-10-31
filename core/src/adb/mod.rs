mod mock;
mod rsadb_transport;

use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{Result, TargetAddress};

pub use mock::MockAdbTransport;
pub use rsadb_transport::RsAdbTransport;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionState {
    Connected,
    Unauthorized,
    Offline,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ShellOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl ShellOutput {
    #[must_use]
    pub fn success(&self) -> bool {
        self.exit_code.is_none_or(|code| code == 0)
    }
}

#[async_trait]
pub trait AdbTransport: Send + Sync {
    async fn connect(&self, target: TargetAddress) -> Result<ConnectionState>;
    async fn shell(&self, command: &str) -> Result<ShellOutput>;
    async fn push(&self, local: &Path, remote: &str) -> Result<()>;
    async fn pull(&self, remote: &str, local: &Path) -> Result<()>;
    async fn install(&self, apk: &Path) -> Result<()>;
    async fn uninstall(&self, package: &str) -> Result<()>;
}
