use serde::{Deserialize, Serialize};

use crate::{AdbTransport, ManagerError, OperationPlan};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransactionStatus {
    Running,
    Complete,
    RolledBack,
    RollbackFailed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct JournalEntry {
    pub command: String,
    pub result: String,
    pub rollback_result: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransactionJournal {
    pub status: TransactionStatus,
    pub entries: Vec<JournalEntry>,
}

#[derive(Debug)]
pub struct ExecutionError {
    pub source: ManagerError,
    pub journal: TransactionJournal,
}

pub struct Executor;

impl Executor {
    /// Executes a confirmed plan and rolls completed steps back on failure.
    ///
    /// # Errors
    ///
    /// Returns the source error together with the retained transaction journal.
    pub async fn execute(
        transport: &dyn AdbTransport,
        plan: &OperationPlan,
    ) -> std::result::Result<TransactionJournal, ExecutionError> {
        let mut journal = TransactionJournal {
            status: TransactionStatus::Running,
            entries: Vec::new(),
        };
        for step in &plan.steps {
            match transport.shell(&step.apply).await {
                Ok(_) => journal.entries.push(JournalEntry {
                    command: step.apply.clone(),
                    result: "success".into(),
                    rollback_result: None,
                }),
                Err(source) => return Err(rollback(transport, plan, journal, source).await),
            }
        }
        for command in &plan.verification_commands {
            if let Err(source) = transport.shell(command).await {
                return Err(rollback(transport, plan, journal, source).await);
            }
        }
        journal.status = TransactionStatus::Complete;
        Ok(journal)
    }
}

async fn rollback(
    transport: &dyn AdbTransport,
    plan: &OperationPlan,
    mut journal: TransactionJournal,
    source: ManagerError,
) -> ExecutionError {
    let mut rollback_failed = false;
    for index in (0..journal.entries.len()).rev() {
        if let Some(command) = &plan.steps[index].rollback {
            let result = transport.shell(command).await;
            rollback_failed |= result.is_err();
            journal.entries[index].rollback_result =
                Some(if result.is_ok() { "success" } else { "failed" }.into());
        }
    }
    journal.status = if rollback_failed {
        TransactionStatus::RollbackFailed
    } else {
        TransactionStatus::RolledBack
    };
    ExecutionError { source, journal }
}
