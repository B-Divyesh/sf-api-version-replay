# Demo sandbox

## Browser

Open <https://api-version-replay.sociobot.in/?demo=1>.

The first screen shows the completed `vr demo` workflow: two redacted imports, five contract changes, two loopback HTTP 204 responses, and the Markdown report path. The editable JSON comparator follows as a secondary tool.

The persistent banner identifies demo mode. `Reset demo` restores and recomputes the editable sample. `Start for real` returns to the normal landing page.

Demo mode keeps its state in memory. It never reads or writes `localStorage`, IndexedDB, cookies, or a CLI vault. It makes no billing request. Leaving or reloading discards edits.

## CLI

Run:

```sh
vr demo
```

The command creates a unique directory under the operating system's temporary directory. It imports the bundled old and new payment webhook fixtures into a new vault, applies default redaction, compares them, starts a loopback-only receiver, replays both versions, and writes `version-replay-report.md`.

The command prints the exact vault and report paths. Run it again for a clean sample. It never reads or writes the default `.version-replay` vault.
