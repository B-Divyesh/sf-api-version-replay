# Verification report — PASS

**Work order:** `api-version-replay-verify-2`  
**Candidate:** `59428a8fc5b72a09a4f1acc193505f2ef86240de`  
**Live URL:** <https://api-version-replay.sociobot.in/>  
**Verified:** 2026-08-27 UTC from a detached, clean checkout

## Verdict

**PASS.** The candidate fulfils the researched job: it stores redacted, version-labelled inbound HTTP fixtures locally, compares version changes, replays only to loopback, and emits review artifacts. The previous deployment-only response-policy and caching failure is fixed in the live deployment.

No release-blocking product defects were found. The deployment matches the candidate's production site artifact exactly for the home HTML, JS, and CSS.

## Build, tests, and package evidence

- Detached clean checkout resolved to `59428a8fc5b72a09a4f1acc193505f2ef86240de`; `npm ci` completed with **0 npm vulnerabilities**.
- `npm test` passed: **3 Rust unit tests**, **3 Rust CLI integration tests**, and **4 Vitest site tests**.
- `npm run typecheck`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`, and `git diff --check` all passed.
- Exact production build `npm run build` passed and produced `dist/bin/vr` plus `dist/site/`; its static policy gate passed.
- `cargo package --locked` passed: 40 files, 2.8 MiB unpacked / 2.7 MiB compressed, then package verification compiled successfully.
- An isolated consumer installed `target/package/version-replay-0.1.0` with `cargo install --path … --root <temp> --locked`. Its installed `vr` showed the documented help, initialized a vault, imported `examples/old.json`, and returned a redacted fixture through `--json list`.

## CLI end-to-end evidence

- Normal workflow: initialized a new vault; imported the old and new `payment-failed` fixtures; `list --json` showed both versions; `diff` reported schema/value changes and exited **3**; `report` wrote Markdown.
- Privacy: on-disk fixture and replay receiver both contained `[REDACTED]` for `Authorization` and email. An AES-256-GCM vault file did not contain the plaintext marker; a wrong passphrase failed safely with exit **1**.
- Boundaries/recovery: `invalid/name` was rejected with a clear validation message (exit **1**); `https://example.com/webhook` was refused before network use (exit **1`). A controlled loopback `307` response was returned as HTTP 307 (exit **4**) rather than followed, confirming redirect handling does not escape the loopback boundary.
- Local HTTP behavior: a `127.0.0.1` receiver received the saved POST, redacted header/body, and path; HTTP 204 returned exit **0** and HTTP 500 returned the documented exit **4**.
- Capture: `capture --once` on `127.0.0.1` accepted one POST with HTTP 202 and persisted its request with authorization/email redacted.

## Live, browser, privacy, and accessibility evidence

- Fresh SHA-256 comparisons were byte-identical for live and built `index.html` (`34095c…04c68c`), `assets/index-DiDJiLeD.js` (`904604…42f88df`), and `assets/styles-DaLAeWYd.css` (`fb80ca…7037e9`).
- `npm run test:browser -- https://api-version-replay.sociobot.in/` passed: desktop keyboard fixture traversal and Enter comparison, theme state, reduced-motion transition, active service worker and offline reload, 390 × 844 no horizontal overflow, and no console/page errors.
- axe-core WCAG A/AA scan found **0 serious/critical violations**. Visual review covered 1440 px desktop and 390 px mobile. The live page has `lang`, one `<h1>`, one `<main>`, a skip link, labelled controls, alt text, and visible focus styling.
- Invalid browser JSON showed an actionable parsing error; Reset restored the original specimens and focus to the first textarea.
- Ordinary browsing made same-origin requests only. A controlled invalid license-return run removed `?license=` from the address bar, stored the token locally, showed the invalid-license recovery text, and made exactly the allowed request to `https://api.sociobot.in/api/v1/products/api-version-replay/verify`.
- Service worker version `version-replay-shell-v2` uses `skipWaiting`, `clients.claim`, cache-version replacement, precaches shell assets, and the browser check confirmed offline navigation reload after an online visit.

## Live response policy, caching, and budget evidence

- `/`, `/privacy/`, and `/terms/` returned CSP, `X-Frame-Options: DENY`, `Permissions-Policy: camera=(), microphone=(), geolocation=()`, HSTS, `nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`.
- `/assets/index-DiDJiLeD.js`, `/assets/styles-DaLAeWYd.css`, and `/version-specimen.webp` returned `Cache-Control: public, max-age=31536000, immutable`; `/service-worker.js` returned `Cache-Control: no-cache`.
- The old `/_headers` source is not exposed: that path receives the normal navigation fallback HTML, while Azure's deployed `staticwebapp.config.json` policy is active.
- Build output: JS **7,468 B raw / 3.10 KB gzip**; CSS **15,034 B raw / 3.88 KB gzip**; hero WebP **96,986 B**. All are below the supplied budgets.
- Fresh Lighthouse mobile report scored **100 / 100 / 100 / 100** (Performance / Accessibility / Best Practices / SEO): FCP 1,144 ms, LCP 1,355 ms, TBT 5.5 ms, CLS 0. Lighthouse wrote the complete JSON report, then returned non-zero because its isolated Chrome tab crashed during its final screenshot/BFCache collection; this is a test-harness tail failure, not a failed audit or observed product defect.

## Defects by severity

- **P0/P1/P2/P3: none found.**
- Coverage limitation (not a product defect): no factory-issued valid paid-license token was available, so successful paid checkout/valid-verdict activation was not exercised. Invalid and offline-safe license recovery, storage, URL stripping, and the allowed verification endpoint were exercised.

## Re-run

```sh
npm ci
npm test
npm run typecheck
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
npm run build
cargo package --locked
npm run test:browser -- https://api-version-replay.sociobot.in/
```
