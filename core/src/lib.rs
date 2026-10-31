#![forbid(unsafe_code)]

pub mod adb;
mod address;
mod device;
mod error;
mod inspect;

pub use adb::{AdbTransport, ConnectionState, MockAdbTransport, RsAdbTransport, ShellOutput};
pub use address::TargetAddress;
pub use device::{DeviceFacts, DeviceIdentity, PackageState, SupportLevel};
pub use error::{ManagerError, Result};
pub use inspect::inspect_device;

/// Shared safety engine for Universal Fire TV Manager clients.
pub const PRODUCT_NAME: &str = "Universal Fire TV Manager";
