use std::{
    collections::BTreeMap,
    path::Path,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde::Deserialize;

use super::{AdbTransport, ConnectionState, ShellOutput};
use crate::{ManagerError, Result, TargetAddress};

#[derive(Deserialize)]
struct Fixture {
    state: ConnectionState,
    shell: BTreeMap<String, ShellOutput>,
}

#[derive(Clone)]
pub struct MockAdbTransport {
    state: ConnectionState,
    shell: Arc<Mutex<BTreeMap<String, ShellOutput>>>,
    shell_calls: Arc<Mutex<Vec<String>>>,
}

impl MockAdbTransport {
    #[must_use]
    pub fn with_state(state: ConnectionState) -> Self {
        Self {
            state,
            shell: Arc::new(Mutex::new(BTreeMap::new())),
            shell_calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Builds a mock transport from a JSON fixture.
    ///
    /// # Errors
    ///
    /// Returns an error when the fixture is not valid JSON or has the wrong shape.
    pub fn from_fixture(json: &str) -> Result<Self> {
        let fixture: Fixture = serde_json::from_str(json)
            .map_err(|error| ManagerError::InvalidData(error.to_string()))?;
        Ok(Self {
            state: fixture.state,
            shell: Arc::new(Mutex::new(fixture.shell)),
            shell_calls: Arc::new(Mutex::new(Vec::new())),
        })
    }

    #[must_use]
    pub fn shell_call_count(&self) -> usize {
        self.shell_calls
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .len()
    }

    pub fn set_shell_response(&self, command: &str, stdout: &str, stderr: &str, exit_code: i32) {
        self.shell
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(
                command.to_owned(),
                ShellOutput {
                    stdout: stdout.to_owned(),
                    stderr: stderr.to_owned(),
                    exit_code: Some(exit_code),
                },
            );
    }

    pub fn fail_shell_command(&self, command: &str, exit_code: i32, stderr: &str) {
        self.shell
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(
                command.to_owned(),
                ShellOutput {
                    stdout: String::new(),
                    stderr: stderr.to_owned(),
                    exit_code: Some(exit_code),
                },
            );
    }
}

#[async_trait]
impl AdbTransport for MockAdbTransport {
    async fn connect(&self, _target: TargetAddress) -> Result<ConnectionState> {
        Ok(self.state)
    }

    async fn shell(&self, command: &str) -> Result<ShellOutput> {
        self.shell_calls
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push(command.to_owned());
        let output = self
            .shell
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(command)
            .cloned()
            .ok_or_else(|| ManagerError::CommandFailed(format!("no fixture for {command}")))?;
        if output.success() {
            Ok(output)
        } else {
            Err(ManagerError::CommandFailed(format!(
                "{command}: {}",
                output.stderr.trim()
            )))
        }
    }

    async fn push(&self, _local: &Path, _remote: &str) -> Result<()> {
        Ok(())
    }

    async fn pull(&self, _remote: &str, _local: &Path) -> Result<()> {
        Ok(())
    }

    async fn install(&self, _apk: &Path) -> Result<()> {
        Ok(())
    }

    async fn uninstall(&self, _package: &str) -> Result<()> {
        Ok(())
    }
}
