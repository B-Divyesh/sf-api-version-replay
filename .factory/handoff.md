# Version Replay — polish round 4 handoff

## Outcome

PASS. Every finding in `.factory/review-1.md` through `.factory/review-4.md` is closed. The repair commit is `23b026cdec23b5f2c948a190a5db40785d6215de`; deployment `9168ee63-d78c-4cd1-8943-987fe1153494` is live at <https://api-version-replay.sociobot.in/>.

## What changed

- Full-document and back-forward-cache restores now focus the current H1 without moving the restored scroll position. The route announcement is refreshed.
- Mobile terminal and comparison-table overflow regions are named, keyboard-focusable, keyboard-scrollable, and use the concrete/moss focus ring.
- The designed 404 now has a canonical and complete Twitter metadata. The route claim covers its metadata, `noindex`, and host rewrite.
- The untestable future Terms promise was removed.
- The completed CLI recording now appears immediately after the first-screen facts, before the workflow explanation.
- The service-worker cache advanced to `version-replay-shell-v6`.
- The catalog description is a 76-character verb-first sentence.
- `.factory/copy-audit.md`, `.factory/claims.json`, and `.factory/polish-4.md` record the current copy, claims, and every cumulative finding.

The first screen remains: `Test old webhook versions against localhost`, the engineer audience, one `Try it with sample data` action, its result, and three plain facts. The `?demo=1` path remains memory-only and shows the completed CLI import/redaction/diff/replay/report workflow before its secondary JSON comparison.

## Clean-clone verification

Clean clone: `/tmp/api-version-replay-polish4.eax62t`
Verified commit: `23b026cdec23b5f2c948a190a5db40785d6215de`

All 20 commands declared in `.factory/claims.json` passed separately:

`cli-demo-workflow`, `primary-demo-workflow`, `demo-output-paths`, `vault-directory`, `redaction-before-storage`, `loopback-only`, `encrypted-storage`, `capture-loopback`, `report-formats`, `contract-dimensions`, `exact-replay`, `fixture-formats`, `exit-codes`, `no-provider-credentials`, `no-telemetry`, `mit-license`, `browser-demo-isolation`, `browser-storage-scope`, `offline-demo`, and `route-metadata`.

The following also passed:

```sh
npm ci
npm test
npm run typecheck
npm run build
npm run test:browser -- http://127.0.0.1:4174/
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
```

`npm run build` produced `dist/bin/vr` and `dist/site/`. The crate package contains 13 files and is 27.2 KiB compressed. Built site assets are 7,325 bytes for the main JavaScript, 994 and 755 bytes for the two small loaders, and 18,345 bytes CSS.

## Live verification

- `npm run test:browser -- https://api-version-replay.sociobot.in/` passed. It covers desktop keyboard use, 390 px Home and Demo in both themes, keyboard scrolling, four full-document Back cases, hash history, offline reload, external fragments, and axe with zero serious/critical findings.
- Fresh Demo instrumentation observed zero storage reads, zero storage writes, no cookies, no cross-origin requests, and no fixture upload. Reset, Start for real, and offline reload passed.
- Cold route checks verified unique titles, H1s, canonicals, OG/Twitter data, and one main landmark for Home, Demo, Privacy, Terms, and 404.
- `/polish-4-missing` returned HTTP 404 with the designed page. Eleven discovered links resolved.
- Home, Privacy, Terms, 404, and `service-worker.js` match the built artifacts by SHA-256.
- Live responses include CSP, HSTS, frame denial, `nosniff`, referrer, and permissions headers.
- Lighthouse: 100 Performance, 100 Accessibility, 100 Best Practices, 100 SEO; FCP 0.9 s, LCP 1.4 s, TBT 0 ms, CLS 0.

Evidence is under `.factory/polish-4-live/`. Key files include:

- `home/screenshot-mobile.png` and `home/screenshot-desktop.png`
- `demo/screenshot-mobile.png` and `demo/screenshot-desktop.png`
- `back-focus-mobile.png`
- `terminal-focus-mobile.png` and `result-focus-mobile.png`
- `not-found/screenshot-mobile.png`
- `privacy/` and `terms/` route reports/screenshots
- `lighthouse.json`

## Run, package, and deploy

```sh
npm ci
npm test
npm run typecheck
npm run build
npm run test:browser -- http://127.0.0.1:4173/
cargo package --locked
```

The work-order deployment used:

```sh
npm ci && npm run build:site
/opt/fleet/lib/deploy-static.sh api-version-replay dist/site
```

The factory owns registry publishing, infrastructure, DNS, and any future merchant registration. No purchase action is advertised while checkout is unavailable. There are no known product or verification gaps in this work order.
