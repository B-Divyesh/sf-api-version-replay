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

## Clean clone evidence

A fresh clone of `origin/main` at `88dc1e6ca8e8a21fe7cb7b3750e55e203a2d7495` ran the complete command set above. All 17 individual claim tests, Rust/site tests, clippy, formatting, typecheck, build, package verification, and browser sweep passed. `npm ci` reported 0 vulnerabilities.

## Deployment and cold-live evidence

Deployed through `/opt/fleet/lib/deploy-static.sh api-version-replay /work/repo/dist/site`.

- Azure deployment ID: `20e4d743-2096-4fcd-982b-0d075733f3e4`.
- Live URL: <https://api-version-replay.sociobot.in/>.
- Home, `?demo=1`, Privacy, Terms, robots, sitemap, favicon, Apple icon, social card, and GitHub links returned HTTP 200.
- `/this-route-does-not-exist` returned HTTP 404 with title `Page not found — Version Replay` and H1 `This page was not found`.
- Cold demo at desktop and 390 px had one H1, completed changes, the banner/reset/exit actions, zero overflow, zero console errors, zero external requests, and zero storage reads/writes.
- Live browser suite passed twice-used route focus/history, offline reload, keyboard, mobile, reduced-motion, and route-wide axe checks.
- Home response includes CSP, `X-Frame-Options: DENY`, Permissions-Policy, `nosniff`, HSTS, and referrer policy.
- Hashed JS returns `Cache-Control: public, max-age=31536000, immutable`; the service worker returns `Cache-Control: no-cache`.
- Live and built SHA-256 hashes match for home HTML, JS, CSS, 404, Privacy, and Terms.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 944 ms, LCP 1,384 ms, TBT 29 ms, CLS 0.

Live evidence:

- `.factory/live/verify.json`
- `.factory/live/lighthouse.json`
- `.factory/live/home-desktop.png`, `.factory/live/home-mobile.png`
- `.factory/live/demo-desktop.png`, `.factory/live/demo-mobile.png`
- `.factory/live/404.png`

## Known gaps

No product, test, deployment, or review gaps are open. Paid purchase was deliberately removed because the reviewed Sociobot checkout returned 404; this avoids advertising an unavailable feature.
