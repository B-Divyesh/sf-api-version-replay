# Polish round 2 handoff — Version Replay

## Outcome

All cumulative findings from review 1 and review 2 are closed. The repair is pushed on `main` at `5f75784` and deployed as Azure Static Web Apps deployment `f1d1a05a-0252-4073-b8f8-57af22bd3a85`.

Live site: <https://api-version-replay.sociobot.in/>

Direct isolated demo: <https://api-version-replay.sociobot.in/?demo=1>

## What changed

- The one-click demo now starts with the completed CLI workflow, including redacted imports, five contract changes, two localhost HTTP 204 results, and the Markdown report path.
- The JSON editor remains available below as a clearly secondary browser-only comparison.
- Demo mode still uses no browser storage, cookies, external requests, or billing state. Reset restores the sample; Start for real leaves demo mode.
- Added `primary-demo-workflow`, `demo-output-paths`, and `vault-directory` to `.factory/claims.json` with one tagged observable test each. The inventory now contains 20 claims.
- Reworded the README build line as an installation instruction.
- Made the Back-navigation gate deterministic and repeated it three times.
- Added pre-render demo layout selection. This removed the demo layout shift without changing the concrete/moss identity.
- Updated the catalog line to the 96-character verb-first sentence: `Compare saved webhook versions and replay each contract against localhost before an API upgrade.`

## Exact verification

Clean clone `/tmp/api-version-replay-round2.BWnHI7` at `5f75784`:

- `npm ci`: 58 packages, 0 vulnerabilities.
- `npm test`: 3 Rust unit tests, 15 Rust claim tests, 3 CLI integration tests, 5 Vitest tests, and all 20 individually dispatched claim commands passed.
- `npm run typecheck`: passed.
- `npm run build`: produced `dist/bin/vr` and `dist/site/`.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo package --locked`: packaged and verified `version-replay 0.1.0` without publishing.
- `npm run test:browser -- http://127.0.0.1:4273/`: passed desktop, 390 px, keyboard, repeated routing/focus, accessibility, privacy, and offline checks.

Local and live evidence:

- Built initial assets: app JS 2.79 KB gzip; CSS 4.52 KB gzip.
- Local Lighthouse: 100 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.7 s, CLS 0, TBT 0 ms.
- Live Lighthouse: 100/100/100/100; LCP 1.4 s, CLS 0, TBT 0 ms. Report: `.factory/live-round2/lighthouse.json`.
- Worker verifier: home and demo each have one H1, `lang=en`, a main landmark, complete image alt text, labelled buttons, and zero console errors. Reports: `.factory/live-round2/home/verify.json` and `.factory/live-round2/demo/verify.json`.
- Live browser suite: passed with zero serious/critical axe violations and no 390 px overflow.
- Cold demo instrumentation: required transcript fields all present; zero storage reads/writes, cookies, external requests, fixture uploads, console errors, or horizontal overflow; Reset restored the fixture.
- Live routes: home, demo, Privacy, Terms, robots, and sitemap return 200; a random unknown route returns 404.
- Live security headers: CSP, `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, strict referrer policy, permissions policy, and HSTS are present.

## How to verify

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
npm run test:browser -- https://api-version-replay.sociobot.in/
```

Every claim command is listed in `.factory/claims.json`. The finding-by-finding record is `.factory/polish-2.md`.

## Known gaps and next steps

None. Registry publication remains factory-owned and was intentionally not performed.
