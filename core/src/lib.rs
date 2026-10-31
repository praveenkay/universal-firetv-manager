#![forbid(unsafe_code)]

pub mod adb;
mod address;
mod backup;
mod device;
mod error;
mod guards;
mod inspect;
mod operation;
mod planner;
mod profile;
mod redaction;
mod restore;
mod transaction;

pub use adb::{AdbTransport, ConnectionState, MockAdbTransport, RsAdbTransport, ShellOutput};
pub use address::TargetAddress;
pub use backup::{BackupManifest, BackupOptions, VerifiedBackup, capture_backup, verify_backup};
pub use device::{DeviceFacts, DeviceIdentity, PackageState, SupportLevel};
pub use error::{ManagerError, Result};
pub use guards::{CriticalGuards, GuardDecision, Mutation};
pub use inspect::inspect_device;
pub use operation::{Operation, OperationPlan, PlanStep};
pub use planner::Planner;
pub use profile::{Profile, ProfileStatus, ProfileStore, Selection};
pub use redaction::export_redacted_report;
pub use restore::restore_backup;
pub use transaction::{
    ExecutionError, Executor, JournalEntry, TransactionJournal, TransactionStatus,
};

/// Shared safety engine for Universal Fire TV Manager clients.
pub const PRODUCT_NAME: &str = "Universal Fire TV Manager";
