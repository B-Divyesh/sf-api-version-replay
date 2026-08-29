# Version Replay polish 1 handoff

## Outcome

All F-1-1 through F-1-98 findings from `.factory/review-1.md` are implemented. The per-finding change and evidence map is `.factory/polish-1.md`.

The product remains a Rust `vr` single-binary CLI with a Vite static documentation site. The concrete-and-moss visual system remains intact.

## What changed

- Rewrote the first screen around the job, audience, one sample-data action, outcome, and three facts.
- Added `vr demo`. It uses bundled fixtures, a unique temporary vault, a loopback receiver, both replays, and a Markdown report.
- Added isolated `?demo=1` browser mode with a persistent banner, completed result, reset, and exit. It never accesses browser storage.
- Added `.factory/claims.json` with 17 claims and exactly one tagged clean-sandbox test per claim.
- Removed the dead checkout and all paid/license claims and commands. There is no purchase action until the factory registers a working product.
- Added route metadata, shared navigation/footer, focus and history restoration, a designed 404, icons, social card, and sitemap updates.
- Rewrote landing, README, legal, status, error, and action copy around `webhook fixture`, `contract changes`, and `report`.
- Added responsive demo/result layouts, contrast fixes, route-wide axe checks, and privacy/offline tests.

## Local verification

Run from `/work/repo`:

```sh
npm ci
npm test
npm run typecheck
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
npm run build
cargo package --locked
npm run test:browser -- http://127.0.0.1:4173/
```

Local results on 2026-08-29:

- Rust: 3 unit, 3 original CLI integration, and 13 claim tests passed.
- Site: 5 Vitest tests passed, including claims-inventory and static-policy validation.
- Claims: all 17 declared commands passed individually.
- Browser: home/demo/privacy/terms/404 passed keyboard, focus/history, reduced-motion, offline, 390 px overflow, and console checks.
- Accessibility: Playwright axe WCAG A/AA reported 0 serious or critical findings on every route.
- URL verifier: title, `lang`, one H1, main, alt text, labels, and console checks passed.
- Build: `dist/bin/vr` and `dist/site/` produced; `cargo package --locked --allow-dirty` packaged and verified 64 files.
- Budgets: initial app JS 6,802 B raw / 2,756 B gzip; CSS 17,944 B raw / 4,492 B gzip; hero WebP 96,986 B.
- Lighthouse mobile local: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 999 ms, LCP 1,527 ms, TBT 12 ms, CLS 0.

Local evidence:

- `.factory/home.png`, `.factory/home-mobile.png`
- `.factory/demo-desktop.png`, `.factory/demo-mobile.png`
- `.factory/home-report.png`, `.factory/404.png`
- `.factory/verify-local.json`, `.factory/lighthouse-local.json`

## Clean clone and live deployment

Pending final commit, clean-clone run, deployment, and cold-live verification. This section will be replaced with exact commit, live hashes, route statuses, headers, and live screenshot paths before handoff.

## Known gaps

No product or test gaps are open. Paid purchase was deliberately removed because the reviewed Sociobot checkout returned 404; this avoids advertising an unavailable feature.
