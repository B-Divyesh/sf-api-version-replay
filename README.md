# Version Replay

Version Replay (`vr`) is a local-first CLI for engineers who need to prove that an integration still accepts an older third-party webhook or HTTP contract before they upgrade it. It stores redacted, version-labelled fixtures on your laptop, compares their headers, values and JSON schemas, replays them only to localhost, and emits a reviewable Markdown or JSON report.

No provider account, public tunnel or credentials are required. There is no telemetry.

## Install

Build the single binary with Rust 1.82 or newer:

```sh
cargo install --path .
vr --help
```

Prebuilt release archives are intended to be attached by the factory after launch; this repository does not publish itself.

## Usage

Create a local vault. Common payment and identity fields and sensitive headers are redacted by default:

```sh
vr init
```

Import two saved webhook envelopes. An envelope may contain `method`, `path`, `headers`, and `body`; a plain JSON document is treated as the body of a `POST /` fixture.

```sh
vr import --name payment-failed --version 2024-04-10 --file examples/old.json
vr import --name payment-failed --version 2025-02-24 --file examples/new.json
```

Compare versions and save the pull-request artifact:

```sh
vr diff --name payment-failed --from 2024-04-10 --to 2025-02-24
vr report --name payment-failed --from 2024-04-10 --to 2025-02-24 --output replay-report.md
```

Replay the exact saved request to a local service. Version Replay refuses non-loopback destinations:

```sh
vr replay --name payment-failed --version 2024-04-10 --to http://127.0.0.1:3000/webhooks/provider
```

Capture the next request sent to a local listener, redact it, then stop:

```sh
vr capture --name payment-failed --version 2024-04-10 --listen 127.0.0.1:9031 --once
```

Add `--json` before the subcommand for stable machine-readable output. `diff` exits `3` when it finds a difference; `replay` exits `4` when localhost returns a non-2xx response; operational failures exit `1`.

### Encrypted vault

Set a passphrase in an environment variable and initialize with encryption. Fixture contents use Argon2id key derivation and AES-256-GCM authenticated encryption; vault metadata stays readable.

```sh
export VERSION_REPLAY_PASSPHRASE='use-a-secret-from-your-password-manager'
vr init --encrypted
```

The passphrase is never written to disk. Losing it makes the fixture files unrecoverable.

### Custom redaction

Body paths use dot notation and `*` for one segment. Header names are case-insensitive.

```sh
vr init --redact-body 'payer.*.email' --redact-header 'x-internal-token'
```

Redaction happens before a fixture is written, including inside encrypted vaults.

### One-time Pro unlock

The free CLI includes local capture, redaction, encrypted storage, individual replay, contract diff and report export. A one-time Pro license adds batch replay across every saved version and JUnit output for CI. Buy through the Sociobot-hosted checkout on the product site, then restore the token on any device:

```sh
vr license activate YOUR_LICENSE_TOKEN
vr license status
vr batch --name payment-failed --to http://127.0.0.1:3000/webhooks/provider
```

License checks are cached for 24 hours and never block free commands. Sociobot/Dodo is the merchant of record.

## Fixture format

```json
{
  "method": "POST",
  "path": "/webhooks/provider",
  "headers": {
    "content-type": "application/json",
    "provider-version": "2024-04-10"
  },
  "body": {
    "type": "payment.failed",
    "data": { "id": "evt_local_001" }
  }
}
```

## Develop and verify

```sh
npm ci
npm test
npm run typecheck
npm run build
```

`npm test` runs Rust unit/integration tests plus site tests. `npm run build` creates the release binary in `dist/bin/vr` and the static landing/docs site in `dist/site/`. To work on the site, use `npm run dev`; to build only it, use `npm run build:site`.

Run the browser acceptance sweep against a built deployment or local preview. It checks desktop and 390 px mobile interaction, keyboard comparison, reduced motion, axe serious/critical findings, same-origin browsing, and an immediate offline reload:

```sh
npm run test:browser -- https://api-version-replay.sociobot.in/
```

Create the registry artifact without publishing:

```sh
cargo package
```

## Privacy and security

Everything in the vault stays local. Replays are restricted to loopback hosts, default redaction is conservative, and the optional vault is encrypted at rest. See the site’s [privacy policy](https://api-version-replay.sociobot.in/privacy/) and [terms](https://api-version-replay.sociobot.in/terms/).

Security reports can be opened as private advisories in the repository. Do not attach real webhook payloads.

## License

MIT. See [LICENSE](LICENSE).
