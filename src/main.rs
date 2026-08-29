use anyhow::{Context, Result, anyhow, bail};
use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    net::TcpListener,
    path::{Path, PathBuf},
    process::ExitCode,
    thread,
};
use tiny_http::{Response, Server, StatusCode};
use version_replay::{
    ContractDiff, Fixture, default_config, diff_contract, init_vault, list_fixtures, load_config,
    load_fixture, markdown_report, parse_fixture_file, redact_fixture, replay_fixture,
    store_fixture,
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
    /// Emit JSON.
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
    /// Run the complete workflow with bundled sample data in a temporary vault.
    Demo,
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
    /// Write a report for two versions.
    Report(ReportArgs),
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
    /// JSON body or request fixture file.
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
        Command::Demo => demo(cli),
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
            let compare = CompareArgs {
                name: args.name.clone(),
                from: args.from.clone(),
                to: args.to.clone(),
            };
            let diff = load_diff(&cli.vault, &compare)?;
            let report = match args.format {
                ReportFormat::Markdown => markdown_report(&diff),
                ReportFormat::Json => serde_json::to_string_pretty(&diff)?,
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
    }
}

fn demo(cli: &Cli) -> Result<u8> {
    let demo_root = std::env::temp_dir().join(format!(
        "version-replay-demo-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));
    let vault = demo_root.join("vault");
    fs::create_dir_all(&demo_root)?;
    let old_path = demo_root.join("payment-failed-2024-04-10.json");
    let new_path = demo_root.join("payment-failed-2025-02-24.json");
    fs::write(&old_path, include_str!("../examples/old.json"))?;
    fs::write(&new_path, include_str!("../examples/new.json"))?;

    let config = default_config(false, Vec::new(), Vec::new());
    init_vault(&vault, &config)?;
    for (version, path) in [("2024-04-10", &old_path), ("2025-02-24", &new_path)] {
        let mut fixture = parse_fixture_file(path, "payment-failed", version)?;
        redact_fixture(&mut fixture, &config);
        store_fixture(&vault, &config, &fixture, false)?;
    }

    let old = load_fixture(&vault, &config, "payment-failed", "2024-04-10")?;
    let new = load_fixture(&vault, &config, "payment-failed", "2025-02-24")?;
    let diff = diff_contract(&old, &new);

    let listener = TcpListener::bind("127.0.0.1:0")?;
    let address = listener.local_addr()?;
    let receiver = thread::spawn(move || -> Result<Vec<String>> {
        let mut requests = Vec::new();
        for _ in 0..2 {
            let (mut stream, _) = listener.accept()?;
            let mut request = Vec::new();
            let mut chunk = [0_u8; 4096];
            loop {
                let read = stream.read(&mut chunk)?;
                if read == 0 {
                    break;
                }
                request.extend_from_slice(&chunk[..read]);
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
            requests.push(String::from_utf8_lossy(&request).into_owned());
            stream.write_all(
                b"HTTP/1.1 204 No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )?;
        }
        Ok(requests)
    });

    let destination = format!("http://{address}/webhooks/provider");
    let old_replay = replay_fixture(&old, &destination)?;
    let new_replay = replay_fixture(&new, &destination)?;
    let requests = receiver
        .join()
        .map_err(|_| anyhow!("sample receiver stopped unexpectedly"))??;
    let report_path = demo_root.join("version-replay-report.md");
    fs::write(&report_path, markdown_report(&diff))?;

    if cli.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "ok": true,
                "demo": true,
                "vault": vault,
                "fixtures": ["payment-failed@2024-04-10", "payment-failed@2025-02-24"],
                "redacted": true,
                "changed": diff.has_changes(),
                "change_counts": {"headers": diff.headers.len(), "schema": diff.schema.len(), "body": diff.body.len()},
                "replays": [old_replay, new_replay],
                "receiver_requests": requests.len(),
                "report": report_path
            }))?
        );
    } else {
        println!("Version Replay sample");
        println!("Imported 2 redacted fixtures into {}", vault.display());
        println!(
            "Compared 2024-04-10 → 2025-02-24: {} contract changes",
            diff.headers.len() + diff.schema.len() + diff.body.len()
        );
        println!("Replayed 2024-04-10 → HTTP {}", old_replay.status);
        println!("Replayed 2025-02-24 → HTTP {}", new_replay.status);
        println!("Wrote Markdown report to {}", report_path.display());
        println!("Sample data is isolated. Run `vr demo` again to reset it.");
    }
    Ok(0)
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
