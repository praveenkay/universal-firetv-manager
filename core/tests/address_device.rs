use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use firetv_core::{DeviceIdentity, SupportLevel, TargetAddress};

#[test]
fn ipv4_defaults_to_adb_port() {
    let target = TargetAddress::parse("10.0.0.42").expect("valid IPv4 target");
    assert_eq!(target.ip(), IpAddr::V4(Ipv4Addr::new(10, 0, 0, 42)));
    assert_eq!(target.port(), 5555);
}

#[test]
fn ipv4_accepts_explicit_port() {
    let target = TargetAddress::parse("10.0.0.42:5566").expect("valid IPv4 target");
    assert_eq!(target.port(), 5566);
    assert_eq!(target.to_string(), "10.0.0.42:5566");
}

#[test]
fn bracketed_ipv6_accepts_explicit_port() {
    let target = TargetAddress::parse("[fd00::44]:5556").expect("valid IPv6 target");
    assert_eq!(
        target.ip(),
        IpAddr::V6("fd00::44".parse::<Ipv6Addr>().unwrap())
    );
    assert_eq!(target.port(), 5556);
    assert_eq!(target.to_string(), "[fd00::44]:5556");
}

#[test]
fn bare_ipv6_defaults_to_adb_port() {
    let target = TargetAddress::parse("fd00::44").expect("valid IPv6 target");
    assert_eq!(target.port(), 5555);
}

#[test]
fn rejects_whitespace_hostnames_metacharacters_and_invalid_ports() {
    for input in [
        " 10.0.0.42",
        "10.0.0.42 ",
        "fire-tv.local",
        "10.0.0.42;reboot",
        "10.0.0.42:0",
        "10.0.0.42:65536",
        "[fd00::44]",
        "",
    ] {
        assert!(TargetAddress::parse(input).is_err(), "accepted {input:?}");
    }
}

#[test]
fn device_identity_serialization_is_stable() {
    let identity = DeviceIdentity {
        serial: "REDACTED-SERIAL".into(),
        model: "Fire TV Test Device".into(),
        codename: "fixture".into(),
        fingerprint: "amazon/fixture/fixture:11/TEST/1:user/release-keys".into(),
        fire_os: Some("8.0.0.0".into()),
        android_sdk: 30,
        abis: vec!["arm64-v8a".into(), "armeabi-v7a".into()],
    };

    let json = serde_json::to_string(&identity).unwrap();
    assert_eq!(
        json,
        r#"{"serial":"REDACTED-SERIAL","model":"Fire TV Test Device","codename":"fixture","fingerprint":"amazon/fixture/fixture:11/TEST/1:user/release-keys","fire_os":"8.0.0.0","android_sdk":30,"abis":["arm64-v8a","armeabi-v7a"]}"#
    );
    assert_eq!(SupportLevel::BaseOnly.to_string(), "base-only");
}
