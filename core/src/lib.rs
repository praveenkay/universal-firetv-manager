#![forbid(unsafe_code)]

pub mod adb;
mod address;
mod backup;
mod device;
mod error;
mod guards;
mod inspect;
mod profile;
mod redaction;

pub use adb::{AdbTransport, ConnectionState, MockAdbTransport, RsAdbTransport, ShellOutput};
pub use address::TargetAddress;
pub use backup::{BackupManifest, BackupOptions, VerifiedBackup, capture_backup, verify_backup};
pub use device::{DeviceFacts, DeviceIdentity, PackageState, SupportLevel};
pub use error::{ManagerError, Result};
pub use guards::{CriticalGuards, GuardDecision, Mutation};
pub use inspect::inspect_device;
pub use profile::{Profile, ProfileStatus, ProfileStore, Selection};
pub use redaction::export_redacted_report;

/// Shared safety engine for Universal Fire TV Manager clients.
pub const PRODUCT_NAME: &str = "Universal Fire TV Manager";
