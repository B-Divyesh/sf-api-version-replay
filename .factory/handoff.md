# Perfection-loop round 3 handoff — Version Replay

## Outcome

**PASS.** Every finding in `.factory/review-1.md`, `.factory/review-2.md`, and `.factory/review-3.md` is resolved. The deployed site is <https://api-version-replay.sociobot.in/>.

The round-3 product changes are commits `54d8899` and `d334183`. Deployment `bbb9d98b-fb64-4764-aece-513a7d528b75` serves the built `d3341837b27250fd1a19185b9c80aefc4497eab2` site artifact. `.factory/polish-3.md` maps every cumulative finding ID to its change and evidence.

## What changed

- Corrected the CLI-guide link to the real `#use-your-fixtures` README section.
- Added an offline source guard and a real-browser external-fragment check.
- Removed the unlisted `no account system` privacy promise while retaining the tested telemetry statement.
- Replaced the unsupported `short-lived` hosting-log assurance with neutral provider-policy wording.
- Bumped the service-worker cache to `version-replay-shell-v5` so returning visitors receive the corrections.
- Updated the 78-character, verb-first catalog description and refreshed the copy audit.
- Limited the publishable Rust crate to runtime source, bundled examples, tests, and required project documents.

The existing concrete/moss identity, first-screen wording, isolated one-click demo, CLI workflow, real routing, route titles, focus restoration, 404, legal pages, mobile layout, and static deployment class were preserved.

## Clean-clone verification

Final clone: `/tmp/api-version-replay-polish3-final.IsW4dD` at `d3341837b27250fd1a19185b9c80aefc4497eab2`.

- `npm ci`: pass; 0 reported vulnerabilities.
- Every one of the 20 `.factory/claims.json` commands: pass when invoked separately.
- `npm test`: pass; 3 Rust unit tests, 15 Rust claim tests, 3 CLI integration tests, 6 Vitest tests, and all 5 browser claim scenarios.
- `npm run typecheck`: pass.
- `npm run build`: pass; created `dist/bin/vr` and `dist/site/`.
- `npm run test:browser -- http://127.0.0.1:4174/`: pass, including external-fragment validation and zero serious/critical axe violations.
- `cargo fmt --check`: pass.
- `cargo clippy --all-targets --all-features -- -D warnings`: pass.
- `cargo package --locked`: pass; 13 files, 101.6 KiB unpacked and 27.2 KiB compressed.
- Static budgets: initial JS files total 8,560 B raw; CSS 18,222 B raw; hero WebP 96,986 B.

## Live verification

- `npm run test:browser -- https://api-version-replay.sociobot.in/`: pass for keyboard interaction, 390 px no-overflow, reduced motion, three Back/focus cycles, offline reload, Home/Demo/Privacy/Terms/404 axe scans, and the external GitHub fragment.
- Cold instrumented `/?demo=1`: pass for the completed CLI output, banner, memory-only isolation, unchanged real storage, same-origin requests, Reset demo, offline reload, and Start for real.
- Worker verifier: Home, Demo, Privacy, and Terms each have the correct title, `lang=en`, one H1, a main landmark, alt text, labelled buttons, and zero console errors.
- Route sweep: Home, Demo, Privacy, Terms, robots, sitemap, icons, and product art return 200. `/round-3-missing-page` returns 404 with the designed recovery page.
- Privacy: live copy contains the tested telemetry statement and provider-policy wording; `no account system` and `short-lived` are absent.
- Guide link: live href ends in `#use-your-fixtures`; the rendered GitHub page contains `#user-content-use-your-fixtures`.
- Response policy: live CSP, HSTS, frame denial, `nosniff`, referrer policy, and permissions policy are present.
- Artifact parity: live and built Home, Privacy, Terms, and service-worker files have matching SHA-256 hashes.
- Lighthouse report `.factory/polish-3-live/lighthouse.json`: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; FCP 1.0 s, LCP 1.4 s, TBT 10 ms, CLS 0. The report was written before the known final full-page-screenshot tab crash.

Screenshots and verifier JSON are in `.factory/polish-3-live/home/`, `demo/`, `privacy/`, and `terms/`; the live 404 is `.factory/polish-3-live/404.png`.

## Run and verify

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

## Known gaps and next steps

No product, claim, accessibility, privacy, routing, deployment, or review gap remains. Registry publication stays with the factory; do not publish the crate from this repository.
