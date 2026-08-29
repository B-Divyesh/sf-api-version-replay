# Perfection-loop polish 4

Released candidate: `7284df2b8cc6eca20d737912e8094fc63fefccbf`  
Review base: `4517d49b799f96958bdd08fb81d60a3dc6eafb4e`  
Repair commit: `23b026cdec23b5f2c948a190a5db40785d6215de`  
Deployment: `9168ee63-d78c-4cd1-8943-987fe1153494`  
Live target: <https://api-version-replay.sociobot.in/>

The final repair was verified from clean clone `/tmp/api-version-replay-polish4.eax62t`. Every command in `.factory/claims.json` passed separately before the aggregate suite. The deployed files match the local build.

Evidence shorthand:

- `C:<id>`: clean-clone `npm run test:claims -- <id>`.
- `T`: clean-clone `npm test`; `B`: enhanced browser smoke against the live URL.
- `H`, `D`, `P`, `LGL`, `N`: live Home, Demo, Privacy, Terms, and 404 checks. Screenshots are under `.factory/polish-4-live/{home,demo,privacy,terms,not-found}/`.
- `R`: cold live route/metadata/link crawl; `S`: live status, security-header, and build-hash sweep.
- `LH`: `.factory/polish-4-live/lighthouse.json`.

## Review 4 findings

| Finding ID | Change made | Evidence |
| --- | --- | --- |
| F-4-1 / F-1-67 | Added `pageshow` restoration for full-document and back-forward-cache navigation. Home, Demo, Privacy, Terms, and 404 focus the current H1 without changing restored scroll and refresh the polite route announcement. The browser suite now checks Home → Privacy → Back and Home → Demo → Back at 1280 and 390 px. | `B` (`assertFullPageBack`); `.factory/polish-4-live/back-focus-mobile.png`; cold live `/privacy/` and `/?demo=1` Back checks. |
| F-4-2 | Made the overflowing terminal output and comparison table focusable, named regions with inset focus rings. Added keyboard scrolling plus light/dark mobile axe coverage. | `B` (`runMobile`); `.factory/polish-4-live/terminal-focus-mobile.png`; `.factory/polish-4-live/result-focus-mobile.png`; live Home and Demo at 390 px. |
| F-4-3 / F-1-68 | Added the 404 canonical plus Twitter title, description, and image. Extended `route-metadata` through the 404 and asserted `noindex` plus the host rewrite. | `C:route-metadata`; `.factory/polish-4-live/not-found/screenshot-mobile.png`; `R` returned 404 for `/polish-4-missing` with complete metadata. |
| F-4-4 | Removed the untestable future-publication sentence and its empty Terms section. Added a source regression test. | `T` test `keeps the terms free of an untestable future-publication promise`; `.factory/polish-4-live/terms/screenshot-desktop.png`; live `/terms/` text check. |
| F-4-5 | Moved the recorded, completed CLI result directly after the first-screen product facts and before the three-step explanation. Added an order regression test. | `T` test `shows the recorded product result before the workflow explanation`; `.factory/polish-4-live/home/screenshot-desktop.png`; live `/` order check. |

## Earlier finding map

| Finding ID | Current change/state | Evidence |
| --- | --- | --- |
| F-1-1 | The first screen names the localhost webhook-version job, engineer audience, one sample action, its result, and three facts. | `C:route-metadata`; `H`; live `/`. |
| F-1-2 | `?demo=1` opens on the completed CLI workflow; `vr demo` uses bundled fixtures and a new temporary vault. | `C:primary-demo-workflow`; `C:cli-demo-workflow`; `D`; live `/?demo=1`. |
| F-1-3 | Demo mode bypasses real browser storage, billing state, and external requests. | `C:browser-demo-isolation`; `D`; cold live Demo recorded zero storage reads/writes. |
| F-1-4 | `.factory/claims.json` has 20 entries with exactly one tagged test each. | `T` claims inventory; all 20 `C` commands; `D`; live Demo. |
| F-1-5 | Report copy and preview contain only structured changes the CLI emits. | `C:report-formats`; `H`; live `/`. |
| F-1-6 | Dead checkout, paid pricing, restore, license, and unavailable Pro copy remain absent. | `R` link/text crawl; `H`; live `/`. |
| F-1-7 | Unknown routes return the designed concrete/moss 404 with recovery. | `R`; `N`; live `/polish-4-missing` returned 404. |
| F-1-8 | Metadata workflow wording maps to the complete CLI demo. | `C:cli-demo-workflow`; `H`; live `/`. |
| F-1-9 | The bundled workflow runs without provider credentials. | `C:no-provider-credentials`; `D`; live `/?demo=1`. |
| F-1-10 | Vague “keep” copy remains replaced by observable save/import wording. | `C:cli-demo-workflow`; `H`; live `/`. |
| F-1-11 | Hero workflow promises are split into directly tested steps. | `C:cli-demo-workflow`; `C:exact-replay`; `H`; live `/`. |
| F-1-12 | The unqualified account/tunnel/telemetry slogan remains absent. | `C:no-telemetry`; `C:no-provider-credentials`; `P`; live `/privacy/`. |
| F-1-13 | Public-host refusal is precise and covers IP, hostname, and lookalike cases. | `C:loopback-only`; `H`; live `/`. |
| F-1-14 | Configured pre-write redaction covers every documented default. | `C:redaction-before-storage`; `H`; live `/`. |
| F-1-15 | Unsupported Pro/JUnit wording is absent; Markdown and JSON remain. | `C:report-formats`; `H`; live `/`. |
| F-1-16 | The universal “every result” claim remains deleted. | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-17 | The provider-sandbox availability claim remains deleted. | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-18 | Import and capture use “webhook fixture” and both paths are tested. | `C:fixture-formats`; `C:capture-loopback`; `H`; live `/`. |
| F-1-19 | Body and header redaction before storage is stated precisely. | `C:redaction-before-storage`; `P`; live `/privacy/`. |
| F-1-20 | Method, path, header, JSON type, and value comparison are exercised. | `C:contract-dimensions`; `H`; live `/`. |
| F-1-21 | Changed contracts exit 3 in the built CLI. | `C:exit-codes`; `D`; live `/?demo=1`. |
| F-1-22 | Replay verifies the saved method, headers, and body at loopback. | `C:exact-replay`; `H`; live `/`. |
| F-1-23 | Public destinations are rejected without an overbroad timing claim. | `C:loopback-only`; `H`; live `/`. |
| F-1-24 | Demo result and banner remain available after an offline reload. | `C:offline-demo`; `D`; cold live offline reload. |
| F-1-25 | Browser privacy wording matches instrumented storage and requests. | `C:browser-demo-isolation`; `D`; cold live Demo instrumentation. |
| F-1-26 | “Zero uploads” remains replaced by precise sample-content wording. | `C:browser-demo-isolation`; `D`; live `/?demo=1`. |
| F-1-27 | Broad “entirely in your browser” wording remains narrowed to observable comparison behavior. | `C:browser-demo-isolation`; `D`; live `/?demo=1`. |
| F-1-28 | The report promises and shows structured contract changes only. | `C:report-formats`; `H`; live `/`. |
| F-1-29 | Unsupported free-tier marketing remains absent. | `R`; `H`; live `/`. |
| F-1-30 | Unsupported purchase and pay-once wording remains absent. | `R`; `H`; live `/`. |
| F-1-31 | Price cards and unconfigured merchant actions remain absent. | `R`; `H`; live `/`. |
| F-1-32 | README opens with a short audience/job sentence. | `C:cli-demo-workflow`; `.factory/copy-audit.md`; `H`; live source link. |
| F-1-33 | README workflow/report sentences remain split and tested. | `C:cli-demo-workflow`; `C:report-formats`; `H`; live source link. |
| F-1-34 | Demo runs with a cleared environment and no provider credentials. | `C:no-provider-credentials`; `D`; live `/?demo=1`. |
| F-1-35 | The telemetry statement maps to a dependency and network-path inventory. | `C:no-telemetry`; `P`; live `/privacy/`. |
| F-1-36 | The unsupported Rust 1.82 statement remains removed. | Clean-clone build; `H`; live source link. |
| F-1-37 | Identity, payment, cookie, and authorization defaults are table-tested. | `C:redaction-before-storage`; `P`; live `/privacy/`. |
| F-1-38 | Request-envelope and plain-body fixture formats are tested. | `C:fixture-formats`; `D`; live `/?demo=1`. |
| F-1-39 | The loopback receiver compares the complete stored request. | `C:exact-replay`; `H`; live `/`. |
| F-1-40 | Replay tests cover allowed and refused destinations. | `C:loopback-only`; `P`; live `/privacy/`. |
| F-1-41 | One-shot capture stores a redacted request and exits. | `C:capture-loopback`; `H`; live `/`. |
| F-1-42 | JSON documentation is limited to parsed `vr --json demo` output. | `C:cli-demo-workflow`; `D`; live `/?demo=1`. |
| F-1-43 | Exit codes 1, 3, and 4 are asserted through real CLI processes. | `C:exit-codes`; `D`; live `/?demo=1`. |
| F-1-44 | Encrypted format and successful decryption are inspected. | `C:encrypted-storage`; `P`; live `/privacy/`. |
| F-1-45 | The passphrase is absent from vault configuration and ciphertext. | `C:encrypted-storage`; `P`; live `/privacy/`. |
| F-1-46 | Missing and wrong passphrases cannot read fixture contents. | `C:encrypted-storage`; `P`; live `/privacy/`. |
| F-1-47 | One-segment wildcard body redaction is exercised. | `C:redaction-before-storage`; `P`; live `/privacy/`. |
| F-1-48 | Case-insensitive custom-header redaction is exercised. | `C:redaction-before-storage`; `P`; live `/privacy/`. |
| F-1-49 | Secret absence is checked in plain and encrypted writes. | `C:redaction-before-storage`; `C:encrypted-storage`; `P`; live `/privacy/`. |
| F-1-50 | Unsupported free-tier feature marketing remains absent. | `R`; `H`; live `/`. |
| F-1-51 | Unsupported Pro commands and feature marketing remain absent. | `T`; `H`; live `/`. |
| F-1-52 | Dead checkout and restore directions remain absent. | `R`; `H`; live `/`. |
| F-1-53 | Unsupported license-cache behavior remains absent. | `T`; `P`; live `/privacy/`. |
| F-1-54 | Test-suite marketing is absent; documented test commands pass. | `T`; `H`; live source link. |
| F-1-55 | Build-output marketing is absent; both required artifacts were verified directly. | Clean-clone `npm run build`; `H`; live source link. |
| F-1-56 | Browser behavior is proved by the browser suite, including mobile axe. | `B`; `H`; live `/`. |
| F-1-57 | Network copy is limited to explicit loopback replay. | `C:no-telemetry`; `C:loopback-only`; `P`; live `/privacy/`. |
| F-1-58 | Replay, redaction, and encryption remain separate tested claims. | `C:loopback-only`; `C:redaction-before-storage`; `C:encrypted-storage`; `P`; live `/privacy/`. |
| F-1-59 | The standard MIT license ships and is tested. | `C:mit-license`; `H`; live `/`. |
| F-1-60 | The remaining telemetry statement is inventoried; the account clause remains absent. | `C:no-telemetry`; `P`; live `/privacy/`. |
| F-1-61 | Vault storage and pre-write redaction match temporary-vault evidence. | `C:cli-demo-workflow`; `C:redaction-before-storage`; `P`; live `/privacy/`. |
| F-1-62 | Encryption and passphrase statements map to cryptographic/storage tests. | `C:encrypted-storage`; `P`; live `/privacy/`. |
| F-1-63 | Network policy describes explicit loopback replay only. | `C:no-telemetry`; `C:loopback-only`; `P`; live `/privacy/`. |
| F-1-64 | Normal website storage is only the theme; Demo touches no storage. | `C:browser-storage-scope`; `C:browser-demo-isolation`; `P`; live `/privacy/`. |
| F-1-65 | Demo request origins and bodies are instrumented. | `C:browser-demo-isolation`; `D`; cold live Demo instrumentation. |
| F-1-66 | Cookie, storage, and request inventories cover normal browsing. | `C:browser-storage-scope`; `P`; live `/privacy/`. |
| F-1-67 | Hash navigation and full-document Back now restore scroll, H1 focus, and announcement. | `B`; `.factory/polish-4-live/back-focus-mobile.png`; live `/privacy/` and `/?demo=1` Back checks. |
| F-1-68 | Every route, including the 404, has complete title, canonical, OG/Twitter art, and icons. | `C:route-metadata`; `N`; live `/polish-4-missing`. |
| F-1-69 | Home, Demo, Privacy, Terms, and 404 share the header/footer shell. | `B`; `H`, `D`, `P`, `LGL`, `N`; all live routes. |
| F-1-70 | External link names identify GitHub, and the guide fragment resolves. | `B` external-fragment stage; `H`; live GitHub link. |
| F-1-71 | Sitemap contains every indexable route; the noindex 404 returns 404 for unknown paths. | `R`; `N`; live `/sitemap.xml` and `/polish-4-missing`. |
| F-1-72 | Decorative CLI lore remains replaced by a literal version label. | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-73 | Image labels name a saved webhook fixture and example data. | `H`; live `/`. |
| F-1-74 | The workflow section has a literal product-specific heading. | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-75 | Step one says “Save and redact a fixture.” | `H`; live `/`. |
| F-1-76 | Step two says “Compare version changes.” | `H`; live `/`. |
| F-1-77 | Step three says “Replay it to localhost.” | `H`; live `/`. |
| F-1-78 | The browser tool is labelled as a JSON-only sample. | `D`; live `/?demo=1`. |
| F-1-79 | The browser heading says “Compare two sample webhook fixtures.” | `C:route-metadata`; `D`; live `/?demo=1`. |
| F-1-80 | Empty state says “No comparison yet.” | `B`; `H`; live `/`. |
| F-1-81 | UI consistently uses fixture, type, and value. | `.factory/copy-audit.md`; `D`; live `/?demo=1`. |
| F-1-82 | The report section is named “Export contract changes.” | `H`; live `/`. |
| F-1-83 | Pricing and its slogan remain absent. | `R`; `H`; live `/`. |
| F-1-84 | Generic optional-upgrade copy remains absent. | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-85 | The mood slogan remains absent. | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-86 | Final action is “Install Version Replay.” | `H`; live `/`. |
| F-1-87 | Footer says what is compared and that it runs on the user's machine. | `H`, `D`, `P`, `LGL`, `N`; all live routes. |
| F-1-88 | README audience sentence remains below 22 words. | `.factory/copy-audit.md`; `H`; live source link. |
| F-1-89 | README feature copy is split and mapped to claims. | `C:cli-demo-workflow`; `C:report-formats`; `.factory/copy-audit.md`; live source link. |
| F-1-90 | User-facing workflow uses “webhook fixture,” not “envelope.” | `.factory/copy-audit.md`; `H`; live `/`. |
| F-1-91 | README says “export a Markdown report.” | `C:report-formats`; `H`; live source link. |
| F-1-92 | The long browser-acceptance sentence remains absent. | `.factory/copy-audit.md`; `H`; live source link. |
| F-1-93 | Copy controls say “Copy install command.” | `B`; `H`; live `/`. |
| F-1-94 | Navigation says “Demo”; primary action says “Try it with sample data.” | `H`; live `/`. |
| F-1-95 | Comparison-bench metaphor remains absent. | `.factory/copy-audit.md`; `D`; live `/?demo=1`. |
| F-1-96 | Reset actions distinguish sample fixtures and the full Demo. | `C:browser-demo-isolation`; `D`; cold live reset. |
| F-1-97 | Install control copies the command and announces the result. | `B`; `H`; live `/`. |
| F-1-98 | Landing, README, legal, errors, and docs use the terminology table consistently. | `.factory/copy-audit.md`; `H`, `P`, `LGL`; live routes. |
| F-2-1 | Caller-selected vault directories are tested without creating the default vault. | `C:vault-directory`; `H`; live `/`. |
| F-2-2 | Plain Demo output prints parseable, existing vault and report paths. | `C:demo-output-paths`; `D`; live `/?demo=1`. |
| F-2-3 | README gives an installation instruction, not an unsupported artifact promise. | Clean-clone `cargo package --locked`; `H`; live source link. |
| F-2-4 | Browser routing waits for settled scroll and focus and now covers cross-document Back too. | `B`; `.factory/polish-4-live/back-focus-mobile.png`; live navigation. |
| F-3-1 | GitHub guide link targets `#use-your-fixtures`, with source and browser fragment validation. | `T` fragment test; `B`; `H`; live GitHub fragment. |
| F-3-2 / F-1-60 | The unlisted “no account system” clause remains removed. | `C:no-telemetry`; `P`; live `/privacy/`. |
| F-3-3 | The unsupported “short-lived” retention assurance remains removed. | `T`; `P`; live `/privacy/`. |

## Final evidence

- Clean clone `/tmp/api-version-replay-polish4.eax62t` at `23b026cdec23b5f2c948a190a5db40785d6215de`: all 20 claim commands passed separately.
- `npm test`, `npm run typecheck`, `npm run build`, `cargo fmt --check`, strict Clippy, and `cargo package --locked` passed. The package has 13 files and is 27.2 KiB compressed.
- The clean-clone and live browser suites passed desktop keyboard use, 390 px layout, both mobile themes, keyboard-scroll regions, hash and document Back, route focus/announcements, offline reload, and axe with zero serious/critical violations.
- Live Demo showed the completed CLI sample, reset correctly, used zero browser-storage calls, preserved seeded real keys, set no cookies, made no cross-origin request, uploaded no fixture text, exited through Start for real, and reloaded offline.
- Live route checks returned 200 for Home, Demo, Privacy, Terms, and public assets; an unknown route returned 404 with complete route metadata. Eleven discovered links resolved.
- Home, Privacy, Terms, 404, and service-worker SHA-256 hashes match `dist/site`. Live responses include CSP, frame denial, HSTS, `nosniff`, referrer, and permissions headers.
- Initial production assets are 7,325 bytes JavaScript plus two small route/style loaders and 18,345 bytes CSS, below the product budgets.
- `LH` records 100 Performance, 100 Accessibility, 100 Best Practices, and 100 SEO; FCP 0.9 s, LCP 1.4 s, TBT 0 ms, and CLS 0.

No finding remains unresolved.
