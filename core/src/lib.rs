#![forbid(unsafe_code)]

mod address;
mod device;
mod error;

pub use address::TargetAddress;
pub use device::{DeviceFacts, DeviceIdentity, PackageState, SupportLevel};
pub use error::{ManagerError, Result};

/// Shared safety engine for Universal Fire TV Manager clients.
pub const PRODUCT_NAME: &str = "Universal Fire TV Manager";
