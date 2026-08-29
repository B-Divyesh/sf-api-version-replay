use assert_cmd::Command;
use serde_json::Value;
use std::{fs, net::TcpStream, path::Path, thread, time::Duration};
use tempfile::tempdir;
use version_replay::{
    Fixture, default_config, diff_contract, parse_fixture_file, redact_fixture, replay_fixture,
    validate_loopback,
};

fn vr() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("vr"))
}

fn init_and_import(temp: &Path) -> std::path::PathBuf {
    let vault = temp.join("vault");
    vr().args(["--vault", vault.to_str().unwrap(), "init"])
        .assert()
        .success();
    for (version, source) in [("old", "examples/old.json"), ("new", "examples/new.json")] {
        vr().args([
            "--vault",
            vault.to_str().unwrap(),
            "import",
            "--name",
            "payment-failed",
            "--version",
            version,
            "--file",
            source,
        ])
        .assert()
        .success();
    }
    vault
}

// @claim:cli-demo-workflow
#[test]
fn claim_cli_demo_workflow() {
    let output = vr().args(["--json", "demo"]).output().unwrap();
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["ok"], true);
    assert_eq!(value["demo"], true);
    assert_eq!(value["redacted"], true);
    assert_eq!(value["changed"], true);
    assert_eq!(value["receiver_requests"], 2);
    let changes = value["change_counts"]["headers"].as_u64().unwrap()
        + value["change_counts"]["schema"].as_u64().unwrap()
        + value["change_counts"]["body"].as_u64().unwrap();
    assert_eq!(changes, 5);
    assert_eq!(value["replays"][0]["status"], 204);
    assert_eq!(value["replays"][1]["status"], 204);
    let vault = Path::new(value["vault"].as_str().unwrap());
    let report = Path::new(value["report"].as_str().unwrap());
    assert!(
        vault
            .join("fixtures/payment-failed--2024-04-10.json")
            .is_file()
    );
    assert!(
        vault
            .join("fixtures/payment-failed--2025-02-24.json")
            .is_file()
    );
    let markdown = fs::read_to_string(report).unwrap();
    assert!(markdown.contains("2024-04-10"));
    assert!(markdown.contains("2025-02-24"));
    assert!(markdown.contains("Contract changed"));
}

// @claim:redaction-before-storage
#[test]
fn claim_redaction_before_storage() {
    let temp = tempdir().unwrap();
    let path = temp.path().join("input.json");
    fs::write(&path, r#"{"method":"POST","path":"/hook","headers":{"Authorization":"secret-a","COOKIE":"secret-b","Set-Cookie":"secret-c","X-Custom-Token":"secret-d"},"body":{"email":"a@b.test","customer_email":"c@d.test","billing_email":"e@f.test","phone":"555","cvc":"123","card_number":"4242","account_number":"999","nested":[{"email":"g@h.test","token":"nested-secret"}]}}"#).unwrap();
    let mut fixture = parse_fixture_file(&path, "redaction", "v1").unwrap();
    redact_fixture(
        &mut fixture,
        &default_config(
            false,
            vec!["nested.*.token".into()],
            vec!["X-CUSTOM-TOKEN".into()],
        ),
    );
    let text = serde_json::to_string(&fixture).unwrap();
    for secret in [
        "secret-a",
        "secret-b",
        "secret-c",
        "secret-d",
        "a@b.test",
        "c@d.test",
        "e@f.test",
        "555",
        "123",
        "4242",
        "999",
        "g@h.test",
        "nested-secret",
    ] {
        assert!(!text.contains(secret), "secret remained: {secret}");
    }
    assert!(text.matches("[REDACTED]").count() >= 13);
}

// @claim:loopback-only
#[test]
fn claim_loopback_only() {
    for allowed in [
        "http://localhost:3000/hook",
        "http://127.0.0.1:3000/hook",
        "http://127.8.9.10/hook",
        "http://[::1]:3000/hook",
    ] {
        assert!(validate_loopback(allowed).is_ok(), "should allow {allowed}");
    }
    for refused in [
        "https://example.com/hook",
        "http://10.0.0.1/hook",
        "http://127.0.0.1.example.com/hook",
        "file:///tmp/hook",
    ] {
        assert!(
            validate_loopback(refused).is_err(),
            "should refuse {refused}"
        );
    }
}

// @claim:encrypted-storage
#[test]
fn claim_encrypted_storage() {
    let temp = tempdir().unwrap();
    let vault = temp.path().join("vault");
    let input = temp.path().join("secret.json");
    let passphrase = "claim-test-passphrase";
    fs::write(&input, r#"{"marker":"plaintext-must-not-remain"}"#).unwrap();
    vr().env("VERSION_REPLAY_PASSPHRASE", passphrase)
        .args(["--vault", vault.to_str().unwrap(), "init", "--encrypted"])
        .assert()
        .success();
    vr().env("VERSION_REPLAY_PASSPHRASE", passphrase)
        .args([
            "--vault",
            vault.to_str().unwrap(),
            "import",
            "--name",
            "secret",
            "--version",
            "v1",
            "--file",
            input.to_str().unwrap(),
        ])
        .assert()
        .success();
    let fixture = fs::read(vault.join("fixtures/secret--v1.vrf")).unwrap();
    let config = fs::read_to_string(vault.join("config.json")).unwrap();
    assert!(fixture.starts_with(b"VR01"));
    assert!(!String::from_utf8_lossy(&fixture).contains("plaintext-must-not-remain"));
    assert!(!config.contains(passphrase));
    vr().env("VERSION_REPLAY_PASSPHRASE", "wrong-passphrase-value")
        .args(["--vault", vault.to_str().unwrap(), "list"])
        .assert()
        .failure();
}

// @claim:capture-loopback
#[test]
fn claim_capture_loopback() {
    let temp = tempdir().unwrap();
    let vault = temp.path().join("vault");
    vr().args(["--vault", vault.to_str().unwrap(), "init"])
        .assert()
        .success();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin!("vr"))
        .args([
            "--vault",
            vault.to_str().unwrap(),
            "capture",
            "--name",
            "captured",
            "--version",
            "v1",
            "--listen",
            &address.to_string(),
            "--once",
        ])
        .spawn()
        .unwrap();
    let mut stream = (0..40)
        .find_map(|_| match TcpStream::connect(address) {
            Ok(stream) => Some(stream),
            Err(_) => {
                thread::sleep(Duration::from_millis(25));
                None
            }
        })
        .expect("capture listener did not start");
    use std::io::{Read, Write};
    let body = r#"{"customer_email":"secret@example.com","ok":true}"#;
    write!(stream, "POST /provider-hook HTTP/1.1\r\nHost: localhost\r\nAuthorization: secret-token\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    assert!(response.contains("202 Accepted"));
    assert!(child.wait().unwrap().success());
    let stored = fs::read_to_string(vault.join("fixtures/captured--v1.json")).unwrap();
    assert!(stored.contains("/provider-hook"));
    assert!(stored.contains("[REDACTED]"));
    assert!(!stored.contains("secret@example.com"));
    assert!(!stored.contains("secret-token"));
}

// @claim:report-formats
#[test]
fn claim_report_formats() {
    let temp = tempdir().unwrap();
    let vault = init_and_import(temp.path());
    let json = vr()
        .args([
            "--vault",
            vault.to_str().unwrap(),
            "report",
            "--name",
            "payment-failed",
            "--from",
            "old",
            "--to",
            "new",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(json.status.success());
    let value: Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["name"], "payment-failed");
    assert_eq!(value["from"], "old");
    assert_eq!(value["to"], "new");
    let markdown = vr()
        .args([
            "--vault",
            vault.to_str().unwrap(),
            "report",
            "--name",
            "payment-failed",
            "--from",
            "old",
            "--to",
            "new",
        ])
        .output()
        .unwrap();
    assert!(markdown.status.success());
    assert!(
        String::from_utf8(markdown.stdout)
            .unwrap()
            .contains("# Version Replay report")
    );
}

// @claim:contract-dimensions
#[test]
fn claim_contract_dimensions() {
    use std::collections::BTreeMap;
    let from = Fixture {
        name: "event".into(),
        version: "v1".into(),
        method: "POST".into(),
        path: "/old".into(),
        headers: BTreeMap::from([("x-version".into(), "1".into())]),
        body: serde_json::json!({"amount": 10, "legacy": true}),
        captured_at: 0,
    };
    let to = Fixture {
        name: "event".into(),
        version: "v2".into(),
        method: "PUT".into(),
        path: "/new".into(),
        headers: BTreeMap::from([("x-version".into(), "2".into())]),
        body: serde_json::json!({"amount": "10", "currency": "usd"}),
        captured_at: 0,
    };
    let diff = diff_contract(&from, &to);
    assert!(diff.method_changed);
    assert!(diff.path_changed);
    assert!(!diff.headers.is_empty());
    assert!(diff.schema.iter().any(|change| change.path == "$.amount"));
    assert!(diff.body.iter().any(|change| change.path == "$.currency"));
}

// @claim:exact-replay
#[test]
fn claim_exact_replay() {
    use std::collections::BTreeMap;
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let receiver = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut request = Vec::new();
        let mut bytes = [0_u8; 4096];
        loop {
            let read = stream.read(&mut bytes).unwrap();
            if read == 0 {
                break;
            }
            request.extend_from_slice(&bytes[..read]);
            let text = String::from_utf8_lossy(&request);
            if let Some(header_end) = text.find("\r\n\r\n") {
                let length = text[..header_end]
                    .lines()
                    .find_map(|line| {
                        line.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .and_then(|value| value.trim().parse::<usize>().ok())
                    })
                    .unwrap_or(0);
                if request.len() >= header_end + 4 + length {
                    break;
                }
            }
        }
        stream
            .write_all(b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
            .unwrap();
        String::from_utf8_lossy(&request).into_owned()
    });
    let fixture = Fixture {
        name: "event".into(),
        version: "v1".into(),
        method: "PATCH".into(),
        path: "/stored".into(),
        headers: BTreeMap::from([("x-sample".into(), "stored-header".into())]),
        body: serde_json::json!({"stored": true}),
        captured_at: 0,
    };
    let result = replay_fixture(&fixture, &format!("http://{address}/receiver")).unwrap();
    let request = receiver.join().unwrap();
    assert_eq!(result.status, 204);
    assert!(request.starts_with("PATCH /receiver HTTP/1.1"));
    assert!(
        request
            .to_ascii_lowercase()
            .contains("x-sample: stored-header")
    );
    assert!(request.contains("{\"stored\":true}"));
}

// @claim:fixture-formats
#[test]
fn claim_fixture_formats() {
    let temp = tempdir().unwrap();
    let plain = temp.path().join("plain.json");
    let envelope = temp.path().join("envelope.json");
    fs::write(&plain, r#"{"hello":"world"}"#).unwrap();
    fs::write(
        &envelope,
        r#"{"method":"PATCH","path":"/hook","headers":{"X-Test":"yes"},"body":{"hello":"world"}}"#,
    )
    .unwrap();
    let plain = parse_fixture_file(&plain, "plain", "v1").unwrap();
    assert_eq!(plain.method, "POST");
    assert_eq!(plain.path, "/");
    let envelope = parse_fixture_file(&envelope, "envelope", "v1").unwrap();
    assert_eq!(envelope.method, "PATCH");
    assert_eq!(envelope.path, "/hook");
    assert_eq!(envelope.headers["x-test"], "yes");
    assert_eq!(envelope.body["hello"], "world");
}

// @claim:exit-codes
#[test]
fn claim_exit_codes() {
    let temp = tempdir().unwrap();
    let vault = init_and_import(temp.path());
    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "diff",
        "--name",
        "payment-failed",
        "--from",
        "old",
        "--to",
        "new",
    ])
    .assert()
    .code(3);
    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "replay",
        "--name",
        "payment-failed",
        "--version",
        "old",
        "--to",
        "https://example.com/hook",
    ])
    .assert()
    .code(1);
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let receiver = thread::spawn(move || {
        use std::io::{Read, Write};
        let (mut stream, _) = listener.accept().unwrap();
        let mut bytes = [0_u8; 4096];
        let _ = stream.read(&mut bytes);
        stream.write_all(b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
    });
    vr().args([
        "--vault",
        vault.to_str().unwrap(),
        "replay",
        "--name",
        "payment-failed",
        "--version",
        "old",
        "--to",
        &format!("http://{address}/hook"),
    ])
    .assert()
    .code(4);
    receiver.join().unwrap();
}

// @claim:no-provider-credentials
#[test]
fn claim_no_provider_credentials() {
    let output = vr()
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .args(["--json", "demo"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["ok"], true);
}

// @claim:no-telemetry
#[test]
fn claim_no_telemetry() {
    let manifest = fs::read_to_string("Cargo.toml").unwrap();
    let source = fs::read_to_string("src/lib.rs").unwrap();
    for forbidden in [
        "opentelemetry",
        "sentry",
        "segment",
        "mixpanel",
        "amplitude",
        "google-analytics",
    ] {
        assert!(!manifest.to_ascii_lowercase().contains(forbidden));
        assert!(!source.to_ascii_lowercase().contains(forbidden));
    }
    assert_eq!(
        source.matches("ureq::request").count(),
        1,
        "replay should be the only request carrying fixture data"
    );
    assert_eq!(
        source.matches("ureq::get").count(),
        0,
        "no background network path should exist"
    );
}

// @claim:mit-license
#[test]
fn claim_mit_license() {
    let license = fs::read_to_string("LICENSE").unwrap();
    assert!(license.contains("Permission is hereby granted, free of charge"));
    assert!(license.contains("THE SOFTWARE IS PROVIDED \"AS IS\""));
}
