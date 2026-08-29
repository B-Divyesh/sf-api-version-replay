# Version Replay

Version Replay is a local CLI for engineers testing older webhook contracts before a provider API upgrade.

It saves redacted fixtures by provider version. Compare changes, replay requests to localhost, and export Markdown or JSON reports.

## Try the sample

The bundled demo creates a new temporary vault each time. It imports two payment webhook fixtures, compares them, replays both, and writes a report.

```sh
cargo run -- demo
```

The command prints the temporary vault and report paths. The browser sample is available at <https://api-version-replay.sociobot.in/?demo=1>.

## Install

Install the CLI from this checkout:

```sh
cargo install --path .
vr --help
```

Registry publishing and release archives are managed outside this repository.

## Use your fixtures

Create a vault, then import two saved webhook fixtures:

```sh
vr init
vr import --name payment-failed --version 2024-04-10 --file examples/old.json
vr import --name payment-failed --version 2025-02-24 --file examples/new.json
```

Compare the versions and export a Markdown report:

```sh
vr diff --name payment-failed --from 2024-04-10 --to 2025-02-24
vr report --name payment-failed --from 2024-04-10 --to 2025-02-24 --output replay-report.md
```

Replay one saved request to a local service:

```sh
vr replay --name payment-failed --version 2024-04-10 --to http://127.0.0.1:3000/webhooks/provider
```

Capture one incoming request on a loopback listener:

```sh
vr capture --name payment-failed --version 2024-04-10 --listen 127.0.0.1:9031 --once
```

Place `--json` before `demo` to receive JSON. A changed diff exits `3`; a rejected replay response exits `4`.

## Fixture format

A fixture can contain `method`, `path`, `headers`, and `body`. A plain JSON file becomes the body of a `POST /` fixture.

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

## Redaction and encryption

Default rules redact common identity, payment, cookie, and authorization fields before storage. Add rules when creating a vault:

```sh
vr init --redact-body 'payer.*.email' --redact-header 'x-internal-token'
```

Encrypted vaults use AES-256-GCM with an Argon2id-derived key. The passphrase stays in the environment and is not written to the vault.

```sh
export VERSION_REPLAY_PASSPHRASE='use-a-secret-from-your-password-manager'
vr init --encrypted
```

Replay accepts loopback destinations only. The CLI has no telemetry client and does not require provider credentials.

## Develop and verify

```sh
npm ci
npm test
npm run typecheck
npm run build
npm run test:browser -- http://127.0.0.1:4173/
```

Create the registry package without publishing:

```sh
cargo package --locked
```

## Deploy the documentation site

```sh
npm run build:site
```

Use `dist/site` as the static deployment directory. The factory manages infrastructure and DNS.

See the [privacy policy](https://api-version-replay.sociobot.in/privacy/) and [terms](https://api-version-replay.sociobot.in/terms/).

## License

[MIT](LICENSE)
