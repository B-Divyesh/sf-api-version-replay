# Perfection-loop polish 1

Candidate base: `e9f16103c1a0ac8ca40609f085c48d9ea83f06fd`  
Live target: <https://api-version-replay.sociobot.in/>

Evidence shorthand: `C:<id>` means `npm run test:claims -- <id>`; `B` means `npm run test:browser`; `A` means the Playwright axe WCAG A/AA scan; `S` means `npm run build:site` and the static-policy test. Screenshots are repository paths under `.factory/`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the metaphor with the requested job, audience, one sample action, and adjacent outcome. | C:route-metadata; `.factory/home.png` |
| F-1-2 | Added `vr demo`, bundled fixtures, temporary vault, loopback receiver, report, and recorded terminal output. | C:cli-demo-workflow; `.factory/demo-desktop.png` |
| F-1-3 | `?demo=1` bypasses browser storage and billing code; state lives in memory. | C:browser-demo-isolation |
| F-1-4 | Added `.factory/claims.json` and one tagged test for every retained claim. | `npm run test:claims`; tag/inventory cross-check |
| F-1-5 | Removed the false replay row and replay-result report copy. | C:report-formats; `.factory/home-report.png` |
| F-1-6 | Removed checkout, pricing, restore, license, batch-Pro, and paid JUnit paths while checkout is unavailable. | link crawl; `vr --help`; `rg 'checkout|Buy the Pro'` |
| F-1-7 | Added a concrete/moss 404 and Azure 404 response override. | S; `.factory/404.png`; live 404 probe |
| F-1-8 | Rewrote metadata and listed the retained end-to-end workflow claim. | C:cli-demo-workflow |
| F-1-9 | Retained only the credential-free statement and tested a cleared environment. | C:no-provider-credentials |
| F-1-10 | Replaced vague “keep” copy with the concrete save/import workflow. | C:cli-demo-workflow |
| F-1-11 | Split the compound hero claim into named, tested workflow steps. | C:cli-demo-workflow; C:exact-replay |
| F-1-12 | Removed the unqualified slogan; retained only tested privacy facts. | C:no-telemetry; C:no-provider-credentials |
| F-1-13 | Retained loopback refusal with hostname/IP/lookalike cases. | C:loopback-only |
| F-1-14 | Reworded to configured pre-write redaction and covered all defaults. | C:redaction-before-storage |
| F-1-15 | Removed the unavailable Pro/JUnit claim; Markdown and JSON remain. | C:report-formats |
| F-1-16 | Deleted the untestable “every result” sentence. | `.factory/copy-audit.md` |
| F-1-17 | Deleted the provider-sandbox availability claim. | `.factory/copy-audit.md` |
| F-1-18 | Uses one term, webhook fixture, and tests import plus capture. | C:capture-loopback; C:cli-demo-workflow |
| F-1-19 | Retained precise redaction-before-storage copy. | C:redaction-before-storage |
| F-1-20 | Retained the five comparison dimensions with a purpose-built fixture pair. | C:contract-dimensions |
| F-1-21 | Kept documented exit 3 and tests the built CLI. | C:exit-codes |
| F-1-22 | Reworded to saved request replay and verifies method, headers, and body. | C:exact-replay |
| F-1-23 | Retained the boundary without “before network” overstatement. | C:loopback-only |
| F-1-24 | Demo shell and sample work after an offline reload. | C:offline-demo |
| F-1-25 | Reworded browser privacy and instrumented all storage/requests. | C:browser-demo-isolation |
| F-1-26 | Removed “Zero uploads”; retained the precise sample-data statement. | C:browser-demo-isolation |
| F-1-27 | Removed broad “entirely” copy; retained observable page-local comparison. | C:browser-demo-isolation |
| F-1-28 | Report copy now promises structured changes only. | C:report-formats |
| F-1-29 | Removed all free-tier marketing claims. | `rg 'stay free|forever' site README.md` |
| F-1-30 | Removed the purchase and CI-upgrade claim. | `rg 'Pay once|checkout' site README.md` |
| F-1-31 | Removed price cards and merchant copy. | home link crawl; `.factory/home.png` |
| F-1-32 | Split the long README opening into a 15-word audience sentence. | `.factory/copy-audit.md`; C:cli-demo-workflow |
| F-1-33 | Split features into short sentences and listed their tests. | C:cli-demo-workflow; C:report-formats |
| F-1-34 | Credential-free demo runs with a cleared environment. | C:no-provider-credentials |
| F-1-35 | Telemetry statement has a dependency and network-path inventory. | C:no-telemetry |
| F-1-36 | Removed the incorrect Rust 1.82 assertion. | README copy audit |
| F-1-37 | Default payment, identity, cookie, and auth rules are table-tested. | C:redaction-before-storage |
| F-1-38 | Standardized on “fixture” and tests envelope/plain JSON parsing. | C:fixture-formats |
| F-1-39 | Records the complete received local request. | C:exact-replay |
| F-1-40 | Exercises allowed and refused URL boundaries. | C:loopback-only |
| F-1-41 | One-shot capture stores the path and redacts body/header secrets. | C:capture-loopback |
| F-1-42 | Narrowed JSON documentation to `vr --json demo`, which is parsed. | C:cli-demo-workflow |
| F-1-43 | Asserts exits 3, 4, and 1 with real processes. | C:exit-codes |
| F-1-44 | Inspects encrypted format and decrypts with the correct passphrase. | C:encrypted-storage |
| F-1-45 | Searches config and ciphertext for the supplied passphrase. | C:encrypted-storage |
| F-1-46 | Wrong passphrase cannot list/decrypt the fixture. | C:encrypted-storage |
| F-1-47 | Nested one-segment wildcard redaction is exercised. | C:redaction-before-storage |
| F-1-48 | Mixed-case custom header rule is exercised. | C:redaction-before-storage |
| F-1-49 | Plain pre-write redaction and encrypted ciphertext absence are tested. | C:redaction-before-storage; C:encrypted-storage |
| F-1-50 | Removed free-tier marketing from README. | copy search |
| F-1-51 | Removed Pro-tier marketing and the inaccessible paid commands. | `vr --help`; copy search |
| F-1-52 | Removed the dead purchase/restore direction. | site/README link crawl |
| F-1-53 | Removed license-cache copy and runtime path. | `rg 'license|verdict' site/src README.md src/main.rs` |
| F-1-54 | Removed the unneeded test-suite marketing claim; commands remain documented. | clean-clone `npm test` |
| F-1-55 | Removed the build-output promise from prose; both artifacts are verified directly. | clean-clone `npm run build` |
| F-1-56 | Replaced the browser-test marketing sentence with actual browser checks. | B; A |
| F-1-57 | Narrowed local behavior to the explicit replay network path. | C:no-telemetry; C:loopback-only |
| F-1-58 | Split replay, redaction, and encryption into separate tested claims. | C:loopback-only; C:redaction-before-storage; C:encrypted-storage |
| F-1-59 | Added the standard MIT heading and a license-content test. | C:mit-license |
| F-1-60 | Privacy copy now matches the source network inventory. | C:no-telemetry |
| F-1-61 | Privacy storage/redaction statements use temporary-vault evidence. | C:cli-demo-workflow; C:redaction-before-storage |
| F-1-62 | Privacy encryption/passphrase statements map to one claim. | C:encrypted-storage |
| F-1-63 | Removed billing reconciliation; only explicit loopback replay remains. | C:no-telemetry; C:loopback-only |
| F-1-64 | Website storage is now only `vr_theme` outside demo; no URL token path remains. | C:browser-storage-scope |
| F-1-65 | Demo request bodies and origins are recorded. | C:browser-demo-isolation |
| F-1-66 | Cookie, storage, and request inventories cover the normal site flow. | C:browser-storage-scope |
| F-1-67 | Hash navigation pushes history, restores scroll, focuses headings, and announces changes; legal loads focus H1. | B routing assertions |
| F-1-68 | Added per-route canonical/OG/Twitter metadata, social card, SVG favicon, and Apple icon. | C:route-metadata; S |
| F-1-69 | All routes share Demo/How/Privacy header and attribution/version footer. | B; route screenshots |
| F-1-70 | External names now say GitHub or “opens in a new site.” | site link crawl |
| F-1-71 | Sitemap now lists home, isolated demo, privacy, and terms; 404 is intentionally `noindex`. | `sitemap.xml`; live URL checks |
| F-1-72 | Replaced decorative lore with `LOCAL CLI · VERSION 0.1`. | `.factory/copy-audit.md` |
| F-1-73 | Changed image labels to `Saved webhook fixture` and `Example data`. | `.factory/home.png` |
| F-1-74 | Renamed the section `How Version Replay works` with a literal workflow. | copy audit |
| F-1-75 | Uses `Save and redact a fixture`. | copy audit |
| F-1-76 | Uses `Compare version changes`. | copy audit |
| F-1-77 | Uses `Replay it to localhost`. | copy audit |
| F-1-78 | Uses `Browser sample · compares JSON only`. | `.factory/demo-desktop.png` |
| F-1-79 | Uses `Compare two sample webhook fixtures`. | C:route-metadata |
| F-1-80 | Empty state is `No comparison yet`. | B |
| F-1-81 | Uses `fixtures`, `type`, and `value` consistently. | copy audit terminology table |
| F-1-82 | Uses `Export contract changes`. | `.factory/home-report.png` |
| F-1-83 | Removed pricing and its slogan. | home link crawl |
| F-1-84 | Removed generic optional-upgrade copy. | copy search |
| F-1-85 | Deleted the mood slogan. | copy search |
| F-1-86 | Final action is `Install Version Replay`. | `.factory/home.png` |
| F-1-87 | Footer now says exactly what is compared and where. | all route screenshots |
| F-1-88 | README audience sentence is below 22 words. | copy audit |
| F-1-89 | README feature sentence was split and simplified. | copy audit; claims inventory |
| F-1-90 | Replaced “envelopes” in user-facing workflow with “webhook fixtures.” | terminology search |
| F-1-91 | Uses `export a Markdown report`. | README; C:report-formats |
| F-1-92 | Removed the long browser-acceptance sentence. | README copy audit |
| F-1-93 | Every copy control says `Copy install command`. | B keyboard/copy controls |
| F-1-94 | Header action says `Demo`; first action says `Try it with sample data`. | `.factory/home.png` |
| F-1-95 | Removed comparison-bench wording. | terminology search |
| F-1-96 | Reset actions say `Reset sample fixtures` or `Reset demo`. | C:browser-demo-isolation |
| F-1-97 | Removed the misleading install jump; the control copies the command. | B |
| F-1-98 | Standardized terms in landing, README, legal pages, errors, and docs. | `.factory/copy-audit.md` terminology table |

## Final evidence

Local and clean-clone commands, deployed hashes, headers, live 404 status, Lighthouse scores, and cold-live screenshots are recorded in `.factory/handoff.md` after deployment.
