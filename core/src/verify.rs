use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{AdbTransport, DeviceFacts, PackageState};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HealthStatus {
    Pass,
    Fail,
    NotApplicable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthCheck {
    pub status: HealthStatus,
    pub detail: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HealthReport {
    pub checks: BTreeMap<String, HealthCheck>,
}

fn check(status: HealthStatus, detail: impl Into<String>) -> HealthCheck {
    HealthCheck {
        status,
        detail: detail.into(),
    }
}

fn enabled_package(facts: &DeviceFacts, package: &str) -> bool {
    matches!(facts.packages.get(package), Some(PackageState::Enabled))
}

fn static_checks(facts: &DeviceFacts) -> BTreeMap<String, HealthCheck> {
    let mut checks = BTreeMap::new();
    checks.insert(
        "home".into(),
        if facts.home_component.is_some() {
            check(HealthStatus::Pass, "A Home activity resolves")
        } else {
            check(HealthStatus::Fail, "No Home activity resolves")
        },
    );
    checks.insert(
        "settings".into(),
        if facts.settings_components.is_empty() {
            check(HealthStatus::Fail, "No settings activity resolves")
        } else {
            check(HealthStatus::Pass, "Settings remains available")
        },
    );
    checks.insert(
        "keyboard".into(),
        match facts.input_method_package.as_deref() {
            Some(package) if enabled_package(facts, package) => {
                check(HealthStatus::Pass, "The configured keyboard is enabled")
            }
            Some(_) => check(HealthStatus::Fail, "The configured keyboard is unavailable"),
            None => check(HealthStatus::NotApplicable, "No keyboard was reported"),
        },
    );
    checks.insert(
        "appstore".into(),
        if facts.packages.contains_key("com.amazon.venezia") {
            if enabled_package(facts, "com.amazon.venezia") {
                check(HealthStatus::Pass, "Amazon Appstore is enabled")
            } else {
                check(HealthStatus::Fail, "Amazon Appstore is disabled")
            }
        } else {
            check(
                HealthStatus::NotApplicable,
                "Amazon Appstore is not present",
            )
        },
    );
    checks.insert(
        "bluetooth".into(),
        if facts
            .capabilities
            .iter()
            .any(|value| value.contains("bluetooth"))
        {
            check(HealthStatus::Pass, "Bluetooth capability is present")
        } else {
            check(HealthStatus::NotApplicable, "Bluetooth was not reported")
        },
    );
    checks.insert(
        "tv-inputs".into(),
        if facts.tv_input_packages.is_empty() {
            check(HealthStatus::NotApplicable, "No TV inputs were reported")
        } else {
            check(HealthStatus::Pass, "TV input services remain available")
        },
    );
    checks.insert(
        "storage".into(),
        match facts.storage_available_bytes {
            Some(bytes) if bytes >= 128 * 1024 * 1024 => {
                check(HealthStatus::Pass, format!("{bytes} bytes available"))
            }
            Some(bytes) => check(HealthStatus::Fail, format!("Only {bytes} bytes available")),
            None => check(HealthStatus::NotApplicable, "Storage was not reported"),
        },
    );
    checks.insert(
        "internal-speaker".into(),
        if facts
            .capabilities
            .iter()
            .any(|value| value.contains("amazon.hardware.audio.speaker"))
        {
            check(
                HealthStatus::Pass,
                "Cube internal-speaker capability is present",
            )
        } else {
            check(
                HealthStatus::NotApplicable,
                "Internal speaker capability was not reported",
            )
        },
    );

    checks
}

pub async fn verify_device(transport: &dyn AdbTransport, facts: &DeviceFacts) -> HealthReport {
    let mut checks = static_checks(facts);
    let accounts = transport.shell("dumpsys account").await;
    checks.insert(
        "accounts".into(),
        match accounts {
            Ok(output) if output.stdout.contains("Account {") => {
                check(HealthStatus::Pass, "At least one account is configured")
            }
            Ok(_) => check(HealthStatus::Fail, "No configured accounts were reported"),
            Err(error) => check(HealthStatus::Fail, format!("Account check failed: {error}")),
        },
    );

    HealthReport { checks }
}
