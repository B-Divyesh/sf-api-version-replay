use anyhow::{Context, Result, anyhow, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};
use tiny_http::{Response, Server, StatusCode};
use version_replay::{
    ContractDiff, Fixture, activate_license, default_config, diff_contract, ensure_pro, init_vault,
    junit_report, list_fixtures, load_config, load_fixture, markdown_report, parse_fixture_file,
    redact_fixture, replay_fixture, save_config, store_fixture,
};

#[derive(Debug, Parser)]
#[command(
    name = "vr",
    version,
    about = "Replay versioned HTTP contracts against localhost",
    long_about = "Store redacted, version-labelled HTTP fixtures locally, compare their wire contracts, and replay exact requests only to loopback services. No provider credentials or public tunnel required.",
    after_help = "Exit codes: 0 success, 1 operational error, 3 differences found, 4 replay received a non-2xx response.\nDocs: https://api-version-replay.sociobot.in"
)]
struct Cli {
    /// Emit stable machine-readable JSON.
    #[arg(long, global = true)]
    json: bool,

    /// Vault directory.
    #[arg(long, global = true, default_value = ".version-replay")]
    vault: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create a local fixture vault.
    Init(InitArgs),
    /// Import and redact a JSON fixture.
    Import(ImportArgs),
    /// Capture incoming localhost requests into the vault.
    Capture(CaptureArgs),
    /// List saved fixture versions.
    List,
    /// Compare headers, body values, and JSON schema between versions.
    Diff(CompareArgs),
    /// Replay one saved request to a loopback URL.
    Replay(ReplayArgs),
    /// Write a review artifact for two versions.
    Report(ReportArgs),
    /// Replay every saved version of a fixture (Pro).
    Batch(BatchArgs),
    /// Activate or inspect the one-time Pro license.
    License(LicenseArgs),
}

#[derive(Debug, Args)]
struct InitArgs {
    /// Encrypt every fixture at rest (passphrase comes from VERSION_REPLAY_PASSPHRASE).
    #[arg(long)]
    encrypted: bool,

    /// Add a body redaction path; supports dot notation, *, and **.
    #[arg(long = "redact-body")]
    redact_body: Vec<String>,

    /// Add a case-insensitive header name to redact.
    #[arg(long = "redact-header")]
    redact_header: Vec<String>,
}

#[derive(Debug, Args)]
struct ImportArgs {
    /// Stable fixture scenario name.
    #[arg(long)]
    name: String,
    /// Provider API version label.
    #[arg(long)]
    version: String,
    /// JSON body or request envelope file.
    #[arg(long)]
    file: PathBuf,
    /// Replace an existing fixture with the same name and version.
    #[arg(long)]
    force: bool,
}

#[derive(Debug, Args)]
struct CaptureArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    version: String,
    /// Loopback address and port to listen on.
    #[arg(long, default_value = "127.0.0.1:9031")]
    listen: String,
    /// Stop after the first captured request.
    #[arg(long)]
    once: bool,
}

#[derive(Debug, Args)]
struct CompareArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String,
}

#[derive(Debug, Args)]
struct ReplayArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    version: String,
    /// Exact loopback URL that should receive the request.
    #[arg(long)]
    to: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum ReportFormat {
    Markdown,
    Json,
    Junit,
}

#[derive(Debug, Args)]
struct ReportArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    from: String,
    #[arg(long)]
    to: String,
    #[arg(long, value_enum, default_value = "markdown")]
    format: ReportFormat,
    /// Write to this file instead of stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct BatchArgs {
    #[arg(long)]
    name: String,
    #[arg(long)]
    to: String,
}

#[derive(Debug, Args)]
struct LicenseArgs {
    #[command(subcommand)]
    action: LicenseAction,
}

#[derive(Debug, Subcommand)]
enum LicenseAction {
    /// Verify and save a license token on this device.
    Activate { token: String },
    /// Show the locally cached license state.
    Status,
    /// Remove the saved license token from this device.
    Remove,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            if cli.json {
                println!("{}", json!({"ok": false, "error": format!("{error:#}")}));
            } else {
                eprintln!("error: {error:#}");
            }
            ExitCode::from(1)
        }
    }
}

fn run(cli: &Cli) -> Result<u8> {
    match &cli.command {
        Command::Init(args) => {
            let config = default_config(
                args.encrypted,
                args.redact_body.clone(),
                args.redact_header.clone(),
            );
            init_vault(&cli.vault, &config)?;
            emit(
                cli.json,
                &json!({"ok": true, "vault": cli.vault, "encrypted": config.encrypted}),
                &format!(
                    "Initialized {} vault at {}",
                    if config.encrypted {
                        "encrypted"
                    } else {
                        "local"
                    },
                    cli.vault.display()
                ),
            )?;
            Ok(0)
        }
        Command::Import(args) => {
            let config = load_config(&cli.vault)?;
            let mut fixture = parse_fixture_file(&args.file, &args.name, &args.version)?;
            redact_fixture(&mut fixture, &config);
            let path = store_fixture(&cli.vault, &config, &fixture, args.force)?;
            emit(
                cli.json,
                &json!({"ok": true, "fixture": fixture, "stored_at": path}),
                &format!(
                    "Stored {}@{} with {} redaction rules",
                    fixture.name,
                    fixture.version,
                    config.redact_body.len() + config.redact_headers.len()
                ),
            )?;
            Ok(0)
        }
        Command::Capture(args) => capture(cli, args),
        Command::List => {
            let config = load_config(&cli.vault)?;
            let fixtures = list_fixtures(&cli.vault, &config)?;
            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({"ok": true, "fixtures": fixtures}))?
                );
            } else if fixtures.is_empty() {
                println!(
                    "No fixtures yet. Import one with `vr import --name NAME --version VERSION --file FILE`."
                );
            } else {
                println!("NAME\tVERSION\tMETHOD\tPATH");
                for fixture in fixtures {
                    println!(
                        "{}\t{}\t{}\t{}",
                        fixture.name, fixture.version, fixture.method, fixture.path
                    );
                }
            }
            Ok(0)
        }
        Command::Diff(args) => {
            let diff = load_diff(&cli.vault, args)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&diff)?);
            } else {
                print!("{}", markdown_report(&diff));
            }
            Ok(if diff.has_changes() { 3 } else { 0 })
        }
        Command::Replay(args) => {
            let config = load_config(&cli.vault)?;
            let fixture = load_fixture(&cli.vault, &config, &args.name, &args.version)?;
            let result = replay_fixture(&fixture, &args.to)?;
            emit(
                cli.json,
                &result,
                &format!(
                    "{} {}@{} → HTTP {}",
                    if result.ok { "PASS" } else { "FAIL" },
                    result.name,
                    result.version,
                    result.status
                ),
            )?;
            Ok(if result.ok { 0 } else { 4 })
        }
        Command::Report(args) => {
            let mut config = load_config(&cli.vault)?;
            if matches!(args.format, ReportFormat::Junit) {
                ensure_pro(&cli.vault, &mut config)?;
            }
            let compare = CompareArgs {
                name: args.name.clone(),
                from: args.from.clone(),
                to: args.to.clone(),
            };
            let diff = load_diff(&cli.vault, &compare)?;
            let report = match args.format {
                ReportFormat::Markdown => markdown_report(&diff),
                ReportFormat::Json => serde_json::to_string_pretty(&diff)?,
                ReportFormat::Junit => junit_report(&diff),
            };
            if let Some(path) = &args.output {
                fs::write(path, report.as_bytes())
                    .with_context(|| format!("write report {}", path.display()))?;
                emit(
                    cli.json,
                    &json!({"ok": true, "output": path, "changed": diff.has_changes()}),
                    &format!("Wrote replay report to {}", path.display()),
                )?;
            } else {
                print!("{report}");
            }
            Ok(0)
        }
        Command::Batch(args) => {
            let mut config = load_config(&cli.vault)?;
            ensure_pro(&cli.vault, &mut config)?;
            let fixtures: Vec<_> = list_fixtures(&cli.vault, &config)?
                .into_iter()
                .filter(|fixture| fixture.name == args.name)
                .collect();
            if fixtures.is_empty() {
                bail!("no fixtures named `{}`", args.name);
            }
            let mut results = Vec::new();
            for fixture in fixtures {
                results.push(replay_fixture(&fixture, &args.to)?);
            }
            let all_ok = results.iter().all(|result| result.ok);
            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&json!({"ok": all_ok, "results": results}))?
                );
            } else {
                for result in &results {
                    println!(
                        "{}\t{}@{}\tHTTP {}",
                        if result.ok { "PASS" } else { "FAIL" },
                        result.name,
                        result.version,
                        result.status
                    );
                }
            }
            Ok(if all_ok { 0 } else { 4 })
        }
        Command::License(args) => license(cli, args),
    }
}

fn capture(cli: &Cli, args: &CaptureArgs) -> Result<u8> {
    let host = args.listen.split(':').next().unwrap_or_default();
    if !matches!(host, "127.0.0.1" | "localhost" | "::1" | "[::1]") {
        bail!("capture listener must bind to localhost, not `{host}`");
    }
    let config = load_config(&cli.vault)?;
    let server =
        Server::http(&args.listen).map_err(|error| anyhow!("start capture listener: {error}"))?;
    if !cli.json {
        eprintln!(
            "Listening at http://{} — send a request or press Ctrl+C",
            args.listen
        );
    }
    let mut sequence = 0_u32;
    for mut request in server.incoming_requests() {
        sequence += 1;
        let version = if sequence == 1 {
            args.version.clone()
        } else {
            format!("{}-{sequence}", args.version)
        };
        let mut body_text = String::new();
        request.as_reader().read_to_string(&mut body_text)?;
        let body = serde_json::from_str(&body_text).unwrap_or(Value::String(body_text));
        let headers = request
            .headers()
            .iter()
            .map(|header| {
                (
                    header.field.as_str().as_str().to_ascii_lowercase(),
                    header.value.as_str().to_string(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut fixture = Fixture {
            name: args.name.clone(),
            version,
            method: request.method().as_str().to_string(),
            path: request.url().to_string(),
            headers,
            body,
            captured_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        redact_fixture(&mut fixture, &config);
        let result = store_fixture(&cli.vault, &config, &fixture, false);
        match result {
            Ok(path) => {
                request.respond(
                    Response::from_string("captured by Version Replay\n")
                        .with_status_code(StatusCode(202)),
                )?;
                emit(
                    cli.json,
                    &json!({"ok": true, "fixture": fixture, "stored_at": path}),
                    &format!("Captured {}@{}", fixture.name, fixture.version),
                )?;
            }
            Err(error) => {
                request.respond(
                    Response::from_string(format!("capture failed: {error}\n"))
                        .with_status_code(StatusCode(500)),
                )?;
                return Err(error);
            }
        }
        if args.once {
            break;
        }
    }
    Ok(0)
}

fn license(cli: &Cli, args: &LicenseArgs) -> Result<u8> {
    let mut config = load_config(&cli.vault)?;
    match &args.action {
        LicenseAction::Activate { token } => {
            let cache = activate_license(&cli.vault, &mut config, token)?;
            emit(
                cli.json,
                &json!({"ok": cache.valid, "valid": cache.valid, "reason": cache.reason, "expires_at": cache.expires_at}),
                if cache.valid {
                    "Pro license active"
                } else {
                    "License no longer active"
                },
            )?;
            Ok(if cache.valid { 0 } else { 1 })
        }
        LicenseAction::Status => {
            let value = config.license.as_ref().map(|cache| {
                json!({
                    "valid": cache.valid,
                    "reason": cache.reason,
                    "checked_at": cache.checked_at,
                    "expires_at": cache.expires_at
                })
            });
            emit(
                cli.json,
                &json!({"ok": true, "license": value}),
                match config.license.as_ref() {
                    Some(cache) if cache.valid => "Pro license active",
                    Some(_) => "License no longer active",
                    None => "No Pro license saved",
                },
            )?;
            Ok(0)
        }
        LicenseAction::Remove => {
            config.license = None;
            save_config(&cli.vault, &config)?;
            emit(
                cli.json,
                &json!({"ok": true, "removed": true}),
                "Removed the saved license from this device",
            )?;
            Ok(0)
        }
    }
}

fn load_diff(vault: &Path, args: &CompareArgs) -> Result<ContractDiff> {
    let config = load_config(vault)?;
    let from = load_fixture(vault, &config, &args.name, &args.from)?;
    let to = load_fixture(vault, &config, &args.name, &args.to)?;
    Ok(diff_contract(&from, &to))
}

fn emit<T: Serialize>(json_output: bool, value: &T, human: &str) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(value)?);
    } else {
        println!("{human}");
    }
    Ok(())
}
