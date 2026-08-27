use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use tempfile::tempdir;

fn vr() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("vr"))
}

#[test]
fn documented_import_diff_and_report_workflow() {
    let temp = tempdir().unwrap();
    let vault = temp.path().join("vault");
    let old = temp.path().join("old.json");
    let new = temp.path().join("new.json");
    fs::write(
        &old,
        r#"{
          "method":"POST",
          "path":"/webhook",
          "headers":{"content-type":"application/json","authorization":"secret"},
          "body":{"customer":{"email":"person@example.com"},"amount":1000}
        }"#,
    )
    .unwrap();
    fs::write(
        &new,
        r#"{
          "method":"POST",
          "path":"/webhook",
          "headers":{"content-type":"application/json","authorization":"another-secret"},
          "body":{"customer":{"email":"new@example.com"},"amount":"1000","currency":"usd"}
        }"#,
    )
    .unwrap();

    vr().args(["--vault", vault.to_str().unwrap(), "init"])
        .assert()
        .success();
    for (version, file) in [("2024-04-10", &old), ("2025-02-24", &new)] {
        vr().args([
            "--vault",
            vault.to_str().unwrap(),
            "import",
            "--name",
            "payment-failed",
            "--version",
            version,
            "--file",
            file.to_str().unwrap(),
        ])
        .assert()
        .success();
    }

    let stored: Value = serde_json::from_slice(
        &fs::read(
            vault
                .join("fixtures")
                .join("payment-failed--2024-04-10.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(stored["headers"]["authorization"], "[REDACTED]");
    assert_eq!(stored["body"]["customer"]["email"], "[REDACTED]");

    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "diff",
        "--name",
        "payment-failed",
        "--from",
        "2024-04-10",
        "--to",
        "2025-02-24",
    ])
    .assert()
    .code(3)
    .stdout(predicate::str::contains("$.currency"));

    let report = temp.path().join("report.md");
    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "report",
        "--name",
        "payment-failed",
        "--from",
        "2024-04-10",
        "--to",
        "2025-02-24",
        "--output",
        report.to_str().unwrap(),
    ])
    .assert()
    .success();
    let report = fs::read_to_string(report).unwrap();
    assert!(report.contains("Version Replay report"));
    assert!(report.contains("Contract changed"));
}

#[test]
fn encrypted_vault_does_not_persist_fixture_plaintext() {
    let temp = tempdir().unwrap();
    let vault = temp.path().join("vault");
    let fixture = temp.path().join("fixture.json");
    fs::write(&fixture, r#"{"token":"fixture-secret-marker"}"#).unwrap();

    vr().env("VERSION_REPLAY_PASSPHRASE", "correct horse battery staple")
        .args(["--vault", vault.to_str().unwrap(), "init", "--encrypted"])
        .assert()
        .success();
    vr().env("VERSION_REPLAY_PASSPHRASE", "correct horse battery staple")
        .args([
            "--vault",
            vault.to_str().unwrap(),
            "import",
            "--name",
            "secret",
            "--version",
            "v1",
            "--file",
            fixture.to_str().unwrap(),
        ])
        .assert()
        .success();

    let bytes = fs::read(vault.join("fixtures").join("secret--v1.vrf")).unwrap();
    assert!(bytes.starts_with(b"VR01"));
    assert!(!String::from_utf8_lossy(&bytes).contains("fixture-secret-marker"));

    vr().env("VERSION_REPLAY_PASSPHRASE", "correct horse battery staple")
        .args(["--vault", vault.to_str().unwrap(), "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("secret\tv1"));
}

#[test]
fn replay_refuses_non_loopback_destinations_without_network_access() {
    let temp = tempdir().unwrap();
    let vault = temp.path().join("vault");
    let fixture = temp.path().join("fixture.json");
    fs::write(&fixture, r#"{"hello":"world"}"#).unwrap();
    vr().args(["--vault", vault.to_str().unwrap(), "init"])
        .assert()
        .success();
    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "import",
        "--name",
        "safe",
        "--version",
        "v1",
        "--file",
        fixture.to_str().unwrap(),
    ])
    .assert()
    .success();

    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "replay",
        "--name",
        "safe",
        "--version",
        "v1",
        "--to",
        "https://example.com/webhook",
    ])
    .assert()
    .failure()
    .stderr(predicate::str::contains("refusing non-loopback"));
}
