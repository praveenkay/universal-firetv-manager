use firetv_core::{HealthStatus, MockAdbTransport, inspect_device, verify_device};

const FIXTURE: &str = include_str!("../../tests/fixtures/raven-inspection.json");

#[tokio::test]
async fn reports_critical_services_and_not_applicable_capabilities() {
    let transport = MockAdbTransport::from_fixture(FIXTURE).unwrap();
    transport.set_shell_response("dumpsys account", "Account {name=REDACTED}", "", 0);
    let facts = inspect_device(&transport).await.unwrap();
    let report = verify_device(&transport, &facts).await;
    assert_eq!(report.checks["home"].status, HealthStatus::Pass);
    assert_eq!(report.checks["settings"].status, HealthStatus::Pass);
    assert_eq!(
        report.checks["appstore"].status,
        HealthStatus::NotApplicable
    );
    assert_eq!(
        report.checks["internal-speaker"].status,
        HealthStatus::NotApplicable
    );
}
