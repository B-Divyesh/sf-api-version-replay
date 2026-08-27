use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use anyhow::{Context, Result, anyhow, bail};
use argon2::Argon2;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use url::Url;

pub const PASSPHRASE_ENV: &str = "VERSION_REPLAY_PASSPHRASE";
pub const BILLING_BASE_ENV: &str = "VERSION_REPLAY_BILLING_BASE";
const PRODUCT_SLUG: &str = "api-version-replay";
const REDACTED: &str = "[REDACTED]";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub format_version: u8,
    pub encrypted: bool,
    pub salt: Option<String>,
    pub redact_body: Vec<String>,
    pub redact_headers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseCache>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCache {
    pub token: String,
    pub valid: bool,
    pub reason: String,
    pub checked_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub version: String,
    pub method: String,
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub body: Value,
    pub captured_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    Added,
    Removed,
    Changed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Change {
    pub path: String,
    pub kind: ChangeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDiff {
    pub name: String,
    pub from: String,
    pub to: String,
    pub method_changed: bool,
    pub path_changed: bool,
    pub headers: Vec<Change>,
    pub schema: Vec<Change>,
    pub body: Vec<Change>,
}

impl ContractDiff {
    pub fn has_changes(&self) -> bool {
        self.method_changed
            || self.path_changed
            || !self.headers.is_empty()
            || !self.schema.is_empty()
            || !self.body.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub name: String,
    pub version: String,
    pub destination: String,
    pub status: u16,
    pub ok: bool,
    pub response_excerpt: String,
}

#[derive(Debug, Deserialize)]
struct VerifyResponse {
    valid: bool,
    reason: String,
    #[serde(default)]
    expires_at: Option<String>,
}

pub fn default_config(encrypted: bool, mut body: Vec<String>, mut headers: Vec<String>) -> Config {
    let mut defaults = vec![
        "**.email".to_string(),
        "**.phone".to_string(),
        "**.cvc".to_string(),
        "**.card_number".to_string(),
        "**.account_number".to_string(),
    ];
    defaults.append(&mut body);
    defaults.sort();
    defaults.dedup();
    let mut default_headers = vec![
        "authorization".to_string(),
        "cookie".to_string(),
        "set-cookie".to_string(),
    ];
    default_headers.append(&mut headers);
    default_headers = default_headers
        .into_iter()
        .map(|value| value.to_ascii_lowercase())
        .collect();
    default_headers.sort();
    default_headers.dedup();

    let salt = encrypted.then(|| {
        let mut bytes = [0_u8; 16];
        OsRng.fill_bytes(&mut bytes);
        BASE64.encode(bytes)
    });
    Config {
        format_version: 1,
        encrypted,
        salt,
        redact_body: defaults,
        redact_headers: default_headers,
        license: None,
    }
}

pub fn init_vault(vault: &Path, config: &Config) -> Result<()> {
    if vault.join("config.json").exists() {
        bail!("vault already exists at {}", vault.display());
    }
    if config.encrypted {
        passphrase()?;
    }
    fs::create_dir_all(vault.join("fixtures"))
        .with_context(|| format!("create vault at {}", vault.display()))?;
    save_config(vault, config)
}

pub fn load_config(vault: &Path) -> Result<Config> {
    let path = vault.join("config.json");
    let bytes = fs::read(&path).with_context(|| {
        format!(
            "vault not initialized at {}; run `vr init`",
            vault.display()
        )
    })?;
    serde_json::from_slice(&bytes).context("parse vault config")
}

pub fn save_config(vault: &Path, config: &Config) -> Result<()> {
    let path = vault.join("config.json");
    let bytes = serde_json::to_vec_pretty(config)?;
    fs::write(&path, bytes).with_context(|| format!("write {}", path.display()))
}

pub fn parse_fixture_file(path: &Path, name: &str, version: &str) -> Result<Fixture> {
    validate_label("name", name)?;
    validate_label("version", version)?;
    let raw = fs::read(path).with_context(|| format!("read fixture input {}", path.display()))?;
    let value: Value = serde_json::from_slice(&raw).context("fixture input is not valid JSON")?;
    let is_envelope = value
        .as_object()
        .is_some_and(|object| object.contains_key("body"));
    let (method, request_path, headers, body) = if is_envelope {
        let object = value.as_object().expect("checked object");
        let method = object
            .get("method")
            .and_then(Value::as_str)
            .unwrap_or("POST")
            .to_ascii_uppercase();
        let request_path = object
            .get("path")
            .and_then(Value::as_str)
            .unwrap_or("/")
            .to_string();
        let mut headers = BTreeMap::new();
        if let Some(map) = object.get("headers").and_then(Value::as_object) {
            for (key, value) in map {
                let value = value
                    .as_str()
                    .ok_or_else(|| anyhow!("header `{key}` must be a string"))?;
                headers.insert(key.to_ascii_lowercase(), value.to_string());
            }
        }
        let body = object.get("body").cloned().unwrap_or(Value::Null);
        (method, request_path, headers, body)
    } else {
        ("POST".to_string(), "/".to_string(), BTreeMap::new(), value)
    };
    if !request_path.starts_with('/') {
        bail!("fixture path must begin with `/`");
    }
    Ok(Fixture {
        name: name.to_string(),
        version: version.to_string(),
        method,
        path: request_path,
        headers,
        body,
        captured_at: now(),
    })
}

pub fn redact_fixture(fixture: &mut Fixture, config: &Config) {
    for (header, value) in &mut fixture.headers {
        if config
            .redact_headers
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(header))
        {
            *value = REDACTED.to_string();
        }
    }
    redact_value(&mut fixture.body, &config.redact_body, &mut Vec::new());
}

pub fn store_fixture(
    vault: &Path,
    config: &Config,
    fixture: &Fixture,
    force: bool,
) -> Result<PathBuf> {
    validate_label("name", &fixture.name)?;
    validate_label("version", &fixture.version)?;
    let extension = if config.encrypted { "vrf" } else { "json" };
    let path = vault.join("fixtures").join(format!(
        "{}--{}.{}",
        fixture.name, fixture.version, extension
    ));
    if path.exists() && !force {
        bail!(
            "fixture {}@{} already exists; pass --force to replace it",
            fixture.name,
            fixture.version
        );
    }
    let json = serde_json::to_vec_pretty(fixture)?;
    let bytes = if config.encrypted {
        encrypt(&json, config)?
    } else {
        json
    };
    fs::write(&path, bytes).with_context(|| format!("write fixture {}", path.display()))?;
    Ok(path)
}

pub fn load_fixture(vault: &Path, config: &Config, name: &str, version: &str) -> Result<Fixture> {
    validate_label("name", name)?;
    validate_label("version", version)?;
    let extension = if config.encrypted { "vrf" } else { "json" };
    let path = vault
        .join("fixtures")
        .join(format!("{name}--{version}.{extension}"));
    let raw = fs::read(&path).with_context(|| {
        format!(
            "fixture {name}@{version} was not found in {}",
            vault.display()
        )
    })?;
    let bytes = if config.encrypted {
        decrypt(&raw, config)?
    } else {
        raw
    };
    serde_json::from_slice(&bytes).context("parse stored fixture")
}

pub fn list_fixtures(vault: &Path, config: &Config) -> Result<Vec<Fixture>> {
    let mut fixtures: Vec<Fixture> = Vec::new();
    for entry in fs::read_dir(vault.join("fixtures")).context("read fixtures directory")? {
        let path = entry?.path();
        if path.extension().and_then(|value| value.to_str())
            != Some(if config.encrypted { "vrf" } else { "json" })
        {
            continue;
        }
        let raw = fs::read(&path)?;
        let bytes = if config.encrypted {
            decrypt(&raw, config)?
        } else {
            raw
        };
        fixtures.push(serde_json::from_slice(&bytes).context("parse stored fixture")?);
    }
    fixtures.sort_by(|a, b| (&a.name, &a.version).cmp(&(&b.name, &b.version)));
    Ok(fixtures)
}

pub fn diff_contract(from: &Fixture, to: &Fixture) -> ContractDiff {
    let mut from_schema = BTreeMap::new();
    let mut to_schema = BTreeMap::new();
    let mut from_values = BTreeMap::new();
    let mut to_values = BTreeMap::new();
    flatten(&from.body, "$", &mut from_schema, &mut from_values);
    flatten(&to.body, "$", &mut to_schema, &mut to_values);
    ContractDiff {
        name: from.name.clone(),
        from: from.version.clone(),
        to: to.version.clone(),
        method_changed: from.method != to.method,
        path_changed: from.path != to.path,
        headers: compare_maps(&from.headers, &to.headers),
        schema: compare_maps(&from_schema, &to_schema),
        body: compare_maps(&from_values, &to_values),
    }
}

pub fn markdown_report(diff: &ContractDiff) -> String {
    let mut output = format!(
        "# Version Replay report: {}\n\n**Compared:** `{}` → `{}`  \n**Result:** {}\n\n",
        diff.name,
        diff.from,
        diff.to,
        if diff.has_changes() {
            "Contract changed"
        } else {
            "No differences"
        }
    );
    if diff.method_changed {
        output.push_str("- Request method changed\n");
    }
    if diff.path_changed {
        output.push_str("- Request path changed\n");
    }
    append_changes(&mut output, "Headers", &diff.headers);
    append_changes(&mut output, "Schema", &diff.schema);
    append_changes(&mut output, "Body values", &diff.body);
    output.push_str(
        "\n_Generated locally by Version Replay 0.1.0. Redacted values remain redacted._\n",
    );
    output
}

pub fn junit_report(diff: &ContractDiff) -> String {
    let failures = usize::from(diff.has_changes());
    let details = xml_escape(&markdown_report(diff));
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuite name=\"Version Replay\" tests=\"1\" failures=\"{failures}\"><testcase name=\"{} {} to {}\">{}</testcase></testsuite>\n",
        xml_escape(&diff.name),
        xml_escape(&diff.from),
        xml_escape(&diff.to),
        if failures == 1 {
            format!("<failure message=\"contract changed\">{details}</failure>")
        } else {
            String::new()
        }
    )
}

pub fn replay_fixture(fixture: &Fixture, destination: &str) -> Result<ReplayResult> {
    validate_loopback(destination)?;
    let mut request = ureq::request(&fixture.method, destination);
    for (header, value) in &fixture.headers {
        if !matches!(
            header.as_str(),
            "host" | "content-length" | "connection" | "transfer-encoding"
        ) {
            request = request.set(header, value);
        }
    }
    if !fixture.headers.contains_key("content-type") {
        request = request.set("content-type", "application/json");
    }
    let response = request.send_string(&serde_json::to_string(&fixture.body)?);
    let (status, text) = match response {
        Ok(response) => {
            let status = response.status();
            let text = response.into_string().unwrap_or_default();
            (status, text)
        }
        Err(ureq::Error::Status(status, response)) => {
            let text = response.into_string().unwrap_or_default();
            (status, text)
        }
        Err(error) => return Err(anyhow!("replay request failed: {error}")),
    };
    Ok(ReplayResult {
        name: fixture.name.clone(),
        version: fixture.version.clone(),
        destination: destination.to_string(),
        status,
        ok: (200..300).contains(&status),
        response_excerpt: text.chars().take(1024).collect(),
    })
}

pub fn validate_loopback(destination: &str) -> Result<()> {
    let url = Url::parse(destination).context("destination must be an absolute http URL")?;
    if !matches!(url.scheme(), "http" | "https") {
        bail!("destination must use http or https");
    }
    let host = url.host_str().unwrap_or_default();
    let normalized_host = host.trim_matches(['[', ']']);
    let allowed = host.eq_ignore_ascii_case("localhost")
        || normalized_host == "127.0.0.1"
        || normalized_host == "::1"
        || normalized_host
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());
    if !allowed {
        bail!("refusing non-loopback destination `{host}`; replay only to localhost");
    }
    Ok(())
}

pub fn activate_license(vault: &Path, config: &mut Config, token: &str) -> Result<LicenseCache> {
    if token.trim().is_empty() {
        bail!("license token cannot be empty");
    }
    let verdict = verify_token(token.trim())?;
    let cache = LicenseCache {
        token: token.trim().to_string(),
        valid: verdict.valid,
        reason: verdict.reason,
        checked_at: now(),
        expires_at: verdict.expires_at,
    };
    config.license = Some(cache.clone());
    save_config(vault, config)?;
    Ok(cache)
}

pub fn ensure_pro(vault: &Path, config: &mut Config) -> Result<()> {
    let Some(cache) = config.license.clone() else {
        bail!("Pro license required; buy or restore one from the Version Replay product site");
    };
    if cache.valid && now().saturating_sub(cache.checked_at) < 86_400 {
        return Ok(());
    }
    match activate_license(vault, config, &cache.token) {
        Ok(current) if current.valid => Ok(()),
        Ok(current) => bail!("license no longer active ({})", current.reason),
        Err(error) if cache.valid => {
            eprintln!("warning: license verification is offline; using the last valid verdict");
            let _ = error;
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn verify_token(token: &str) -> Result<VerifyResponse> {
    let base = env::var(BILLING_BASE_ENV).unwrap_or_else(|_| "https://api.sociobot.in".to_string());
    let mut url = Url::parse(&format!(
        "{}/api/v1/products/{PRODUCT_SLUG}/verify",
        base.trim_end_matches('/')
    ))?;
    url.query_pairs_mut().append_pair("license", token);
    let response = ureq::get(url.as_str())
        .timeout(std::time::Duration::from_secs(8))
        .call()
        .map_err(|error| anyhow!("could not verify license: {error}"))?;
    response.into_json().context("parse license verdict")
}

fn validate_label(label: &str, value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 100
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || "._-".contains(character))
    {
        bail!("{label} must use 1–100 letters, numbers, dots, dashes, or underscores");
    }
    Ok(())
}

fn passphrase() -> Result<String> {
    let value = env::var(PASSPHRASE_ENV).with_context(|| {
        format!("encrypted vault requires the {PASSPHRASE_ENV} environment variable")
    })?;
    if value.len() < 12 {
        bail!("{PASSPHRASE_ENV} must contain at least 12 characters");
    }
    Ok(value)
}

fn key(config: &Config) -> Result<[u8; 32]> {
    let salt = config
        .salt
        .as_ref()
        .context("encrypted vault has no salt")?;
    let salt = BASE64.decode(salt).context("invalid vault salt")?;
    let mut key = [0_u8; 32];
    Argon2::default()
        .hash_password_into(passphrase()?.as_bytes(), &salt, &mut key)
        .map_err(|error| anyhow!("derive encryption key: {error}"))?;
    Ok(key)
}

fn encrypt(plaintext: &[u8], config: &Config) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(&key(config)?).expect("32 byte AES key");
    let mut nonce_bytes = [0_u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext)
        .map_err(|_| anyhow!("encrypt fixture"))?;
    let mut output = b"VR01".to_vec();
    output.extend_from_slice(&nonce_bytes);
    output.extend_from_slice(&ciphertext);
    Ok(output)
}

fn decrypt(input: &[u8], config: &Config) -> Result<Vec<u8>> {
    if input.len() < 32 || &input[..4] != b"VR01" {
        bail!("fixture is not a supported encrypted Version Replay file");
    }
    let cipher = Aes256Gcm::new_from_slice(&key(config)?).expect("32 byte AES key");
    cipher
        .decrypt(Nonce::from_slice(&input[4..16]), &input[16..])
        .map_err(|_| anyhow!("could not decrypt fixture; check {PASSPHRASE_ENV}"))
}

fn redact_value(value: &mut Value, patterns: &[String], path: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                path.push(key.clone());
                if patterns.iter().any(|pattern| path_matches(pattern, path)) {
                    *value = Value::String(REDACTED.to_string());
                } else {
                    redact_value(value, patterns, path);
                }
                path.pop();
            }
        }
        Value::Array(array) => {
            for value in array {
                path.push("*".to_string());
                redact_value(value, patterns, path);
                path.pop();
            }
        }
        _ => {}
    }
}

fn path_matches(pattern: &str, path: &[String]) -> bool {
    fn recurse(pattern: &[&str], path: &[String]) -> bool {
        if pattern.is_empty() {
            return path.is_empty();
        }
        if pattern[0] == "**" {
            return recurse(&pattern[1..], path)
                || (!path.is_empty() && recurse(pattern, &path[1..]));
        }
        !path.is_empty()
            && (pattern[0] == "*" || pattern[0].eq_ignore_ascii_case(&path[0]))
            && recurse(&pattern[1..], &path[1..])
    }
    recurse(&pattern.split('.').collect::<Vec<_>>(), path)
}

fn flatten(
    value: &Value,
    path: &str,
    schema: &mut BTreeMap<String, String>,
    values: &mut BTreeMap<String, String>,
) {
    let kind = match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(number) if number.is_i64() || number.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    };
    schema.insert(path.to_string(), kind.to_string());
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                flatten(child, &format!("{path}.{key}"), schema, values);
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                flatten(child, &format!("{path}[{index}]"), schema, values);
            }
        }
        _ => {
            values.insert(path.to_string(), value.to_string());
        }
    }
}

fn compare_maps(from: &BTreeMap<String, String>, to: &BTreeMap<String, String>) -> Vec<Change> {
    let keys: BTreeSet<&String> = from.keys().chain(to.keys()).collect();
    keys.into_iter()
        .filter_map(|path| match (from.get(path), to.get(path)) {
            (None, Some(after)) => Some(Change {
                path: path.clone(),
                kind: ChangeKind::Added,
                before: None,
                after: Some(after.clone()),
            }),
            (Some(before), None) => Some(Change {
                path: path.clone(),
                kind: ChangeKind::Removed,
                before: Some(before.clone()),
                after: None,
            }),
            (Some(before), Some(after)) if before != after => Some(Change {
                path: path.clone(),
                kind: ChangeKind::Changed,
                before: Some(before.clone()),
                after: Some(after.clone()),
            }),
            _ => None,
        })
        .collect()
}

fn append_changes(output: &mut String, title: &str, changes: &[Change]) {
    output.push_str(&format!("## {title}\n\n"));
    if changes.is_empty() {
        output.push_str("No differences.\n\n");
        return;
    }
    output.push_str("| Change | Path | Before | After |\n| --- | --- | --- | --- |\n");
    for change in changes {
        output.push_str(&format!(
            "| {:?} | `{}` | `{}` | `{}` |\n",
            change.kind,
            change.path.replace('|', "\\|"),
            change.before.as_deref().unwrap_or("—").replace('|', "\\|"),
            change.after.as_deref().unwrap_or("—").replace('|', "\\|")
        ));
    }
    output.push('\n');
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fixture(version: &str, body: Value) -> Fixture {
        Fixture {
            name: "invoice".into(),
            version: version.into(),
            method: "POST".into(),
            path: "/hook".into(),
            headers: BTreeMap::from([("provider-version".into(), version.into())]),
            body,
            captured_at: 0,
        }
    }

    #[test]
    fn recursive_redaction_covers_nested_arrays() {
        let mut value = json!({"items": [{"customer": {"email": "person@example.com"}}]});
        redact_value(&mut value, &["**.email".into()], &mut Vec::new());
        assert_eq!(value["items"][0]["customer"]["email"], REDACTED);
    }

    #[test]
    fn diff_reports_schema_and_value_changes() {
        let old = fixture("2024-01", json!({"id": "one", "amount": 10}));
        let new = fixture(
            "2025-01",
            json!({"id": "two", "amount": "10", "status": "open"}),
        );
        let diff = diff_contract(&old, &new);
        assert!(diff.has_changes());
        assert!(diff.schema.iter().any(|change| change.path == "$.amount"));
        assert!(diff.schema.iter().any(|change| change.path == "$.status"));
        assert!(diff.body.iter().any(|change| change.path == "$.id"));
    }

    #[test]
    fn replay_rejects_public_hosts() {
        let error = validate_loopback("https://example.com/hook").unwrap_err();
        assert!(error.to_string().contains("non-loopback"));
        validate_loopback("http://localhost:3000/hook").unwrap();
        validate_loopback("http://[::1]:3000/hook").unwrap();
    }
}
