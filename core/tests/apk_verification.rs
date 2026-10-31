use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use firetv_core::{
    ApkIdentity, ApkRequirement, DeviceFacts, DeviceIdentity, LauncherCatalog, verify_apk,
};

fn facts(sdk: u32) -> DeviceFacts {
    DeviceFacts {
        identity: DeviceIdentity {
            serial: "S".into(),
            model: "M".into(),
            codename: "C".into(),
            fingerprint: "F".into(),
            fire_os: Some(if sdk >= 30 { "8" } else { "7" }.into()),
            android_sdk: sdk,
            abis: vec!["arm64-v8a".into()],
        },
        packages: BTreeMap::new(),
        home_component: None,
        settings_components: BTreeSet::new(),
        tv_input_packages: BTreeSet::new(),
        input_method_package: None,
        accessibility_services: BTreeSet::new(),
        capabilities: BTreeSet::new(),
        diagnostics: BTreeMap::new(),
        storage_available_bytes: None,
    }
}

#[test]
fn catalog_offers_projectivy_and_free_compatible_choices() {
    let catalog = LauncherCatalog::bundled().unwrap();
    let choices = catalog.compatible_launchers(&facts(28));
    assert!(
        choices
            .iter()
            .any(|entry| entry.id == "projectivy" && entry.user_supplied)
    );
    assert!(
        choices
            .iter()
            .any(|entry| entry.id == "dumbtv-home" && entry.open_source)
    );
    assert!(choices.iter().any(|entry| entry.id == "ltv-launcher"));
    assert!(
        !choices
            .iter()
            .any(|entry| entry.id == "firetv-web-launcher")
    );
}

#[test]
fn fire_os_8_catalog_includes_profile_gated_web_launcher() {
    let choices = LauncherCatalog::bundled()
        .unwrap()
        .compatible_launchers(&facts(30));
    let web = choices
        .iter()
        .find(|entry| entry.id == "firetv-web-launcher")
        .unwrap();
    assert!(web.requires_validated_profile);
}

#[test]
fn verifies_hash_package_signer_sdk_and_abi() {
    let temp = tempfile::tempdir().unwrap();
    let apk = temp.path().join("launcher.apk");
    fs::write(&apk, b"fixture apk").unwrap();
    let requirement = ApkRequirement {
        package: "com.example.launcher".into(),
        signer_sha256: "SIGNER".into(),
        sha256: Some("afbe02bf06f56d9276b23a2e1bf23676d7125d09e90facc47ba011d5a0c6168c".into()),
        min_sdk: 25,
        abis: vec!["arm64-v8a".into()],
    };
    let identity = ApkIdentity {
        package: "com.example.launcher".into(),
        signer_sha256: "SIGNER".into(),
        min_sdk: 25,
        abis: vec!["arm64-v8a".into()],
    };
    assert!(verify_apk(&apk, &identity, &requirement, &facts(28)).is_ok());
    for bad in [
        ApkIdentity {
            package: "wrong".into(),
            ..identity.clone()
        },
        ApkIdentity {
            signer_sha256: "wrong".into(),
            ..identity.clone()
        },
        ApkIdentity {
            min_sdk: 31,
            ..identity.clone()
        },
        ApkIdentity {
            abis: vec!["x86".into()],
            ..identity.clone()
        },
    ] {
        assert!(verify_apk(&apk, &bad, &requirement, &facts(28)).is_err());
    }
    fs::write(&apk, b"altered").unwrap();
    assert!(verify_apk(&apk, &identity, &requirement, &facts(28)).is_err());
}
