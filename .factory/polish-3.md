# Perfection-loop polish 3

Released candidate: `03783af146be2a87c7edccd6883b70344727060b`  
Review base: `e75928a0915bf4605bc32a4c80706fe88fa707d1`  
Repair commits: `54d8899`, `d334183`  
Deployment: `bbb9d98b-fb64-4764-aece-513a7d528b75`  
Live target: <https://api-version-replay.sociobot.in/>

The final candidate was verified from clean clone `/tmp/api-version-replay-polish3-final.IsW4dD` at `d3341837b27250fd1a19185b9c80aefc4497eab2`. Every command in `.factory/claims.json` passed separately. The aggregate test, build, browser, accessibility, privacy, offline, formatting, lint, package, route, header, and live checks also passed.

Evidence shorthand: `C:<id>` means the separately run `npm run test:claims -- <id>`; `T` means clean-clone `npm test`; `B` means `npm run test:browser -- https://api-version-replay.sociobot.in/`; `V` means the worker URL verifier; `S` means the live status/header/hash sweep; and `L` means `.factory/polish-3-live/lighthouse.json`.

## Current review findings

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Changed the GitHub CLI-guide target from missing `#usage` to `#use-your-fixtures`. Added a source test that maps repository fragments to README headings and a browser check that opens the external page and finds its rendered anchor. | `Azure Static Web Apps response policy > points GitHub fragment links at real README headings`; B external-fragment stage; live Home link; `.factory/polish-3-live/home/screenshot-desktop.png` |
| F-3-2 / F-1-60 | Removed the separate, unlisted `no account system` promise. Privacy now retains only the exact telemetry statement listed in `claims.json`. | C:no-telemetry; live `/privacy/` text check; `.factory/polish-3-live/privacy/screenshot-mobile.png` |
| F-3-3 | Removed the unsupported `short-lived` retention assurance. Privacy now says the hosting provider may process logs under its own retention policy. | live `/privacy/` positive and negative text checks; `.factory/polish-3-live/privacy/screenshot-desktop.png` |

## Cumulative finding map

| Finding ID | Current change/state | Evidence |
| --- | --- | --- |
| F-1-1 | First screen states the localhost webhook-version job, engineer audience, one sample action, its result, and three facts. | C:route-metadata; B; `.factory/polish-3-live/home/screenshot-mobile.png` |
| F-1-2 | `?demo=1` opens on the completed CLI workflow; `vr demo` uses a new temporary vault and bundled fixtures. | C:primary-demo-workflow; C:cli-demo-workflow; `.factory/polish-3-live/demo/screenshot-mobile.png` |
| F-1-3 | Demo bypasses real browser storage and external billing/network paths. | C:browser-demo-isolation; live cold instrumented demo |
| F-1-4 | `claims.json` has 20 entries and exactly one tagged test per entry. | `claims inventory` test; all 20 C commands; T |
| F-1-5 | Report copy and preview show only structured changes the CLI emits. | C:report-formats; live Home report preview |
| F-1-6 | Dead checkout and all unavailable paid purchase/license copy remain removed. | S live link/text crawl |
| F-1-7 | Unknown paths return the designed concrete/moss 404 with a recovery action. | S returned 404; `.factory/polish-3-live/404.png` |
| F-1-8 | Metadata workflow claim is concrete and covered by the CLI demo. | C:cli-demo-workflow; C:route-metadata |
| F-1-9 | Credential-free behavior is retained and tested with a cleared environment. | C:no-provider-credentials |
| F-1-10 | Vague “keep” copy was replaced by the observable save/import workflow. | C:cli-demo-workflow; live Home copy |
| F-1-11 | The compound hero promise was split into tested workflow steps. | C:cli-demo-workflow; C:exact-replay |
| F-1-12 | The unqualified account/tunnel/telemetry slogan is absent; retained facts have direct claims. | C:no-telemetry; C:no-provider-credentials; live Privacy check |
| F-1-13 | Public-destination refusal is stated precisely and covers hostname, IP, and lookalike cases. | C:loopback-only |
| F-1-14 | Copy says configured fields are removed before storage and every default is tested. | C:redaction-before-storage |
| F-1-15 | Unsupported Pro/JUnit wording is absent; Markdown and JSON remain. | C:report-formats; live text crawl |
| F-1-16 | The universal “every result” claim remains deleted. | `.factory/copy-audit.md`; live text crawl |
| F-1-17 | The provider-sandbox availability claim remains deleted. | `.factory/copy-audit.md`; live text crawl |
| F-1-18 | Import and capture use the single term “webhook fixture” and both paths are tested. | C:fixture-formats; C:capture-loopback |
| F-1-19 | Pre-write body/header redaction is stated precisely. | C:redaction-before-storage |
| F-1-20 | Method, path, header, JSON type, and primitive-value comparison are directly exercised. | C:contract-dimensions |
| F-1-21 | Changed contracts exit 3 in the built CLI. | C:exit-codes |
| F-1-22 | The loopback receiver verifies saved method, headers, and body. | C:exact-replay |
| F-1-23 | The UI states public-host rejection without an overbroad timing claim. | C:loopback-only |
| F-1-24 | The demo reloads with its completed result after the browser goes offline. | C:offline-demo; live cold offline reload |
| F-1-25 | Browser privacy wording matches instrumented storage and request behavior. | C:browser-demo-isolation |
| F-1-26 | “Zero uploads” was replaced by the precise sample-content statement. | C:browser-demo-isolation |
| F-1-27 | Broad “entirely in your browser” wording was narrowed to observable comparison behavior. | C:browser-demo-isolation |
| F-1-28 | The report promises structured contract changes only. | C:report-formats |
| F-1-29 | Unsupported free-tier marketing remains absent. | live text crawl |
| F-1-30 | Unsupported purchase and pay-once wording remains absent. | live text/link crawl |
| F-1-31 | Price cards and unconfigured merchant actions remain absent. | live Home screenshot and link crawl |
| F-1-32 | README opens with a short audience/job sentence. | C:cli-demo-workflow; `.factory/copy-audit.md` |
| F-1-33 | README feature sentences are short and map to workflow/report tests. | C:cli-demo-workflow; C:report-formats |
| F-1-34 | The demo runs without provider credentials. | C:no-provider-credentials |
| F-1-35 | Telemetry wording maps to the source/dependency network-path inventory. | C:no-telemetry |
| F-1-36 | The incorrect Rust 1.82 assertion remains removed. | README audit; clean-clone build |
| F-1-37 | Default payment, identity, cookie, and authorization redaction is table-tested. | C:redaction-before-storage |
| F-1-38 | Request-envelope and plain-body fixture formats are tested and named consistently. | C:fixture-formats |
| F-1-39 | The loopback receiver compares the complete stored request. | C:exact-replay |
| F-1-40 | Replay boundary tests cover allowed and refused destinations. | C:loopback-only |
| F-1-41 | One-shot capture stores a redacted request and exits. | C:capture-loopback |
| F-1-42 | JSON documentation is limited to the parsed `vr --json demo` result. | C:cli-demo-workflow |
| F-1-43 | Exit codes 1, 3, and 4 are tested through real CLI processes. | C:exit-codes |
| F-1-44 | The encrypted format and successful decryption are inspected. | C:encrypted-storage |
| F-1-45 | The passphrase is absent from vault configuration and ciphertext. | C:encrypted-storage |
| F-1-46 | A missing or wrong passphrase cannot read fixture contents. | C:encrypted-storage |
| F-1-47 | One-segment wildcard body redaction is exercised. | C:redaction-before-storage |
| F-1-48 | Case-insensitive custom header redaction is exercised. | C:redaction-before-storage |
| F-1-49 | Secret absence is checked in plain and encrypted writes. | C:redaction-before-storage; C:encrypted-storage |
| F-1-50 | Unsupported free-tier feature marketing remains absent. | live/README text audit |
| F-1-51 | Unsupported Pro commands and feature marketing remain absent. | CLI help and live/README text audit |
| F-1-52 | Dead checkout and restore directions remain absent. | live/README link audit |
| F-1-53 | Unsupported license-cache behavior remains absent. | source and copy audit |
| F-1-54 | Test-suite marketing is absent; the documented commands run successfully. | T |
| F-1-55 | Build-output marketing is absent; both required artifacts are verified directly. | clean-clone `npm run build` |
| F-1-56 | Browser behavior is proved by the browser suite, not a prose promise. | B; zero serious/critical axe findings |
| F-1-57 | Network copy is limited to the explicit loopback replay path. | C:no-telemetry; C:loopback-only |
| F-1-58 | Replay, redaction, and encryption are separate precise claims. | C:loopback-only; C:redaction-before-storage; C:encrypted-storage |
| F-1-59 | The standard MIT license ships and is tested. | C:mit-license |
| F-1-60 | The remaining exact telemetry statement is inventoried; the unlisted account clause was removed in round 3. | C:no-telemetry; live Privacy check |
| F-1-61 | Vault storage and pre-write redaction match temporary-vault evidence. | C:cli-demo-workflow; C:redaction-before-storage |
| F-1-62 | Encryption and passphrase statements map to cryptographic/storage tests. | C:encrypted-storage |
| F-1-63 | Network policy describes explicit loopback replay only. | C:no-telemetry; C:loopback-only |
| F-1-64 | Normal website storage is only the theme; demo touches no storage. | C:browser-storage-scope; C:browser-demo-isolation |
| F-1-65 | Demo request origins and bodies are instrumented. | C:browser-demo-isolation |
| F-1-66 | Cookie, storage, and request inventories cover normal browsing. | C:browser-storage-scope |
| F-1-67 | Hash navigation creates history, restores scroll, focuses headings, and announces changes. | B repeats the cycle three times |
| F-1-68 | Routes retain distinct titles, descriptions, canonicals, social art, and icons. | C:route-metadata; V |
| F-1-69 | Home, Demo, Privacy, Terms, and 404 share the required header/footer shell. | B; V; live screenshots |
| F-1-70 | External links name GitHub; the guide fragment is now validated end to end. | B external-fragment stage; `points GitHub fragment links at real README headings` |
| F-1-71 | Sitemap contains every indexable route; the noindex 404 returns 404. | S |
| F-1-72 | Decorative CLI lore remains replaced by a literal version label. | `.factory/copy-audit.md`; live Home screenshot |
| F-1-73 | Image labels name a saved webhook fixture and example data. | live Home screenshot |
| F-1-74 | The section heading literally names how Version Replay works. | `.factory/copy-audit.md` |
| F-1-75 | Step one says “Save and redact a fixture.” | live Home screenshot |
| F-1-76 | Step two says “Compare version changes.” | live Home screenshot |
| F-1-77 | Step three says “Replay it to localhost.” | live Home screenshot |
| F-1-78 | The browser tool is labelled as a JSON-only sample. | `.factory/polish-3-live/demo/screenshot-mobile.png` |
| F-1-79 | The browser heading says “Compare two sample webhook fixtures.” | C:route-metadata; live Demo screenshot |
| F-1-80 | Empty state says “No comparison yet.” | B |
| F-1-81 | UI consistently uses fixture, type, and value. | `.factory/copy-audit.md` |
| F-1-82 | The report section is named “Export contract changes.” | live Home screenshot |
| F-1-83 | Pricing and its slogan remain absent. | live Home text audit |
| F-1-84 | Generic optional-upgrade copy remains absent. | live Home text audit |
| F-1-85 | The mood slogan remains absent. | live Home text audit |
| F-1-86 | Final action is “Install Version Replay.” | live Home screenshot |
| F-1-87 | Footer says what is compared and that it runs on the user's machine. | V; live route screenshots |
| F-1-88 | README audience sentence remains below 22 words. | `.factory/copy-audit.md` |
| F-1-89 | README feature copy remains split and mapped to claims. | `.factory/copy-audit.md`; C:cli-demo-workflow; C:report-formats |
| F-1-90 | User-facing workflow uses “webhook fixture,” not “envelope.” | README/live terminology audit |
| F-1-91 | README says “export a Markdown report.” | C:report-formats |
| F-1-92 | The long browser-acceptance sentence remains absent. | README copy audit |
| F-1-93 | Copy controls say “Copy install command.” | B keyboard path |
| F-1-94 | Navigation says “Demo”; the primary action says “Try it with sample data.” | live Home screenshot |
| F-1-95 | Comparison-bench metaphor remains absent. | `.factory/copy-audit.md` |
| F-1-96 | Reset actions distinguish sample fixtures and the full demo. | C:browser-demo-isolation; live cold reset |
| F-1-97 | The install control copies the install command and announces the result. | B |
| F-1-98 | Landing, README, legal, errors, and docs use the terminology table consistently. | `.factory/copy-audit.md` |
| F-2-1 | Caller-selected vault directories are tested without creating the default vault. | C:vault-directory |
| F-2-2 | Plain demo output prints parseable, existing vault and report paths. | C:demo-output-paths |
| F-2-3 | README gives an installation instruction instead of an unsupported artifact promise. | README audit; clean-clone `cargo package --locked` |
| F-2-4 | Browser routing waits for settled scroll and H1 focus, repeated three times. | B |

## Final evidence

- All 20 claim commands passed separately from the final clean clone, followed by passing `npm test`.
- `npm run typecheck`, `npm run build`, `cargo fmt --check`, strict Clippy, and `cargo package --locked` passed. The build produced `dist/bin/vr` and `dist/site/`.
- The publishable crate contains 13 files and is 27.2 KiB compressed; factory screenshots and site evidence no longer enter the CLI crate.
- Live browser smoke passed desktop keyboard use, 390 px layout, three history/focus cycles, offline reload, every route, the external fragment, and axe with zero serious/critical violations.
- A separate cold live demo check passed completed output, memory-only isolation, reset, offline reload, and Start-for-real behavior.
- Home, Demo, Privacy, and Terms worker-verifier reports are under `.factory/polish-3-live/`; each records one H1, one main landmark, correct title/lang, labelled buttons, alt text, and zero console errors.
- The live status/header sweep returned 200 for every public route/asset, 404 for an unknown route, and the expected CSP, frame denial, `nosniff`, referrer, permissions, and HSTS headers.
- Built and live Home, Privacy, Terms, and service-worker files have matching SHA-256 hashes. The live worker identifies cache `version-replay-shell-v5`.
- `L` records 100 Performance, 100 Accessibility, 100 Best Practices, and 100 SEO; FCP 1.0 s, LCP 1.4 s, TBT 10 ms, and CLS 0. Lighthouse saved its completed audit before its known final full-page-screenshot tab crash.

No review finding remains unresolved.
