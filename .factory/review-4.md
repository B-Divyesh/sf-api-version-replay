# Adversarial first-read review 4 — Version Replay

**Verdict: FAIL**

Reviewed 2026-08-29 UTC against live <https://api-version-replay.sociobot.in/> and repository commit `7284df2b8cc6eca20d737912e8094fc63fefccbf`. Live Home, Privacy, Terms, and 404 HTML are byte-for-byte identical to the clean-clone build. This was a fresh review, not a diff-only check.

The first screen, one-click demo, demo isolation, CLI sample, declared claim tests, visual identity, dead-link crawl, and production build pass. Five findings remain. Three are blocking: a reopened Back/focus defect, a serious mobile keyboard defect, and a reopened metadata defect on the 404.

## Cold first screen: 30-second result

Fresh Chromium contexts at 390 × 844 and 1440 × 900 opened `/` with empty browser storage. No scrolling occurred before this assessment.

| Question | Answer visible at both sizes | Result |
| --- | --- | --- |
| What does it do? | `Test old webhook versions against localhost` | Clear. |
| For whom? | `For engineers checking an integration before upgrading a provider API.` | Clear. |
| What should I click first? | `Try it with sample data` | Clear. |

The adjacent result text is `See saved versions, contract changes, and local replay results.` The first screen also shows `Runs locally · loopback replay only · MIT licensed`. Both loads were same-origin only, had no console or page errors, left storage and cookies empty, and had no horizontal page overflow. Screenshots were captured at `/tmp/review4-home-mobile.png` and `/tmp/review4-home-desktop.png`.

## Findings, ordered by severity

### Blocking

#### F-4-1 / F-1-67 (reopened) — Back navigation loses focus on full-page routes

- **Exact location:** Home → `Privacy`, Home → `Try it with sample data`, then browser Back.
- **Evidence:** Privacy and Demo initially focus their H1. After Back to Home, the URL and scroll position restore correctly, but `document.activeElement` is `BODY`, not `#hero-title`. This reproduced at desktop and 390 px. The hash-only `How it works` cycle passes because `main.ts` handles `popstate`; cross-document Back uses the browser page cache and has no equivalent restoration hook. `page.ts` only focuses the H1 during initial script evaluation.
- **Why this blocks:** Earlier F-1-67 required Back/forward to restore scroll and focus. The current implementation fixes only in-document hash history. A keyboard or screen-reader user returning from a real route is dropped at the document instead of the page heading. Under the history rule, this half-fix is blocking again.
- **Concrete fix:** On `pageshow`/page-cache restoration, focus `#hero-title` or the current route H1 and update the live region without changing the restored scroll. Add browser tests for Home → Privacy → Back and Home → Demo → Back at desktop and 390 px, asserting URL, scroll, H1 focus, and announcement.

#### F-4-2 — Mobile horizontal result regions are not keyboard reachable

- **Exact location:** Home/Demo `.terminal-recording pre`; Demo `.result-table-wrap` at 390 px.
- **Evidence:** axe 4.11.4 reports serious `scrollable-region-focusable` violations in light and dark themes. The terminal is 354 px wide with 689 px of scrollable content; the demo result is 354 px wide with 550 px of content. Both have `tabIndex === -1` and no focusable descendant. Home has one serious node; Demo has two. Desktop axe passes because the regions do not overflow there.
- **Why this blocks:** Keyboard users, particularly in Safari, cannot reach or horizontally scroll content that contains the command output and comparison result. This violates the non-negotiable keyboard baseline on the requested phone viewport.
- **Concrete fix:** Make each overflow container keyboard-focusable with a useful accessible name and visible focus state, or reflow the content so it does not require horizontal scrolling. Run axe at 390 px on Home and Demo in both themes. Update `scripts/browser-smoke.mjs`; its current mobile branch checks only overflow, banner visibility, and console errors, while axe runs at desktop width.

#### F-4-3 / F-1-68 (reopened) — The 404 still lacks required route metadata

- **Exact location:** Live unknown route such as `/review-4-missing`; source `site/404.html:3`.
- **Evidence:** The 404 correctly returns HTTP 404 and has a title, description, OG title/description/image, favicon, and Apple icon. It has no canonical link and only `twitter:card`; `twitter:title`, `twitter:description`, and `twitter:image` are absent. Home, Demo, Privacy, and Terms contain all of these fields.
- **Why this blocks:** Original F-1-68 covered all live routes and required a per-route canonical plus complete OG/Twitter metadata. The 404 was only partly repaired, while later polish records declared the metadata finding closed. The review instructions make an unfixed earlier finding blocking again.
- **Concrete fix:** Add a canonical for the designed 404 (for example `/404.html`) and Twitter title, description, and product social image. Extend `route-metadata` to cover `404.html` and assert its HTTP/noindex treatment separately from indexed routes.

### Minor

#### F-4-4 — The Terms page contains an unlisted future-process claim

- **Exact quote/location:** Terms → Changes: `Material changes to these terms will appear here with a new effective date.`
- **Why this is a finding:** This is a promise a visitor can rely on about future publication behavior. It has no `.factory/claims.json` entry or clean-sandbox test, and a current build cannot prove how a future legal change will be published.
- **Concrete fix:** Delete the sentence and its otherwise empty section. If the promise must remain, define a release-policy check that blocks any material Terms change unless the effective date changes, then inventory that observable rule.

#### F-4-5 — The landing page puts instructions before the product preview

- **Exact location:** Home order is hero/facts → `How Version Replay works` → browser sample → recorded CLI demo.
- **Why this is a finding:** The mandatory site skeleton places the product itself or a live preview immediately after the first screen, before the three-step explanation. Here, a visitor must read the abstract workflow before reaching either working sample. The hero art is product-specific art, not a product preview.
- **Concrete fix:** Move `Replay the complete CLI sample` or the browser sample directly below the first-screen facts. Follow it with `How Version Replay works`, then limits/privacy and the install action.

## Copy audit

Counting method: a hyphenated term, URL, or code token is one word; punctuation and separators are not words. Metadata copy is included because visitors see it in search/share contexts. No landing or README sentence exceeds 22 words. No banned marketing adjective appears. Technical terms such as CLI, JSON, localhost, loopback, AES-256-GCM, and Argon2id fit the named engineering audience.

### Landing-page sentences

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| L-01 | 18 | Test saved webhook versions against localhost with a local CLI that redacts, compares, replays, and reports contract changes. | Listed claims. |
| L-02 | 9 | Save, compare, and replay old webhook versions against localhost. | Listed claims. |
| L-03 | 6 | Test old webhook versions against localhost. | Clear H1. |
| L-04 | 10 | For engineers checking an integration before upgrading a provider API. | Clear audience. |
| L-05 | 9 | See saved versions, contract changes, and local replay results. | `primary-demo-workflow`. |
| L-06 | 7 | Runs locally · loopback replay only · MIT licensed | Listed claims. |
| L-07 | 5 | You choose the vault directory. | `vault-directory`. |
| L-08 | 6 | Configured fields are removed before storage. | `redaction-before-storage`. |
| L-09 | 4 | Public destinations are rejected. | `loopback-only`. |
| L-10 | 9 | Use a saved webhook fixture for each provider version. | `cli-demo-workflow`. |
| L-11 | 10 | Import JSON or capture one request on a loopback listener. | `fixture-formats`, `capture-loopback`. |
| L-12 | 8 | List method, path, header, type, and value changes. | `contract-dimensions`. |
| L-13 | 10 | Send either saved request to code running on your machine. | `exact-replay`. |
| L-14 | 5 | Available after this page loads | `offline-demo`. |
| L-15 | 8 | Edit either sample webhook fixture, then compare them. | Clear instruction. |
| L-16 | 9 | Sample contents are not sent to a server. | `browser-demo-isolation`. |
| L-17 | 5 | The browser sample compares JSON. | Accurate scope. |
| L-18 | 8 | Run the CLI demo for import and replay. | Clear instruction. |
| L-19 | 3 | No comparison yet. | Clear empty state. |
| L-20 | 9 | Compare the fixtures to list type and value changes. | Clear next action. |
| L-21 | 14 | `vr demo` creates a new temporary vault and leaves its report there for inspection. | `cli-demo-workflow`. |
| L-22 | 7 | Recorded from the bundled payment webhook fixtures. | `cli-demo-workflow`. |
| L-23 | 11 | The Markdown report names both versions and lists their structured changes. | `report-formats`. |
| L-24 | 8 | It does not send replays to public hosts. | `loopback-only`. |
| L-25 | 7 | It does not replace provider integration tests. | Useful limit. |
| L-26 | 11 | The browser sample uses only the text shown on this page. | `browser-demo-isolation`. |
| L-27 | 11 | Real webhook data belongs in the CLI, not this sample page. | Useful privacy guidance. |
| L-28 | 9 | Compare and replay saved webhook versions on your machine. | Listed claims. |

### Dynamic landing-page sentences

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| D-01 | 5 | Both webhook fixtures need JSON. | Clear cause. |
| D-02 | 2 | Fixtures match. | Clear result. |
| D-03 | 7 | No type or value changes were found. | Clear result. |
| D-04 | 6 | The fixtures could not be compared. | Clear fallback. |
| D-05 | 2 | Comparison stopped. | Clear error heading. |
| D-06 | 6 | Check commas, quotes, and closing braces. | Actionable recovery. |
| D-07 | 6 | Install command copied to the clipboard. | Clear confirmation. |
| D-08 | 7 | Copy this command: `cargo install --git https://github.com/B-Divyesh/sf-api-version-replay` | Clear fallback. |
| D-09 | 6 | Demo — sample data, nothing is saved | `browser-demo-isolation`. |
| D-10 | 4 | Offline · sample still works | `offline-demo`. |

Headings and actions are literal and short: `Save, compare, then replay`, `Compare two sample webhook fixtures`, `Replay the complete CLI sample`, `Export contract changes`, `What it does not do`, and `Install Version Replay`. Actions name their result: `Try it with sample data`, `Copy install command`, `Compare sample fixtures`, `Reset sample fixtures`, `Reset demo`, `Start for real`, and `Return home`. The theme control's accessible name is `Use dark theme` or `Use light theme`. No copy rewrite is required for these controls.

### README sentences

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| R-01 | 17 | Version Replay is a local CLI for engineers testing older webhook contracts before a provider API upgrade. | `cli-demo-workflow`. |
| R-02 | 7 | It saves redacted fixtures by provider version. | `cli-demo-workflow`. |
| R-03 | 12 | Compare changes, replay requests to localhost, and export Markdown or JSON reports. | Listed claims. |
| R-04 | 10 | The bundled demo creates a new temporary vault each time. | `cli-demo-workflow`. |
| R-05 | 14 | It imports two payment webhook fixtures, compares them, replays both, and writes a report. | `cli-demo-workflow`. |
| R-06 | 9 | The command prints the temporary vault and report paths. | `demo-output-paths`. |
| R-07 | 7 | The browser sample is available at `https://api-version-replay.sociobot.in/?demo=1`. | Live route verified. |
| R-08 | 6 | Install the CLI from this checkout. | Clear instruction. |
| R-09 | 10 | Registry publishing and release archives are managed outside this repository. | Repository scope. |
| R-10 | 9 | Create a vault, then import two saved webhook fixtures. | Clear instruction. |
| R-11 | 8 | Compare the versions and export a Markdown report. | `report-formats`. |
| R-12 | 8 | Replay one saved request to a local service. | `exact-replay`. |
| R-13 | 8 | Capture one incoming request on a loopback listener. | `capture-loopback`. |
| R-14 | 7 | Place `--json` before `demo` to receive JSON. | `cli-demo-workflow`. |
| R-15 | 11 | A changed diff exits `3`; a rejected replay response exits `4`. | `exit-codes`. |
| R-16 | 9 | A fixture can contain `method`, `path`, `headers`, and `body`. | `fixture-formats`. |
| R-17 | 11 | A plain JSON file becomes the body of a `POST /` fixture. | `fixture-formats`. |
| R-18 | 12 | Default rules redact common identity, payment, cookie, and authorization fields before storage. | `redaction-before-storage`. |
| R-19 | 6 | Add rules when creating a vault. | Clear instruction. |
| R-20 | 8 | Encrypted vaults use AES-256-GCM with an Argon2id-derived key. | `encrypted-storage`. |
| R-21 | 13 | The passphrase stays in the environment and is not written to the vault. | `encrypted-storage`. |
| R-22 | 5 | Replay accepts loopback destinations only. | `loopback-only`. |
| R-23 | 12 | The CLI has no telemetry client and does not require provider credentials. | `no-telemetry`, `no-provider-credentials`. |
| R-24 | 6 | Create the registry package without publishing. | Clear instruction. |
| R-25 | 7 | Use `dist/site` as the static deployment directory. | Clear deployment instruction. |
| R-26 | 6 | The factory manages infrastructure and DNS. | Repository boundary. |
| R-27 | 6 | See the privacy policy and terms. | Clear links. |
| R-28 | 1 | MIT. | License label; `mit-license`. |

README headings name their sections. The same terms are used consistently: `webhook fixture`, `version`, `contract changes`, `report`, `browser sample`, `demo`, `vault`, and `loopback`/`localhost`.

## Demo and sandbox behavior

- One activation of `Try it with sample data` opens `/?demo=1`.
- The first 390 px demo screen already shows the completed `vr demo` output: two redacted imports, five contract changes, two loopback HTTP 204 results, and the Markdown report path.
- The persistent banner says `Demo — sample data, nothing is saved` and contains working `Reset demo` and `Start for real` actions.
- Editing the old fixture to `{}` and resetting restores the original payment fixture and comparison.
- A context seeded with `real:user-data` and `vr_theme` recorded zero local/session-storage calls in Demo. Both real keys stayed unchanged. There were no cookies, cross-origin requests, or request bodies containing fixture data.
- After one online load, the live demo reloaded offline with its banner and completed comparison.
- A direct CLI run from fresh directory `/tmp/review4-cli-demo.*` created a separate `/tmp/version-replay-demo-*` vault and report, printed both paths, redacted the stored authorization/email fields, replayed both versions to loopback, and created nothing in the caller directory.

## Claim results

Every command in `.factory/claims.json` ran separately from clean clone `/tmp/api-version-replay-review4.4ozSuZ` after `npm ci`.

| Claim | Result |
| --- | --- |
| `cli-demo-workflow` | PASS |
| `primary-demo-workflow` | PASS |
| `demo-output-paths` | PASS |
| `vault-directory` | PASS |
| `redaction-before-storage` | PASS |
| `loopback-only` | PASS |
| `encrypted-storage` | PASS |
| `capture-loopback` | PASS |
| `report-formats` | PASS |
| `contract-dimensions` | PASS |
| `exact-replay` | PASS |
| `fixture-formats` | PASS |
| `exit-codes` | PASS |
| `no-provider-credentials` | PASS |
| `no-telemetry` | PASS |
| `mit-license` | PASS |
| `browser-demo-isolation` | PASS |
| `browser-storage-scope` | PASS |
| `offline-demo` | PASS |
| `route-metadata` | PASS for its declared Home/Demo/Privacy/Terms scope; F-4-3 records the omitted 404. |

The inventory test finds exactly one `@claim:<id>` tag for every declared claim. Landing and README product claims map to the inventory. F-4-4 is the remaining unlisted live statement.

## Structure, routing, accessibility, and identity

- Home, Demo, Privacy, and Terms have distinct titles, descriptions, canonicals, OG/Twitter metadata, one H1, one main landmark, favicons, and the common header/footer. F-4-3 records the 404 exception.
- Unknown paths return the designed concrete/moss 404 with a recovery action and HTTP 404.
- Every discovered internal page, image, icon, robots, sitemap, GitHub source link, and CLI-guide fragment resolves. The GitHub guide fragment exists.
- Hash navigation creates history, restores scroll, focuses the section heading, and announces it. F-4-1 records the separate full-page Back failure.
- Live responses include CSP as a response header, frame denial, HSTS, `nosniff`, referrer policy, and permissions policy. No CSP or console error appeared.
- Home has no document-width overflow at 390 px. F-4-2 records the inaccessible intentional overflow inside the terminal and result table.
- Light and dark 390 px axe runs otherwise report zero WCAG A/AA violations on Home, Demo, Privacy, Terms, and 404.
- The concrete/moss palette, specimen art, compressed type, hard rules, terminal, and report sheet follow `.factory/design.md`. The site is visually specific to this CLI and is not a generic SaaS template.
- The landing order misses the mandatory preview-before-explanation sequence as F-4-5.

## Other quality gates

From the clean clone, `npm test`, `npm run typecheck`, and `npm run build` pass. The build creates `dist/bin/vr` and `dist/site/`. Initial built JavaScript is about 3.3 KB gzip, far below the 150 KB site limit. The worker URL verifier reports the correct title, `lang=en`, one H1, one main landmark, complete alt text, labelled buttons, and zero console errors. The repository Playwright smoke passes, but its viewport coverage misses F-4-2 and its routing coverage misses F-4-1.

The standalone axe CLI could not start because its downloaded ChromeDriver targets Chrome 152 while the preinstalled Playwright Chromium is 145. The required equivalent Playwright axe integration ran directly instead and produced the serious mobile evidence in F-4-2.

## History verification

Every earlier `review-*.md`, `polish-*.md`, and handoff was read. Each earlier finding was checked against live behavior and current source.

| Earlier ID | Current verification |
| --- | --- |
| F-1-1 | Closed: both cold viewports state job, audience, first action, result, and facts. |
| F-1-2 | Closed: browser and CLI sample paths show/run the complete workflow. |
| F-1-3 | Closed: Demo records no real storage or external request access. |
| F-1-4 | Closed for the original absence: 20 claims and one tag each exist; F-4-4 is a new omission. |
| F-1-5 | Closed: the report preview matches the generated structured-change report. |
| F-1-6 | Closed: checkout, pricing, license, and unavailable paid copy remain absent. |
| F-1-7 | Closed: unknown paths return the designed 404 and recovery link. |
| F-1-8 | Closed: metadata workflow wording maps to the CLI demo claims. |
| F-1-9 | Closed: credential-free demo test passes. |
| F-1-10 | Closed: vague keep wording is absent; import/save behavior passes. |
| F-1-11 | Closed: each workflow verb has a passing claim. |
| F-1-12 | Closed: the unqualified slogan remains absent. |
| F-1-13 | Closed: public-host refusal tests pass. |
| F-1-14 | Closed: configured pre-write redaction tests pass. |
| F-1-15 | Closed: unsupported Pro/JUnit wording is absent. |
| F-1-16 | Closed: the universal `every result` claim is absent. |
| F-1-17 | Closed: the provider-sandbox availability claim is absent. |
| F-1-18 | Closed: import and capture use `webhook fixture` and pass. |
| F-1-19 | Closed: pre-write body/header redaction passes. |
| F-1-20 | Closed: all five comparison dimensions pass. |
| F-1-21 | Closed: changed diff exit 3 passes. |
| F-1-22 | Closed: saved method, headers, and body replay pass. |
| F-1-23 | Closed: public destinations are rejected in the boundary test. |
| F-1-24 | Closed: live and clean-sandbox Demo reload offline. |
| F-1-25 | Closed: request and storage instrumentation confirms the precise browser wording. |
| F-1-26 | Closed: `Zero uploads` is absent; precise sample-content wording passes. |
| F-1-27 | Closed: the broad `entirely` wording is absent; page-local behavior passes. |
| F-1-28 | Closed: generated Markdown contains structured contract changes. |
| F-1-29 | Closed: unsupported free-tier marketing is absent. |
| F-1-30 | Closed: purchase/pay-once wording is absent. |
| F-1-31 | Closed: price cards and merchant action are absent. |
| F-1-32 | Closed: README starts with a 17-word audience/job sentence. |
| F-1-33 | Closed: README workflow/report sentences are split and tested. |
| F-1-34 | Closed: the cleared-environment credential test passes. |
| F-1-35 | Closed: the no-telemetry source/network-path test passes. |
| F-1-36 | Closed: the unsupported Rust 1.82 statement is absent. |
| F-1-37 | Closed: default redaction fields are table-tested. |
| F-1-38 | Closed: request and plain-body fixture formats pass. |
| F-1-39 | Closed: exact saved-request replay passes. |
| F-1-40 | Closed: allowed/refused replay boundaries pass. |
| F-1-41 | Closed: one-shot capture stores a redacted fixture and exits. |
| F-1-42 | Closed: documented JSON behavior is limited to tested Demo output. |
| F-1-43 | Closed: exit codes 1, 3, and 4 pass. |
| F-1-44 | Closed: encrypted format/decryption pass. |
| F-1-45 | Closed: passphrase absence from the vault passes. |
| F-1-46 | Closed: wrong/missing passphrase rejection passes. |
| F-1-47 | Closed: wildcard body redaction passes. |
| F-1-48 | Closed: case-insensitive header redaction passes. |
| F-1-49 | Closed: plain and encrypted secret-absence checks pass. |
| F-1-50 | Closed: unsupported free-tier copy remains absent. |
| F-1-51 | Closed: unsupported Pro commands/copy remain absent. |
| F-1-52 | Closed: dead purchase/restore directions remain absent. |
| F-1-53 | Closed: unsupported license-cache behavior remains absent. |
| F-1-54 | Closed: test-suite marketing is absent; documented commands pass. |
| F-1-55 | Closed: build-output marketing is absent; the build itself passes. |
| F-1-56 | Closed for its copy issue: browser-test marketing is absent. |
| F-1-57 | Closed: network wording is limited to explicit loopback replay. |
| F-1-58 | Closed: replay, redaction, and encryption are separate tested claims. |
| F-1-59 | Closed: MIT license content passes. |
| F-1-60 | Closed: `no account system` remains absent; telemetry wording is inventoried. |
| F-1-61 | Closed: vault storage and redaction tests pass. |
| F-1-62 | Closed: encryption/passphrase tests pass. |
| F-1-63 | Closed: network policy describes explicit loopback replay only. |
| F-1-64 | Closed: normal storage contains only theme; Demo touches none. |
| F-1-65 | Closed: Demo origins and request bodies are instrumented. |
| F-1-66 | Closed: cookie/storage/request inventory passes. |
| F-1-67 | **Reopened as F-4-1:** cross-page Back returns focus to `BODY`; only hash Back passes. |
| F-1-68 | **Reopened as F-4-3:** 404 has no canonical and incomplete Twitter metadata. |
| F-1-69 | Closed: all routes retain the shared header/footer shell. |
| F-1-70 | Closed: external names identify GitHub and the guide fragment exists. |
| F-1-71 | Closed: sitemap lists indexable routes; unknown paths return 404. |
| F-1-72 | Closed: the literal local CLI/version label remains. |
| F-1-73 | Closed: image labels name a saved fixture and example data. |
| F-1-74 | Closed: the workflow heading is literal. |
| F-1-75 | Closed: step one says `Save and redact a fixture`. |
| F-1-76 | Closed: step two says `Compare version changes`. |
| F-1-77 | Closed: step three says `Replay it to localhost`. |
| F-1-78 | Closed: the browser sample is labelled JSON-only. |
| F-1-79 | Closed: the browser heading names sample webhook fixtures. |
| F-1-80 | Closed: empty state says `No comparison yet`. |
| F-1-81 | Closed: fixture/type/value terminology is consistent. |
| F-1-82 | Closed: report heading is `Export contract changes`. |
| F-1-83 | Closed: pricing slogan remains absent. |
| F-1-84 | Closed: generic optional-upgrade slogan remains absent. |
| F-1-85 | Closed: mood slogan remains absent. |
| F-1-86 | Closed: final action is `Install Version Replay`. |
| F-1-87 | Closed: footer states the operation and local location plainly. |
| F-1-88 | Closed: README audience sentence is 17 words. |
| F-1-89 | Closed: README feature copy is split and tested. |
| F-1-90 | Closed: user-facing stored input is `webhook fixture`. |
| F-1-91 | Closed: README says `export a Markdown report`. |
| F-1-92 | Closed: the long acceptance-test sentence remains absent. |
| F-1-93 | Closed: copy controls say `Copy install command`. |
| F-1-94 | Closed: navigation says Demo; primary action names sample data. |
| F-1-95 | Closed: comparison-bench metaphor remains absent. |
| F-1-96 | Closed: reset actions distinguish fixtures from the full Demo. |
| F-1-97 | Closed: install control copies and announces the command. |
| F-1-98 | Closed: landing, README, legal, error, and docs terminology is consistent. |
| F-2-1 | Closed: caller-selected vault test passes. |
| F-2-2 | Closed: plain Demo output paths exist and pass. |
| F-2-3 | Closed: README gives an install instruction, not an artifact promise. |
| F-2-4 | Closed for its timing scope: repeated hash Back assertions are stable; F-4-1 covers the untested cross-page case. |
| F-3-1 | Closed: CLI guide points to the live `#use-your-fixtures` target. |
| F-3-2 | Closed: `no account system` is absent. |
| F-3-3 | Closed: unsupported `short-lived` wording is absent. |

## Missed leverage

No AI step is appropriate for a deterministic, privacy-first contract comparison and replay tool. Adding model output would weaken reproducibility. The brief's expected adjacent capabilities—fixture import, loopback capture, diff, replay, Markdown/JSON export, and an isolated sample—are present. No missing import, export, sync, or Sociobot-gateway feature was found.

## What would make this perfect

1. Restore and announce H1 focus after full-page Back/Forward navigation, including page-cache restores.
2. Make the mobile terminal and comparison table keyboard-scrollable and add 390 px axe coverage in both themes.
3. Complete and test the designed 404's canonical and Twitter metadata.
4. Remove or inventory/test the Terms publication promise.
5. Put a working product preview before `How Version Replay works`.
6. Rerun every claim and the complete review from fresh contexts. PASS requires zero findings.
