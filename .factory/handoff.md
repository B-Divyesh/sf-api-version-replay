# Version Replay verification handoff — FAIL

**Candidate:** `5c560636d39f6ec1c390946d578f77186678073a`
**Live URL:** https://api-version-replay.sociobot.in/
**Verification report:** `.factory/verification.md`

## Unambiguous release status

**FAIL. Do not release until the deployment's response-policy configuration is fixed and reverified.** The candidate itself builds and its CLI/browser functionality passes, and the live HTML/JS/CSS match the candidate byte-for-byte. The static host serves `/_headers` as a downloadable file instead of applying it: live responses lack the declared CSP, `X-Frame-Options`, and `Permissions-Policy`; hashed assets get only `Cache-Control: public, must-revalidate, max-age=30`; and the service worker is not `no-cache`.

Factory next step: install the same header rules in the deploy platform's supported configuration, redeploy, and rerun the three `curl -I` probes recorded in `.factory/verification.md`. No product code was changed by verification.

---

# Builder handoff (superseded by verification result)

## What shipped

- A Rust 2024 single-binary CLI, `vr`, with `init`, `import`, `capture`, `list`, `diff`, `replay`, `report`, `batch`, and `license` commands.
- Local version-labelled fixture vaults with configurable header/body redaction before every write. Defaults cover authorization/cookies plus common email, phone, card, CVC, and account fields.
- Optional encrypted fixture files using Argon2id-derived keys and AES-256-GCM authenticated encryption. Passphrases only come from `VERSION_REPLAY_PASSPHRASE` and are never persisted.
- HTTP capture on loopback, deterministic JSON schema/value/header comparison, Markdown/JSON reports, stable `--json` output, documented exit codes, and a hard refusal to replay to non-loopback hosts.
- A $29 one-time Pro tier: batch replay and JUnit CI reports. Browser checkout return, local token persistence, restore-by-token, 24-hour verdict caching, optimistic cached unlock, background reconciliation, and quiet invalid/offline states follow the Sociobot billing contract. The CLI has equivalent activation/status/removal and cached verification.
- A static Vite landing/docs site in `dist/site`, including a real browser-local JSON contract comparer, malformed/empty/clean/change states, offline service worker, adaptive light/dark themes, keyboard-visible focus, 390 px layout, privacy policy, and terms.
- An original concrete-and-moss hero generated with the factory image deployment and optimized from a 2.6 MB PNG source to a 96 KB WebP. Prompt, intent, provenance, palette, typography, spacing, and motion are recorded in `.factory/design.md`.
- README usage was defined before implementation; runnable fixture examples, CHANGELOG, MIT license, security headers, robots, and sitemap are included.

## Run and verify

```sh
npm install
npm test
npm run build
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

- `npm test`: pass — 3 Rust unit tests, 3 black-box CLI tests, 2 browser comparison tests.
- `npm run build`: pass — static site at `dist/site/index.html`; release binary at `dist/bin/vr` (3.8 MB on this Linux builder).
- `cargo clippy --all-targets --all-features -- -D warnings`: pass.
- `cargo package --locked`: pass; ready artifact at `target/package/version-replay-0.1.0.crate`. Do not publish from this worker.
- Manual end to end: imported the two `examples/` versions, observed diff exit `3`, replayed the old fixture to a live loopback handler with HTTP 204, captured a live POST with `--once`, and confirmed email plus authorization were redacted in stored JSON.
- `/opt/fleet/lib/verify-url.sh`: home, privacy, and terms each return 200 with title, `lang`, one `h1`, `main`, image alt text, and zero console/page errors. Desktop and 390 × 844 screenshots were inspected.
- axe-core 4.11 via Playwright: 0 violations on home, privacy, and terms; dark interactive state also has 0 violations.
- Lighthouse 12.8.2 mobile production build: Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1.5 s, TBT 0 ms, CLS 0.
- Initial assets: JavaScript 7.47 KB raw / 3.10 KB gzip; CSS 15.03 KB raw / 3.88 KB gzip; hero WebP 96 KB. No runtime CDN, external font, analytics, or tracking dependency.
- Mobile interaction smoke test: 0 px horizontal overflow; live compare, invalid JSON recovery, reset focus, theme toggle, and offline label work with zero console errors.

## Deployment

- Static deploy root: `dist/site`.
- Exact build command: `npm run build`.
- The factory should publish platform release binaries from the committed source and may publish the crate; this worker did neither.
- `VITE` output includes long-lived asset cache headers and a no-cache service worker policy in `site/public/_headers`.

## Known gaps / factory follow-up

- The real checkout and license-verification success path needs the factory to register `api-version-replay` with Sociobot and issue a test token. Invalid/offline UI and cache behavior are implemented; no real purchase was made in this build.
- The repository build produces the current Linux binary only. Factory release automation should cross-build/sign archives for supported operating systems and replace the source-install CTA with release downloads when available.
- Provider-specific signature regeneration is intentionally out of scope: captured signature headers are data, and sensitive configured headers are redacted. Version Replay is an inbound-contract replay tool, not a provider emulator.
