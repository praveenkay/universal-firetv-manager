use thiserror::Error;

pub type Result<T> = std::result::Result<T, ManagerError>;

#[derive(Debug, Error)]
pub enum ManagerError {
    #[error("invalid Fire TV address: {0}")]
    InvalidTarget(String),
    #[error("device is waiting for ADB authorization")]
    Unauthorized,
    #[error("device is offline")]
    Offline,
    #[error("ADB transport error: {0}")]
    Transport(String),
    #[error("device command failed: {0}")]
    CommandFailed(String),
    #[error("invalid device data: {0}")]
    InvalidData(String),
    #[error("integrity verification failed: {0}")]
    Integrity(String),
    #[error("backup belongs to a different device")]
    DeviceMismatch,
    #[error("operation is unsupported: {0}")]
    Unsupported(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
