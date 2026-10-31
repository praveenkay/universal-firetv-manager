use firetv_core::{
    AdbTransport, ConnectionState, ManagerError, MockAdbTransport, PackageState, TargetAddress,
    inspect_device,
};

const FIXTURE: &str = include_str!("../../tests/fixtures/raven-inspection.json");

#[tokio::test]
async fn unauthorized_connection_runs_no_shell_commands() {
    let transport = MockAdbTransport::with_state(ConnectionState::Unauthorized);
    let state = transport
        .connect(TargetAddress::parse("10.0.0.20").unwrap())
        .await
        .unwrap();
    assert_eq!(state, ConnectionState::Unauthorized);
    assert_eq!(transport.shell_call_count(), 0);
}

#[tokio::test]
async fn offline_connection_runs_no_shell_commands() {
    let transport = MockAdbTransport::with_state(ConnectionState::Offline);
    let state = transport
        .connect(TargetAddress::parse("10.0.0.20").unwrap())
        .await
        .unwrap();
    assert_eq!(state, ConnectionState::Offline);
    assert_eq!(transport.shell_call_count(), 0);
}

#[tokio::test]
async fn inspects_redacted_fire_tv_fixture() {
    let transport = MockAdbTransport::from_fixture(FIXTURE).unwrap();
    assert_eq!(
        transport
            .connect(TargetAddress::parse("10.0.0.20").unwrap())
            .await
            .unwrap(),
        ConnectionState::Connected
    );

    let facts = inspect_device(&transport).await.unwrap();
    assert_eq!(facts.identity.serial, "REDACTED-SERIAL");
    assert_eq!(facts.identity.codename, "raven-fixture");
    assert_eq!(facts.identity.android_sdk, 28);
    assert_eq!(
        facts.packages.get("com.amazon.tv.launcher"),
        Some(&PackageState::Enabled)
    );
    assert_eq!(
        facts.packages.get("com.example.disabled"),
        Some(&PackageState::Disabled)
    );
    assert_eq!(
        facts.home_component.as_deref(),
        Some("com.amazon.tv.launcher/.ui.HomeActivity_vNext")
    );
    assert!(facts.tv_input_packages.contains("com.amazon.franktvinput"));
    assert_eq!(
        facts.input_method_package.as_deref(),
        Some("com.amazon.tv.ime")
    );
    assert!(facts.capabilities.contains("bluetooth"));
    assert_eq!(facts.storage_available_bytes, Some(7_340_032_000));
}

#[tokio::test]
async fn missing_required_property_is_rejected() {
    let transport = MockAdbTransport::from_fixture(FIXTURE).unwrap();
    transport.set_shell_response("getprop ro.product.model", "", "", 0);
    let error = inspect_device(&transport).await.unwrap_err();
    assert!(matches!(error, ManagerError::InvalidData(_)));
}

#[tokio::test]
async fn command_failure_is_propagated() {
    let transport = MockAdbTransport::from_fixture(FIXTURE).unwrap();
    transport.fail_shell_command(
        "pm list packages --user 0",
        1,
        "package manager unavailable",
    );
    let error = inspect_device(&transport).await.unwrap_err();
    assert!(matches!(error, ManagerError::CommandFailed(_)));
}
