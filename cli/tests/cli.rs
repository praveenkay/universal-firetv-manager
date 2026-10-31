use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_lists_recovery_workflow() {
    Command::cargo_bin("firetvctl")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("inspect"))
        .stdout(predicate::str::contains("backup"))
        .stdout(predicate::str::contains("install"))
        .stdout(predicate::str::contains("restore"));
}

#[test]
fn rejects_unsafe_or_non_numeric_target_before_connecting() {
    Command::cargo_bin("firetvctl")
        .unwrap()
        .args(["connect", "fire-tv.local; reboot"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("numeric IPv4 or IPv6"));
}

#[test]
fn mutation_requires_backup_and_confirmation_pair() {
    Command::cargo_bin("firetvctl")
        .unwrap()
        .args(["install", "192.0.2.1", "--apk", "launcher.apk"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--backup"));
}

#[test]
fn catalog_json_includes_projectivy_and_free_option() {
    Command::cargo_bin("firetvctl")
        .unwrap()
        .args(["launchers", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("projectivy"))
        .stdout(predicate::str::contains("dumbtv-home"));
}
