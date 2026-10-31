use std::collections::{BTreeMap, BTreeSet};

use firetv_core::{
    CriticalGuards, DeviceFacts, DeviceIdentity, GuardDecision, Mutation, PackageState,
};

fn facts() -> DeviceFacts {
    DeviceFacts {
        identity: DeviceIdentity {
            serial: "REDACTED".into(),
            model: "Fixture".into(),
            codename: "fixture".into(),
            fingerprint: "fixture".into(),
            fire_os: None,
            android_sdk: 28,
            abis: vec!["arm64-v8a".into()],
        },
        packages: BTreeMap::from([
            ("com.amazon.tv.launcher".into(), PackageState::Enabled),
            ("com.amazon.tv.ime".into(), PackageState::Enabled),
            ("com.amazon.franktvinput".into(), PackageState::Enabled),
            ("com.example.optional".into(), PackageState::Enabled),
        ]),
        home_component: Some("com.amazon.tv.launcher/.Home".into()),
        settings_components: BTreeSet::new(),
        tv_input_packages: BTreeSet::from(["com.amazon.franktvinput".into()]),
        input_method_package: Some("com.amazon.tv.ime".into()),
        accessibility_services: BTreeSet::new(),
        capabilities: BTreeSet::new(),
        diagnostics: BTreeMap::new(),
        storage_available_bytes: None,
    }
}

#[test]
fn protects_static_and_runtime_critical_packages() {
    let facts = facts();
    for package in [
        "com.amazon.tv.launcher",
        "com.amazon.tv.ime",
        "com.amazon.franktvinput",
    ] {
        assert!(matches!(
            CriticalGuards::allows(
                &Mutation::DisablePackage {
                    package: package.into()
                },
                &facts
            ),
            GuardDecision::Denied(_)
        ));
    }
}

#[test]
fn absent_unknown_and_non_enabled_packages_are_denied() {
    let facts = facts();
    assert!(matches!(
        CriticalGuards::allows(
            &Mutation::DisablePackage {
                package: "com.unknown".into()
            },
            &facts
        ),
        GuardDecision::Denied(_)
    ));
}

#[test]
fn enabled_noncritical_package_is_allowed() {
    assert_eq!(
        CriticalGuards::allows(
            &Mutation::DisablePackage {
                package: "com.example.optional".into()
            },
            &facts()
        ),
        GuardDecision::Allowed
    );
}
