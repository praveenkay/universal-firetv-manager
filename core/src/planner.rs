use crate::{
    CriticalGuards, DeviceFacts, GuardDecision, ManagerError, Mutation, Operation, OperationPlan,
    PlanStep, Result, Selection, VerifiedBackup,
};

pub struct Planner;

impl Planner {
    /// Creates a guarded operation plan tied to a verified baseline backup.
    ///
    /// # Errors
    ///
    /// Returns an error when the backup is missing/mismatched or a safety guard denies a mutation.
    pub fn plan(
        operation: Operation,
        mutations: &[Mutation],
        facts: &DeviceFacts,
        _selection: &Selection,
        backup: Option<&VerifiedBackup>,
    ) -> Result<OperationPlan> {
        let backup =
            backup.ok_or_else(|| ManagerError::InvalidData("baseline backup required".into()))?;
        if backup.manifest.identity != facts.identity {
            return Err(ManagerError::DeviceMismatch);
        }
        let mut steps = Vec::new();
        for mutation in mutations {
            if let GuardDecision::Denied(reason) = CriticalGuards::allows(mutation, facts) {
                return Err(ManagerError::Unsupported(reason));
            }
            steps.push(step_for(mutation));
        }
        Ok(OperationPlan {
            operation,
            backup_identity: facts.identity.clone(),
            steps,
            verification_commands: Vec::new(),
        })
    }

    /// Creates an exact settings restore plan from a matching backup.
    ///
    /// # Errors
    ///
    /// Returns an error if the backup belongs to a different device or contains malformed setting keys.
    pub fn restore(backup: &VerifiedBackup, facts: &DeviceFacts) -> Result<OperationPlan> {
        if backup.manifest.identity != facts.identity {
            return Err(ManagerError::DeviceMismatch);
        }
        let mut steps = Vec::new();
        for (path, value) in &backup.manifest.settings {
            let (namespace, key) = path.split_once('/').ok_or_else(|| {
                ManagerError::InvalidData(format!("invalid setting path: {path}"))
            })?;
            let apply = value.as_ref().map_or_else(
                || format!("settings delete {namespace} {key}"),
                |value| format!("settings put {namespace} {key} {value}"),
            );
            steps.push(PlanStep {
                mutation: Mutation::SetSetting {
                    namespace: namespace.into(),
                    key: key.into(),
                    value: value.clone().unwrap_or_default(),
                },
                apply,
                rollback: None,
            });
        }
        Ok(OperationPlan {
            operation: Operation::Restore,
            backup_identity: facts.identity.clone(),
            steps,
            verification_commands: Vec::new(),
        })
    }
}

fn step_for(mutation: &Mutation) -> PlanStep {
    match mutation {
        Mutation::DisablePackage { package } => PlanStep::disable(package),
        Mutation::EnablePackage { package } => PlanStep {
            mutation: mutation.clone(),
            apply: format!("pm enable --user 0 {package}"),
            rollback: Some(format!("pm disable-user --user 0 {package}")),
        },
        Mutation::SetSetting {
            namespace,
            key,
            value,
        } => PlanStep {
            mutation: mutation.clone(),
            apply: format!("settings put {namespace} {key} {value}"),
            rollback: None,
        },
    }
}
