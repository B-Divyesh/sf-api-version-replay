# Verification report — FAIL

**Work order:** `api-version-replay-verify-1`  
**Candidate:** `5c560636d39f6ec1c390946d578f77186678073a` (`main`)  
**Live URL:** https://api-version-replay.sociobot.in/  
**Verified:** 2026-08-27 UTC, clean checkout

## Verdict

**FAIL — deployment response-policy defect.** The live application is byte-for-byte the site produced by the candidate, and the CLI/site workflows pass. However, the hosting platform is serving the committed `/_headers` file as a public static object instead of applying it. Consequently the live site has no enforced Content-Security-Policy, no clickjacking protection, no Permissions-Policy, and incorrect cache lifetimes. This is a release blocker under the required browser response-policy and caching checks.

## Blocking defect

### P1 — deployed security and cache policy is not applied

Fresh `curl -I` checks on `/`, `/assets/index-DiDJiLeD.js`, `/assets/styles-DaLAeWYd.css`, `/version-specimen.webp`, and `/service-worker.js` all returned only:

```
Cache-Control: public, must-revalidate, max-age=30
Strict-Transport-Security: max-age=10886400; includeSubDomains; preload
Referrer-Policy: strict-origin-when-cross-origin
X-Content-Type-Options: nosniff
```

They did **not** return the candidate's declared `Content-Security-Policy`, `X-Frame-Options: DENY`, or `Permissions-Policy`. Hashed JS/CSS and the WebP are not immutable, and the service worker is not `no-cache`. `GET /_headers` returned HTTP 200 and the raw 632-byte configuration text, proving the host ignored it rather than applying it.

Required factory follow-up: configure these rules in the actual static-host/deployment configuration (or use its supported configuration filename), redeploy, then rerun the header probes. Do not treat the raw `_headers` file in the artifact as proof of the live policy.

## Candidate/build verification

- Clean checkout began at the requested SHA with no worktree changes; `npm ci` completed with 0 reported npm vulnerabilities.
- `npm test` passed: 3 Rust unit tests, 3 Rust black-box CLI integration tests, and 2 Vitest site tests.
- `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check`, and `git diff --check` passed.
- Exact production command `npm run build` passed and produced `dist/site/` plus `dist/bin/vr`; `cargo package --locked` passed and verified `target/package/version-replay-0.1.0`.
- Installed that packaged source into an isolated consumer root with `cargo install --path target/package/version-replay-0.1.0 --root <temp> --locked`. The installed `vr` showed its documented help, initialized a vault, imported `examples/old.json`, redacted it, and listed it via `--json`.
- The built and live home HTML SHA-256 both equal `34095c153e0dbb47c680d1eb986164b2156732308a7a3cd7b77169780f04c68c`. Live `index-DiDJiLeD.js` and `styles-DaLAeWYd.css` names and SHA-256 values also match the production build exactly.

## Independent CLI exercise

- Normal path: initialized a fresh vault; imported the old/new `payment-failed` envelopes; `list --json` returned both versions; `diff` found header/body/schema changes and exited `3`; `report` wrote a Markdown review artifact.
- Privacy/security: disk fixtures contained `[REDACTED]` for the `Authorization` header and `customer_email`; original example secrets were absent. Encrypted-vault fixture begins `VR01`; its plaintext marker was absent; a wrong passphrase failed safely.
- Boundary/recovery: an invalid fixture label failed with exit `1` and a clear validation message; non-loopback `https://example.com/hook` was rejected before a request with exit `1`.
- Local replay: a temporary `127.0.0.1` receiver received the saved POST with redacted body/header values; HTTP 204 yielded exit `0`, and HTTP 500 yielded the documented exit `4`.
- Capture: `capture --once` on `127.0.0.1:9355` accepted one POST (202), retained its method/path, and redacted the submitted authorization/email values before disk write.

## Browser, privacy, accessibility, and PWA exercise

- Desktop live run: one title, `lang`, one `<h1>`, and one `<main>`; normal browser-local compare produced schema/value changes. Malformed JSON produced an actionable error and Reset restored fixtures and focus. No console/page errors.
- Keyboard: visible 3 px focus outline observed on the reachable textarea; buttons operated with Enter. Theme toggle worked. With reduced motion emulated, transition duration was `1e-05s`.
- axe-core Playwright WCAG A/AA scan: **0 serious or critical violations**.
- At 390 x 844 mobile: `scrollWidth === clientWidth === 390`; comparison and dark theme worked with no console/page errors.
- PWA: active service worker controlled the page (`version-replay-shell-v1`); after the first online load, offline reload returned HTTP 200 and rendered the home H1. The worker uses `skipWaiting` and `clients.claim`; the update cache naming/code was inspected.
- Outbound requests during ordinary browsing were same-origin only. A deliberately invalid license-return test stored the token locally, stripped it from the URL, displayed the invalid-license recovery state, and made the single allowed request to `https://api.sociobot.in/api/v1/products/api-version-replay/verify`. No analytics, CDN fonts/scripts, or provider credentials were observed.

## Performance evidence

- Built initial JS: 7,468 bytes raw / 3.10 KB gzip; CSS: 15,034 bytes raw / 3.88 KB gzip; hero WebP: 96,986 bytes. All are within the stated budgets.
- Lighthouse 12.8.2 mobile report generated scores of Performance 100, Accessibility 100, Best Practices 100, SEO 100; LCP 1,374 ms, TBT 18.5 ms, CLS 0. The Lighthouse process returned non-zero only because the isolated Chrome tab crashed during post-audit screenshot/BFCache collection; the written JSON report contains the completed scores. This does not change the independently observed P1 deployment-policy failure.

## Retest command set

```sh
npm ci
npm test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
npm run build
cargo package --locked
curl -I https://api-version-replay.sociobot.in/
curl -I https://api-version-replay.sociobot.in/assets/index-DiDJiLeD.js
curl -I https://api-version-replay.sociobot.in/service-worker.js
```
