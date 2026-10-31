use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use firetv_core::{
    BackupManifest, CriticalGuards, DeviceFacts, DeviceIdentity, Executor, ManagerError, Mutation,
    Operation, OperationPlan, PackageState, PlanStep, Planner, ProfileStore, TransactionStatus,
    VerifiedBackup,
};

fn facts() -> DeviceFacts {
    DeviceFacts {
        identity: DeviceIdentity {
            serial: "S".into(),
            model: "M".into(),
            codename: "C".into(),
            fingerprint: "F".into(),
            fire_os: None,
            android_sdk: 28,
            abis: vec!["arm64-v8a".into()],
        },
        packages: BTreeMap::from([
            ("com.optional.one".into(), PackageState::Enabled),
            ("com.optional.two".into(), PackageState::Enabled),
        ]),
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

fn backup() -> VerifiedBackup {
    VerifiedBackup {
        path: Path::new("/fixture").to_owned(),
        manifest: BackupManifest {
            schema_version: 1,
            captured_unix_seconds: 1,
            identity: facts().identity,
            settings: BTreeMap::from([("secure/missing".into(), None)]),
            files: BTreeMap::new(),
            warnings: Vec::new(),
        },
    }
}

#[test]
fn planner_requires_backup_and_exposes_rollback() {
    let selection = ProfileStore::bundled().unwrap().select(&facts());
    let mutation = Mutation::DisablePackage {
        package: "com.optional.one".into(),
    };
    assert!(matches!(
        Planner::plan(
            Operation::Install,
            &[mutation.clone()],
            &facts(),
            &selection,
            None
        ),
        Err(ManagerError::InvalidData(_))
    ));
    let plan = Planner::plan(
        Operation::Install,
        &[mutation],
        &facts(),
        &selection,
        Some(&backup()),
    )
    .unwrap();
    assert_eq!(
        plan.steps[0].apply,
        "pm disable-user --user 0 com.optional.one"
    );
    assert_eq!(
        plan.steps[0].rollback,
        Some("pm enable --user 0 com.optional.one".into())
    );
    assert_eq!(
        CriticalGuards::allows(&plan.steps[0].mutation, &facts()),
        firetv_core::GuardDecision::Allowed
    );
}

#[tokio::test]
async fn failure_rolls_back_completed_steps_and_retains_journal() {
    let transport =
        firetv_core::MockAdbTransport::with_state(firetv_core::ConnectionState::Connected);
    transport.set_shell_response(
        "pm disable-user --user 0 com.optional.one",
        "Success",
        "",
        0,
    );
    transport.fail_shell_command("pm disable-user --user 0 com.optional.two", 1, "failed");
    transport.set_shell_response("pm enable --user 0 com.optional.one", "enabled", "", 0);
    let plan = OperationPlan {
        operation: Operation::Install,
        backup_identity: facts().identity,
        steps: vec![
            PlanStep::disable("com.optional.one"),
            PlanStep::disable("com.optional.two"),
        ],
        verification_commands: Vec::new(),
    };
    let journal = Executor::execute(&transport, &plan)
        .await
        .unwrap_err()
        .journal;
    assert_eq!(journal.status, TransactionStatus::RolledBack);
    assert!(
        transport
            .shell_calls()
            .ends_with(&["pm enable --user 0 com.optional.one".into()])
    );
}

#[tokio::test]
async fn failed_verification_triggers_rollback() {
    let transport =
        firetv_core::MockAdbTransport::with_state(firetv_core::ConnectionState::Connected);
    transport.set_shell_response(
        "pm disable-user --user 0 com.optional.one",
        "Success",
        "",
        0,
    );
    transport.set_shell_response("pm enable --user 0 com.optional.one", "enabled", "", 0);
    transport.fail_shell_command("verify critical services", 1, "not healthy");
    let mut plan = OperationPlan {
        operation: Operation::Repair,
        backup_identity: facts().identity,
        steps: vec![PlanStep::disable("com.optional.one")],
        verification_commands: vec!["verify critical services".into()],
    };
    let error = Executor::execute(&transport, &plan).await.unwrap_err();
    assert_eq!(error.journal.status, TransactionStatus::RolledBack);
    plan.verification_commands.clear();
}

#[test]
fn restore_plan_deletes_setting_that_was_absent() {
    let plan = Planner::restore(&backup(), &facts()).unwrap();
    assert!(
        plan.steps
            .iter()
            .any(|step| step.apply == "settings delete secure missing")
    );
}
