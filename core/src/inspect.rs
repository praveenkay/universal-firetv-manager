use std::collections::{BTreeMap, BTreeSet};

use crate::{AdbTransport, DeviceFacts, DeviceIdentity, ManagerError, PackageState, Result};

async fn required(transport: &dyn AdbTransport, command: &str) -> Result<String> {
    let value = transport.shell(command).await?.stdout.trim().to_owned();
    if value.is_empty() || value == "null" {
        return Err(ManagerError::InvalidData(format!(
            "required command returned no value: {command}"
        )));
    }
    Ok(value)
}

async fn optional(transport: &dyn AdbTransport, command: &str) -> Option<String> {
    transport
        .shell(command)
        .await
        .ok()
        .map(|output| output.stdout.trim().to_owned())
        .filter(|value| !value.is_empty() && value != "null")
}

fn package_lines(output: &str) -> impl Iterator<Item = &str> {
    output
        .lines()
        .filter_map(|line| line.trim().strip_prefix("package:"))
        .filter(|package| !package.is_empty())
}

fn component_packages(output: &str) -> BTreeSet<String> {
    output
        .lines()
        .filter_map(|line| line.trim().split_once('/').map(|(package, _)| package))
        .filter(|package| !package.is_empty())
        .map(str::to_owned)
        .collect()
}

fn parse_storage_bytes(output: &str) -> Option<u64> {
    let fields: Vec<&str> = output.lines().last()?.split_whitespace().collect();
    fields.get(3)?.parse::<u64>().ok()?.checked_mul(1024)
}

/// Captures read-only compatibility facts from a connected Fire TV.
///
/// # Errors
///
/// Returns an error when a required identity/package command fails or returns malformed data.
#[expect(
    clippy::too_many_lines,
    reason = "the fixed read-only inventory is intentionally visible in one audit sequence"
)]
pub async fn inspect_device(transport: &dyn AdbTransport) -> Result<DeviceFacts> {
    let serial = required(transport, "getprop ro.serialno").await?;
    let model = required(transport, "getprop ro.product.model").await?;
    let codename = required(transport, "getprop ro.product.device").await?;
    let fingerprint = required(transport, "getprop ro.build.fingerprint").await?;
    let fire_os = optional(transport, "getprop ro.build.version.fireos").await;
    let android_sdk = required(transport, "getprop ro.build.version.sdk")
        .await?
        .parse()
        .map_err(|_| ManagerError::InvalidData("invalid Android SDK value".into()))?;
    let abis = required(transport, "getprop ro.product.cpu.abilist")
        .await?
        .split(',')
        .map(str::trim)
        .filter(|abi| !abi.is_empty())
        .map(str::to_owned)
        .collect();

    let enabled = required(transport, "pm list packages --user 0").await?;
    let disabled = transport
        .shell("pm list packages -d --user 0")
        .await?
        .stdout;
    let mut packages = BTreeMap::new();
    for package in package_lines(&enabled) {
        packages.insert(package.to_owned(), PackageState::Enabled);
    }
    for package in package_lines(&disabled) {
        packages.insert(package.to_owned(), PackageState::Disabled);
    }

    let home_component = optional(
        transport,
        "cmd package resolve-activity --brief -a android.intent.action.MAIN -c android.intent.category.HOME",
    )
    .await;
    let settings_components = optional(
        transport,
        "cmd package query-activities --brief -a android.settings.SETTINGS",
    )
    .await
    .unwrap_or_default()
    .lines()
    .map(str::trim)
    .filter(|line| line.contains('/'))
    .map(str::to_owned)
    .collect();
    let tv_input_packages = component_packages(
        &optional(
            transport,
            "cmd package query-services --components -a android.media.tv.TvInputService",
        )
        .await
        .unwrap_or_default(),
    );
    let input_method_package = optional(transport, "settings get secure default_input_method")
        .await
        .and_then(|component| {
            component
                .split_once('/')
                .map(|(package, _)| package.to_owned())
        });
    let accessibility_services = optional(
        transport,
        "settings get secure enabled_accessibility_services",
    )
    .await
    .unwrap_or_default()
    .split(':')
    .map(str::trim)
    .filter(|service| !service.is_empty())
    .map(str::to_owned)
    .collect();

    let feature_output = optional(transport, "pm list features")
        .await
        .unwrap_or_default();
    let mut capabilities = BTreeSet::new();
    if feature_output.contains("android.hardware.bluetooth") {
        capabilities.insert("bluetooth".into());
    }
    if feature_output.contains("android.software.leanback") {
        capabilities.insert("leanback".into());
    }
    if !tv_input_packages.is_empty() {
        capabilities.insert("tv-inputs".into());
    }

    let mut diagnostics = BTreeMap::new();
    for (name, command) in [
        ("bluetooth", "dumpsys bluetooth_manager"),
        ("hdmi", "dumpsys hdmi_control"),
        ("audio", "dumpsys audio"),
        ("network", "dumpsys connectivity"),
    ] {
        if let Some(value) = optional(transport, command).await {
            diagnostics.insert(name.into(), value);
        }
    }
    let storage_available_bytes = optional(transport, "df -k /data")
        .await
        .and_then(|output| parse_storage_bytes(&output));

    Ok(DeviceFacts {
        identity: DeviceIdentity {
            serial,
            model,
            codename,
            fingerprint,
            fire_os,
            android_sdk,
            abis,
        },
        packages,
        home_component,
        settings_components,
        tv_input_packages,
        input_method_package,
        accessibility_services,
        capabilities,
        diagnostics,
        storage_available_bytes,
    })
}
