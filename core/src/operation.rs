use serde::{Deserialize, Serialize};

use crate::{DeviceIdentity, Mutation};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Operation {
    Install,
    Repair,
    Uninstall,
    Restore,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlanStep {
    pub mutation: Mutation,
    pub apply: String,
    pub rollback: Option<String>,
}

impl PlanStep {
    #[must_use]
    pub fn disable(package: &str) -> Self {
        Self {
            mutation: Mutation::DisablePackage {
                package: package.into(),
            },
            apply: format!("pm disable-user --user 0 {package}"),
            rollback: Some(format!("pm enable --user 0 {package}")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OperationPlan {
    pub operation: Operation,
    pub backup_identity: DeviceIdentity,
    pub steps: Vec<PlanStep>,
    pub verification_commands: Vec<String>,
}
