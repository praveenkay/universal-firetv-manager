use serde::{Deserialize, Serialize};

use crate::{DeviceFacts, PackageState};

const PROTECTED_PACKAGES: &[&str] = &[
    "com.amazon.tv.launcher",
    "com.amazon.firehomestarter",
    "com.amazon.tv.settings.v2",
    "com.amazon.tv.settings.core",
    "com.android.settings",
    "com.amazon.tv.oobe",
    "com.amazon.firebat",
    "com.amazon.venezia",
    "com.amazon.tigris",
    "com.android.bluetooth",
    "com.amazon.fireinputdevices",
    "com.amazon.tv.devicecontrol",
    "com.amazon.device.software.ota",
    "com.amazon.tv.forcedotaupdater.v2",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Mutation {
    DisablePackage {
        package: String,
    },
    EnablePackage {
        package: String,
    },
    SetSetting {
        namespace: String,
        key: String,
        value: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GuardDecision {
    Allowed,
    Denied(String),
}

pub struct CriticalGuards;

impl CriticalGuards {
    #[must_use]
    pub fn allows(mutation: &Mutation, facts: &DeviceFacts) -> GuardDecision {
        let Mutation::DisablePackage { package } = mutation else {
            return GuardDecision::Allowed;
        };
        if facts.packages.get(package) != Some(&PackageState::Enabled) {
            return GuardDecision::Denied("package is absent, unknown, or not enabled".into());
        }
        let current_home = facts
            .home_component
            .as_deref()
            .and_then(|component| component.split_once('/').map(|(name, _)| name));
        if PROTECTED_PACKAGES.contains(&package.as_str())
            || current_home == Some(package)
            || facts.input_method_package.as_deref() == Some(package)
            || facts.tv_input_packages.contains(package)
        {
            return GuardDecision::Denied("critical runtime or protected package".into());
        }
        GuardDecision::Allowed
    }
}
