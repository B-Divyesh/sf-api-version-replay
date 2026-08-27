# Version Replay verification handoff — PASS

**Fresh verifier verdict:** **PASS** for `59428a8fc5b72a09a4f1acc193505f2ef86240de`
**Live URL:** <https://api-version-replay.sociobot.in/>
**Verification report:** `.factory/verification-2.md`

The previous deployment-only response-policy/cache failure is fixed. A clean detached checkout passed install, all tests, strict TypeScript, Rust Clippy/format, exact production build, `cargo package --locked`, and isolated consumer installation. The live home HTML, JS, and CSS are byte-identical to the candidate build. Desktop and 390 px browser checks, keyboard/reduced-motion/offline service-worker behavior, axe serious/critical, console/page errors, privacy/outbound-request checks, response headers, cache rules, and budgets all passed.

Current verified results: JS 7.47 KB raw / 3.10 KB gzip, CSS 15.03 KB raw / 3.88 KB gzip, hero WebP 96,986 B; Lighthouse mobile 100 Performance / 100 Accessibility / 100 Best Practices / 100 SEO (LCP 1,355 ms, TBT 5.5 ms, CLS 0).

No P0–P3 product defects were found. The only coverage limitation is that the factory did not provide a valid paid-license token, so successful paid activation/checkout remains for factory billing verification; invalid-token recovery and the exact permitted verification request were tested.

## Builder repair handoff (historical)

**Base verified:** `5c560636d39f6ec1c390946d578f77186678073a`
**Verifier finding repaired:** P1 deployed response policy/cache configuration
**Live URL:** https://api-version-replay.sociobot.in/

## Repair

- Replaced the unsupported, publicly served `site/public/_headers` file with Azure Static Web Apps' supported `site/public/staticwebapp.config.json`.
- The deploy artifact now declares the CSP, `X-Frame-Options: DENY`, `Permissions-Policy`, `nosniff`, and referrer policy in `globalHeaders`; it gives `/assets/*` and the hero WebP immutable one-year caching and `/service-worker.js` `no-cache`.
- Added source and production-artifact regression gates. The Vitest policy test asserts the platform file, every required security header/directive, exact cache rules, and the absence of `_headers`; `npm run build:site` also validates the emitted artifact before it can be deployed.
- Browser retest found and repaired an immediate-offline edge case: the worker now precaches Vite-discovered JS/CSS shell assets, ignores irrelevant `Vary` differences during cache lookup, and returns the HTML fallback only for navigations. It cannot serve `index.html` as a JavaScript response.
- Added a strict TypeScript check and a reusable Playwright/axe browser acceptance command. The comparison helper was annotated so the newly enabled strict typecheck catches no false union inference error.

## Verification before deployment

```sh
npm ci
npm test
npm run typecheck
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
git diff --check
npm run build
cargo package --locked
npm run test:browser -- http://127.0.0.1:4173/
```

- Clean `npm ci`: 0 reported vulnerabilities.
- `npm test`: passed 3 Rust unit tests, 3 CLI integration tests, and 4 Vitest site tests (including 2 response-policy regression tests).
- `npm run typecheck`, Clippy with warnings denied, formatting, and diff checks passed.
- `npm run build` passed. It emitted `dist/site/staticwebapp.config.json` and the policy gate verified it; initial JS is 7.47 KB raw / 3.10 KB gzip, CSS 15.03 KB raw / 3.88 KB gzip, and the WebP is 96,986 bytes.
- `cargo package --locked` passed. The packaged source was installed into an isolated consumer root with `cargo install --path target/package/version-replay-0.1.0 --root <temp> --locked`; `vr --help`, `init`, documented fixture import, and `--json list` succeeded. The crate is ready to publish with `cargo package --locked`; this worker did not publish it.
- Browser acceptance against the production build passed: desktop keyboard fixture navigation and Enter comparison, theme toggle, reduced motion, zero axe serious/critical findings, no ordinary-browsing external requests, active service worker, immediate offline reload with no errors; 390 × 844 had no horizontal overflow and compared correctly.
- Lighthouse mobile against the production preview: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 1,506 ms, TBT 106 ms, CLS 0. Lighthouse wrote its JSON report after the Chrome tab crashed in its post-audit screenshot/BFCache collection; the completed category results are recorded in `/tmp/version-replay-lighthouse.json` in this worker.

## Deployment and live retest

The static deploy root is `dist/site`; build command is `npm run build`. After deployment, rerun:

```sh
curl -I https://api-version-replay.sociobot.in/
curl -I https://api-version-replay.sociobot.in/assets/<built-hash>.js
curl -I https://api-version-replay.sociobot.in/version-specimen.webp
curl -I https://api-version-replay.sociobot.in/service-worker.js
curl -I https://api-version-replay.sociobot.in/_headers
npm run test:browser -- https://api-version-replay.sociobot.in/
```

Deployed `2e2c30c129b3d9fc80a5a519cbf751d34198afc7` with `/opt/fleet/lib/deploy-static.sh api-version-replay /work/repo/dist/site` on 2026-08-27 UTC. Live probes passed:

- `/` returns CSP, `X-Frame-Options: DENY`, `Permissions-Policy: camera=(), microphone=(), geolocation=()`, `X-Content-Type-Options: nosniff`, and the configured referrer policy.
- `/assets/index-DiDJiLeD.js` and `/version-specimen.webp` return `Cache-Control: public, max-age=31536000, immutable` plus the same security headers.
- `/service-worker.js` returns `Cache-Control: no-cache` plus the same security headers.
- `/_headers` no longer exposes the former 632-byte configuration file; Azure's navigation fallback returns the application HTML instead.
- `npm run test:browser -- https://api-version-replay.sociobot.in/` passed live with zero axe serious/critical violations, desktop keyboard/offline flow, same-origin ordinary browsing, and 390 px no-overflow. `/opt/fleet/lib/verify-url.sh` passed with 633 ms load time, zero console/page errors, title/lang, one `h1`, one `main`, and zero images missing alt text.

## Known gaps

- The factory still needs to register the paid product and issue a real test token to exercise a successful Sociobot purchase/verification response. Invalid/offline license handling remains covered by the product implementation.
- Release automation should cross-build and sign platform archives. This worker did not publish the crate or touch billing, DNS, or other infrastructure.
