use crate::{
    AdbTransport, DeviceFacts, Executor, Planner, Result, TransactionJournal, VerifiedBackup,
};

/// Restores settings from a verified backup belonging to the connected device.
///
/// # Errors
///
/// Returns an error for identity mismatch, malformed backup state, or failed execution/rollback.
pub async fn restore_backup(
    transport: &dyn AdbTransport,
    backup: &VerifiedBackup,
    facts: &DeviceFacts,
) -> Result<TransactionJournal> {
    let plan = Planner::restore(backup, facts)?;
    Executor::execute(transport, &plan)
        .await
        .map_err(|error| error.source)
}
