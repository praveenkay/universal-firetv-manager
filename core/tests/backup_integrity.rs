use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use firetv_core::{
    BackupOptions, DeviceFacts, DeviceIdentity, ManagerError, MockAdbTransport, PackageState,
    capture_backup, export_redacted_report, verify_backup,
};

fn facts() -> DeviceFacts {
    DeviceFacts {
        identity: DeviceIdentity {
            serial: "SERIAL-SECRET".into(),
            model: "Fixture".into(),
            codename: "fixture".into(),
            fingerprint: "amazon/fixture".into(),
            fire_os: Some("8".into()),
            android_sdk: 30,
            abis: vec!["arm64-v8a".into()],
        },
        packages: BTreeMap::from([("com.example".into(), PackageState::Enabled)]),
        home_component: None,
        settings_components: BTreeSet::new(),
        tv_input_packages: BTreeSet::new(),
        input_method_package: None,
        accessibility_services: BTreeSet::new(),
        capabilities: BTreeSet::new(),
        diagnostics: BTreeMap::from([(
            "network".into(),
            "IP 10.0.0.20 MAC aa:bb:cc:dd:ee:ff SERIAL-SECRET".into(),
        )]),
        storage_available_bytes: Some(1024),
    }
}

#[tokio::test]
async fn captures_metadata_missing_settings_and_hash_inventory() {
    let transport = MockAdbTransport::with_state(firetv_core::ConnectionState::Connected);
    transport.set_shell_response("settings get secure present_key", "value\n", "", 0);
    transport.set_shell_response("settings get secure missing_key", "null\n", "", 0);
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("backup");
    let manifest = capture_backup(
        &transport,
        &facts(),
        &destination,
        &BackupOptions {
            full: false,
            settings: vec![
                ("secure".into(), "present_key".into()),
                ("secure".into(), "missing_key".into()),
            ],
        },
    )
    .await
    .unwrap();
    assert_eq!(
        manifest.settings["secure/present_key"].as_deref(),
        Some("value")
    );
    assert_eq!(manifest.settings["secure/missing_key"], None);
    assert!(manifest.files.contains_key("device-facts.json"));
    assert!(destination.join("BACKUP-LIMITS.md").is_file());
    verify_backup(&destination, &facts().identity).unwrap();
}

#[tokio::test]
async fn rejects_tampering_and_mismatched_identity() {
    let transport = MockAdbTransport::with_state(firetv_core::ConnectionState::Connected);
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("backup");
    capture_backup(
        &transport,
        &facts(),
        &destination,
        &BackupOptions::default(),
    )
    .await
    .unwrap();
    fs::write(destination.join("device-facts.json"), "tampered").unwrap();
    assert!(matches!(
        verify_backup(&destination, &facts().identity),
        Err(ManagerError::Integrity(_))
    ));
    let mut other = facts().identity;
    other.serial = "OTHER".into();
    assert!(matches!(
        verify_backup(&destination, &other),
        Err(ManagerError::DeviceMismatch)
    ));
}

#[tokio::test]
async fn failed_capture_removes_incomplete_directory() {
    let transport = MockAdbTransport::with_state(firetv_core::ConnectionState::Connected);
    let temp = tempfile::tempdir().unwrap();
    let destination = temp.path().join("backup");
    let result = capture_backup(
        &transport,
        &facts(),
        &destination,
        &BackupOptions {
            full: false,
            settings: vec![("secure".into(), "unavailable".into())],
        },
    )
    .await;
    assert!(result.is_err());
    assert!(!destination.exists());
    assert!(!temp.path().join("backup.incomplete").exists());
}

#[test]
fn redacts_device_and_network_identifiers() {
    let report = export_redacted_report(&facts()).unwrap();
    assert!(!report.contains("SERIAL-SECRET"));
    assert!(!report.contains("10.0.0.20"));
    assert!(!report.contains("aa:bb:cc:dd:ee:ff"));
    assert!(report.contains("REDACTED"));
}
