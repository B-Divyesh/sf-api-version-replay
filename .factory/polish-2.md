# Perfection-loop polish 2

Candidate: `51f6e33b676bbd5c77de6e7a80e3597dde9f22ac`  
Repair commit: `5f75784`  
Deployment: `f1d1a05a-0252-4073-b8f8-57af22bd3a85`  
Live target: <https://api-version-replay.sociobot.in/>

This round reread `.factory/review-1.md`, `.factory/polish-1.md`, and `.factory/review-2.md`. The detailed F-1 implementation history remains in `polish-1.md`; every cumulative ID is re-mapped below to its current implementation and round-2 evidence. Ranges mean every ID in the inclusive range maps to the stated change and evidence.

Evidence shorthand: `C:<id>` is `npm run test:claims -- <id>` from the clean clone; `B` is the repeated browser smoke suite; `L` is live Lighthouse; `V` is `/opt/fleet/lib/verify-url.sh`; and `S` is the live route/header sweep.

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | The first screen names the localhost webhook-version job, its engineer audience, and one sample action. | C:route-metadata; `.factory/home-mobile.png`; live `/` |
| F-1-2 | The primary `?demo=1` screen now opens on the completed `vr demo` transcript: two redacted imports, five changes, both loopback 204 results, and the report path. The JSON comparator is secondary. | C:primary-demo-workflow; C:cli-demo-workflow; `.factory/live-round2/demo-cold-mobile.png`; live `/?demo=1` |
| F-1-3 | Demo mode bypasses real browser storage and billing state and keeps edits in memory. | C:browser-demo-isolation; live cold instrumentation: zero reads/writes/cookies/external requests |
| F-1-4 | The claims inventory now has 20 entries, each with exactly one tagged observable test. | `npm test`; `claims inventory` test; clean clone |
| F-1-5 | Report UI and copy show only structured contract changes that `vr report` emits. | C:report-formats; `.factory/home-report.png` |
| F-1-6 | Unavailable checkout, price, restore, license, and paid-feature copy remain removed. | live link crawl; `rg 'checkout\|Buy the Pro'` |
| F-1-7 | The concrete/moss 404 and Azure response override remain active. | S: unknown URL 404; `.factory/404.png` |
| F-1-8–F-1-12 | Meta and first-screen claims use concrete, tested workflow, credential, telemetry, and replay wording. | C:cli-demo-workflow; C:no-provider-credentials; C:no-telemetry; C:primary-demo-workflow |
| F-1-13–F-1-23 | Public-host refusal, pre-write redaction, formats, import/capture, comparison dimensions, exit 3, and exact loopback replay are implemented and tested. | C:loopback-only; C:redaction-before-storage; C:report-formats; C:capture-loopback; C:contract-dimensions; C:exit-codes; C:exact-replay |
| F-1-24–F-1-27 | The isolated browser sample works offline and sends no fixture contents or cross-origin requests. | C:offline-demo; C:browser-demo-isolation; live cold instrumentation |
| F-1-28 | The report preview promises and shows structured changes rather than an unsupported replay result. | C:report-formats; `.factory/home-report.png` |
| F-1-29–F-1-31 | Unsupported free/paid tier and one-time-price claims remain removed. | live page text/link crawl |
| F-1-32–F-1-35 | README audience, workflow, credential, and telemetry statements are short and mapped to claims. | C:cli-demo-workflow; C:no-provider-credentials; C:no-telemetry; `.factory/copy-audit.md` |
| F-1-36 | The unsupported Rust-version assertion remains removed. | README audit |
| F-1-37–F-1-41 | Default redaction, fixture formats, exact replay, loopback validation, and capture storage have direct tests. | C:redaction-before-storage; C:fixture-formats; C:exact-replay; C:loopback-only; C:capture-loopback |
| F-1-42–F-1-43 | JSON demo output and exit codes 1, 3, and 4 are tested through the built CLI. | C:cli-demo-workflow; C:exit-codes |
| F-1-44–F-1-49 | Encryption, passphrase exclusion/rejection, wildcard/header redaction, and pre-write secrecy are tested. | C:encrypted-storage; C:redaction-before-storage |
| F-1-50–F-1-53 | Unsupported tier, checkout, restore, and license-cache behavior remains absent. | source and live text/link crawl |
| F-1-54–F-1-56 | Test/build/browser prose was narrowed to observable behavior; the actual gates run from a clean clone. | clean-clone `npm test`, `npm run build`, and B |
| F-1-57–F-1-58 | Network and storage copy is limited to explicit loopback replay, tested redaction, and tested encryption. | C:no-telemetry; C:loopback-only; C:redaction-before-storage; C:encrypted-storage |
| F-1-59 | The standard MIT license is shipped and tested. | C:mit-license |
| F-1-60–F-1-66 | Privacy pages match actual CLI/browser storage and network behavior; demo remains isolated. | C:no-telemetry; C:browser-storage-scope; C:browser-demo-isolation; live cold instrumentation |
| F-1-67 | Hash navigation uses history, restores scroll, focuses headings, and announces changes. | B repeats the forward/back/focus cycle three times |
| F-1-68 | Every public route retains distinct title, description, canonical, OG/Twitter image, icons, and theme color. | C:route-metadata; V |
| F-1-69 | Home, demo, Privacy, Terms, and 404 retain the common header/footer shell. | B; V; live route crawl |
| F-1-70 | External links name GitHub or state that they open another site. | live link crawl |
| F-1-71 | Sitemap lists all public routes; the designed 404 is excluded and returns HTTP 404. | S: `/sitemap.xml` 200 and unknown route 404 |
| F-1-72–F-1-87 | Decorative lore, ambiguous section names, bench jargon, pricing slogans, and misleading replay/report wording remain replaced by literal task language. | `.factory/copy-audit.md`; live home/demo screenshots |
| F-1-88–F-1-98 | README and control wording stays under 22 words, uses one term per concept, and names each action precisely. | `.factory/copy-audit.md`; B keyboard controls |
| F-2-1 | Added `vault-directory`; it initializes/imports only beneath a caller-selected nested path and proves no default vault appears. | C:vault-directory |
| F-2-2 | Added `demo-output-paths`; it runs plain `vr demo`, parses both printed paths, and verifies the directory and report exist. | C:demo-output-paths |
| F-2-3 | Replaced the build-artifact promise with the instruction `Install the CLI from this checkout:`. Packaging is independently verified. | README; clean-clone `cargo package --locked` |
| F-2-4 | The browser gate now waits for both settled scroll and focused `#hero-title`, then repeats the route cycle three times. | B, local and live |

## Final evidence

- Clean clone: `/tmp/api-version-replay-round2.BWnHI7` at `5f75784`; `npm ci`, `npm test`, `npm run typecheck`, `npm run build`, `cargo fmt --check`, strict Clippy, `cargo package --locked`, and browser smoke all passed.
- Claims: all 20 commands in `.factory/claims.json` passed; the inventory test found one tagged test per claim.
- Browser/accessibility/privacy/offline: Playwright passed desktop, 390 px, keyboard, three Back/focus cycles, all public routes, zero serious/critical axe findings, no overflow, isolated storage/network checks, and offline reload.
- Performance: `.factory/live-round2/lighthouse.json` records 100 performance, 100 accessibility, 100 best practices, and 100 SEO; LCP 1.4 s, CLS 0, TBT 0 ms.
- Live: `/`, `/?demo=1`, `/privacy/`, `/terms/`, `/robots.txt`, and `/sitemap.xml` return 200; an unknown route returns 404. CSP, frame denial, nosniff, referrer, and permissions headers are present.
- Screenshots: `.factory/demo-desktop-round2.png`, `.factory/demo-mobile-round2.png`, `.factory/live-round2/demo-cold-mobile.png`, and `.factory/live-round2/{home,demo}/screenshot-*.png`.

No finding remains open.
