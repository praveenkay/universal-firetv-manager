use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signer, SigningKey};
use firetv_core::{
    DeviceFacts, DeviceIdentity, Profile, ProfileStatus, ProfileStore, SupportLevel,
};

fn facts(fingerprint: &str, sdk: u32, abis: &[&str]) -> DeviceFacts {
    DeviceFacts {
        identity: DeviceIdentity {
            serial: "REDACTED".into(),
            model: "Fixture".into(),
            codename: "raven-fixture".into(),
            fingerprint: fingerprint.into(),
            fire_os: Some("7.0.0.0".into()),
            android_sdk: sdk,
            abis: abis.iter().map(|value| (*value).into()).collect(),
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
fn exact_fingerprint_selects_experimental_profile_without_auto_enabling_it() {
    let store = ProfileStore::bundled().unwrap();
    let selection = store.select(&facts(
        "amazon/raven-fixture/raven-fixture:9/TEST/fixture:user/release-keys",
        28,
        &["arm64-v8a"],
    ));
    assert_eq!(selection.level, SupportLevel::BaseOnly);
    assert_eq!(
        selection.exact_profile.as_ref().unwrap().id,
        "raven-experimental"
    );
    assert_eq!(
        selection.exact_profile.as_ref().unwrap().status,
        ProfileStatus::Experimental
    );
}

#[test]
fn changed_or_unknown_fingerprint_falls_back_to_base_safe() {
    let store = ProfileStore::bundled().unwrap();
    let selection = store.select(&facts("amazon/unknown:new-build", 28, &["arm64-v8a"]));
    assert_eq!(selection.level, SupportLevel::BaseOnly);
    assert!(selection.exact_profile.is_none());
    assert_eq!(selection.base.id, "base-safe");
}

#[test]
fn remote_profile_with_invalid_signature_is_rejected() {
    let profile = Profile::base_safe();
    let json = serde_json::to_vec(&profile).unwrap();
    let trusted = SigningKey::from_bytes(&[7; 32]);
    let attacker = SigningKey::from_bytes(&[9; 32]);
    let signature = attacker.sign(&json);
    assert!(
        ProfileStore::load_signed(&json, signature.to_bytes(), trusted.verifying_key()).is_err()
    );
}

#[test]
fn incompatible_sdk_or_abi_blocks_profile() {
    let store = ProfileStore::bundled().unwrap();
    let selection = store.select(&facts("amazon/unknown", 22, &["x86"]));
    assert_eq!(selection.level, SupportLevel::Blocked);
}
