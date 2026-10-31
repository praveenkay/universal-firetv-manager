use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use rsadb::{Device, Session, auth, transport};
use tokio::{sync::RwLock, time::timeout};

use super::{AdbTransport, ConnectionState, ShellOutput};
use crate::{ManagerError, Result, TargetAddress};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
const AUTH_WAIT: Duration = Duration::from_secs(60);
const COMMAND_TIMEOUT: Duration = Duration::from_secs(45);
const REMOTE_INSTALL_PATH: &str = "/data/local/tmp/universal-firetv-manager-install.apk";

pub struct RsAdbTransport {
    key_paths: auth::KeyPaths,
    device: RwLock<Option<Arc<Device<Session>>>>,
}

impl RsAdbTransport {
    #[must_use]
    pub fn new(key_directory: &Path) -> Self {
        Self {
            key_paths: auth::KeyPaths::in_dir(key_directory),
            device: RwLock::new(None),
        }
    }

    fn map_error(error: rsadb::Error) -> ManagerError {
        match error {
            rsadb::Error::Unauthorized => ManagerError::Unauthorized,
            rsadb::Error::Disconnected | rsadb::Error::NoDevice(_) => ManagerError::Offline,
            other => ManagerError::Transport(other.to_string()),
        }
    }

    async fn connected_device(&self) -> Result<Arc<Device<Session>>> {
        self.device
            .read()
            .await
            .clone()
            .ok_or(ManagerError::Offline)
    }

    async fn run_shell(&self, command: &str) -> Result<ShellOutput> {
        let device = self.connected_device().await?;
        let output = timeout(COMMAND_TIMEOUT, device.shell(command))
            .await
            .map_err(|_| ManagerError::Transport(format!("command timed out: {command}")))?
            .map_err(Self::map_error)?;
        let output = ShellOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.exit_code.map(i32::from),
        };
        if output.success() {
            Ok(output)
        } else {
            Err(ManagerError::CommandFailed(format!(
                "{command}: {}",
                output.stderr.trim()
            )))
        }
    }

    fn validate_package(package: &str) -> Result<()> {
        if package.is_empty()
            || !package
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'_')
        {
            return Err(ManagerError::InvalidData("invalid package name".into()));
        }
        Ok(())
    }
}

impl Default for RsAdbTransport {
    fn default() -> Self {
        let directory = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map_or_else(|| PathBuf::from("."), PathBuf::from)
            .join(".universal-firetv-manager");
        Self::new(&directory)
    }
}

#[async_trait]
impl AdbTransport for RsAdbTransport {
    async fn connect(&self, target: TargetAddress) -> Result<ConnectionState> {
        let key = auth::load_or_generate(&self.key_paths, "universal-firetv-manager")
            .map_err(Self::map_error)?;
        let connection = timeout(CONNECT_TIMEOUT + AUTH_WAIT, async {
            let tcp = transport::tcp::connect(target.socket_addr()).await?;
            Session::connect(tcp, &key, AUTH_WAIT).await
        })
        .await;
        match connection {
            Ok(Ok(session)) => {
                *self.device.write().await = Some(Arc::new(Device::new(session)));
                Ok(ConnectionState::Connected)
            }
            Ok(Err(rsadb::Error::Unauthorized)) => Ok(ConnectionState::Unauthorized),
            Ok(Err(
                rsadb::Error::Io(_)
                | rsadb::Error::Disconnected
                | rsadb::Error::NoDevice(_)
                | rsadb::Error::Timeout(_),
            ))
            | Err(_) => Ok(ConnectionState::Offline),
            Ok(Err(error)) => Err(Self::map_error(error)),
        }
    }

    async fn shell(&self, command: &str) -> Result<ShellOutput> {
        self.run_shell(command).await
    }

    async fn push(&self, local: &Path, remote: &str) -> Result<()> {
        let device = self.connected_device().await?;
        timeout(COMMAND_TIMEOUT, device.push_file(local, remote))
            .await
            .map_err(|_| ManagerError::Transport("push timed out".into()))?
            .map_err(Self::map_error)
    }

    async fn pull(&self, remote: &str, local: &Path) -> Result<()> {
        let device = self.connected_device().await?;
        timeout(COMMAND_TIMEOUT, device.pull_to_file(remote, local))
            .await
            .map_err(|_| ManagerError::Transport("pull timed out".into()))?
            .map(|_| ())
            .map_err(Self::map_error)
    }

    async fn install(&self, apk: &Path) -> Result<()> {
        self.push(apk, REMOTE_INSTALL_PATH).await?;
        let result = self
            .run_shell(&format!("pm install -r --user 0 {REMOTE_INSTALL_PATH}"))
            .await;
        let _ = self
            .run_shell(&format!("rm -f {REMOTE_INSTALL_PATH}"))
            .await;
        result.map(|_| ())
    }

    async fn uninstall(&self, package: &str) -> Result<()> {
        Self::validate_package(package)?;
        self.run_shell(&format!("pm uninstall --user 0 {package}"))
            .await
            .map(|_| ())
    }
}
