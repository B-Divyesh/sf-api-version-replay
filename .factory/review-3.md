# Adversarial first-read review 3 — Version Replay

**Verdict: FAIL**

Reviewed 2026-08-29 UTC against live <https://api-version-replay.sociobot.in/> and repository commit `03783af146be2a87c7edccd6883b70344727060b`. The live home HTML, app JavaScript, and CSS are byte-for-byte identical to the clean-clone build. This was a full review from fresh contexts, not a diff-only check.

The first screen and demo now pass. All 20 declared claim tests pass. The review still has one blocking reopened finding and two minor findings, so the required zero-finding standard is not met.

## Cold first screen: 30-second result

Fresh Playwright contexts at 390 × 844 and 1440 × 900 opened `/` with empty browser storage. No scrolling occurred before this assessment.

| Question | Answer visible on phone and desktop | Result |
| --- | --- | --- |
| What does it do? | `Test old webhook versions against localhost` | Clear. |
| For whom? | `For engineers checking an integration before upgrading a provider API.` | Clear. |
| What should I click first? | `Try it with sample data` | Clear. |

The adjacent result text is also visible: `See saved versions, contract changes, and local replay results.` The three first-screen facts are `Runs locally`, `loopback replay only`, and `MIT licensed`. Both cold loads used same-origin requests only, had no console or page errors, and had no horizontal overflow.

## Findings, ordered by severity

### Blocking

#### F-3-2 / F-1-60 (reopened) — The no-account claim remains unlisted and untested

- **Exact quote/location:** Privacy → Network access: `The project has no account system or telemetry client.`
- **Why this fails:** `.factory/claims.json` lists and tests `The CLI has no telemetry client; its only network request is the explicit loopback replay.` It does not list the separate `no account system` promise. The `claim_no_telemetry` test checks telemetry dependency names and `ureq` call counts; it never checks for account/authentication state or code. Earlier F-1-60 included the no-account claim. Leaving that clause in the live policy while marking F-1-60 fixed was a partial fix, so the history rule makes this blocking again.
- **Concrete fix:** Either delete `no account system` and use `The project has no telemetry client.`, or add a `no-account-system` claim whose clean-sandbox test inventories account/authentication dependencies, configuration, persistence, and external requests. List `Privacy` in `where`.

### Minor

#### F-3-1 — The CLI-guide link has a dead fragment

- **Exact quote/location:** Home → Export contract changes: `Read the CLI guide on GitHub (opens in a new site)` links to `https://github.com/B-Divyesh/sf-api-version-replay#usage`.
- **Why this fails:** GitHub returns HTTP 200, but the page contains no `#usage` target. The README heading is `Use your fixtures`, whose rendered fragment is `#use-your-fixtures`. The link drops a visitor at the repository top instead of the promised guide. A status-only crawler misses this failure.
- **Concrete fix:** Change the href to `https://github.com/B-Divyesh/sf-api-version-replay#use-your-fixtures`. Add a link test that loads external fragment links and confirms the target ID exists after navigation.

#### F-3-3 — The hosting-log retention wording is vague and unlisted

- **Exact quote/location:** Privacy → Website records: `Hosting infrastructure may keep short-lived request and security logs.`
- **Why this fails:** `short-lived` implies a retention assurance but gives no duration. No claims entry or test covers hosting logs or retention, and the browser request test cannot observe server-side retention.
- **Concrete fix:** Remove the unsupported duration adjective. Use `The hosting provider may process request and security logs under its own retention policy.` If the product keeps a duration claim, state the number and add verifiable evidence for it.

## Copy audit

Counting treats a hyphenated term, URL, date, or code command as one word. Separators such as `·` are not words. No landing-page or README sentence exceeds 22 words. No supplied banned marketing adjective appears. Domain terms such as CLI, JSON, localhost, loopback, AES-256-GCM, and Argon2id are appropriate for the named engineering audience and are used consistently.

### Landing-page sentences

This includes metadata copy because search and share previews are part of the landing page.

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| L-01 | 18 | Test saved webhook versions against localhost with a local CLI that redacts, compares, replays, and reports contract changes. | Listed claims. |
| L-02 | 9 | Save, compare, and replay old webhook versions against localhost. | Listed claims. |
| L-03 | 6 | Test old webhook versions against localhost. | Clear headline. |
| L-04 | 10 | For engineers checking an integration before upgrading a provider API. | Clear audience. |
| L-05 | 9 | See saved versions, contract changes, and local replay results. | `primary-demo-workflow`. |
| L-06 | 7 | Runs locally · loopback replay only · MIT licensed. | Listed claims. |
| L-07 | 5 | You choose the vault directory. | `vault-directory`. |
| L-08 | 6 | Configured fields are removed before storage. | `redaction-before-storage`. |
| L-09 | 4 | Public destinations are rejected. | `loopback-only`. |
| L-10 | 9 | Use a saved webhook fixture for each provider version. | `cli-demo-workflow`. |
| L-11 | 10 | Import JSON or capture one request on a loopback listener. | `fixture-formats`, `capture-loopback`. |
| L-12 | 8 | List method, path, header, type, and value changes. | `contract-dimensions`. |
| L-13 | 10 | Send either saved request to code running on your machine. | `exact-replay`. |
| L-14 | 8 | Edit either sample webhook fixture, then compare them. | Clear instruction. |
| L-15 | 8 | Sample contents are not sent to a server. | `browser-demo-isolation`. |
| L-16 | 5 | The browser sample compares JSON. | Accurate scope. |
| L-17 | 8 | Run the CLI demo for import and replay. | Clear instruction. |
| L-18 | 3 | No comparison yet. | Clear empty state. |
| L-19 | 9 | Compare the fixtures to list type and value changes. | Clear next action. |
| L-20 | 14 | `vr demo` creates a new temporary vault and leaves its report there for inspection. | `cli-demo-workflow`. |
| L-21 | 7 | Recorded from the bundled payment webhook fixtures. | `cli-demo-workflow`. |
| L-22 | 11 | The Markdown report names both versions and lists their structured changes. | `report-formats`. |
| L-23 | 8 | It does not send replays to public hosts. | `loopback-only`. |
| L-24 | 7 | It does not replace provider integration tests. | Useful limit. |
| L-25 | 11 | The browser sample uses only the text shown on this page. | `browser-demo-isolation`. |
| L-26 | 11 | Real webhook data belongs in the CLI, not this sample page. | Useful privacy guidance. |
| L-27 | 9 | Compare and replay saved webhook versions on your machine. | Listed claims. |

### Dynamic landing-page sentences

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| D-01 | 5 | Both webhook fixtures need JSON. | Clear error cause. |
| D-02 | 2 | Fixtures match. | Clear result. |
| D-03 | 7 | No type or value changes were found. | Clear result. |
| D-04 | 6 | The fixtures could not be compared. | Clear fallback. |
| D-05 | 2 | Comparison stopped. | Clear error heading. |
| D-06 | 6 | Check commas, quotes, and closing braces. | Actionable recovery. |
| D-07 | 6 | Install command copied to the clipboard. | Clear confirmation. |
| D-08 | 4 | Copy this command: `[install command]`. | Clear fallback. |
| D-09 | 6 | Demo — sample data, nothing is saved. | Demo state; see claims results. |
| D-10 | 4 | Offline · sample still works. | `offline-demo`. |
| D-11 | 5 | Available after this page loads. | `offline-demo`. |

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
| R-09 | 10 | Registry publishing and release archives are managed outside this repository. | Clear scope. |
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
| R-25 | 7 | Use `dist/site` as the static deployment directory. | Clear instruction. |
| R-26 | 6 | The factory manages infrastructure and DNS. | Clear repository boundary. |
| R-27 | 6 | See the privacy policy and terms. | Clear links. |

### Headings, actions, and terminology

The headings name their sections: `Save, compare, then replay`, `Compare two sample webhook fixtures`, `Replay the complete CLI sample`, `Export contract changes`, `What it does not do`, and `Install Version Replay`. Step headings also name actions. None is metaphorical or reusable marketing copy.

Buttons and action links name results: `Try it with sample data`, `Copy install command`, `Compare sample fixtures`, `Reset sample fixtures`, `Reset demo`, `Start for real`, and `Return home`. The GitHub guide label is clear, but its target fails as F-3-1.

Terminology is consistent: stored input is a `webhook fixture`; the identifier is a `version`; differences are `contract changes`; output is a `report`; CLI storage is a `vault`; and replay destinations are `loopback` or `localhost`.

## Demo and sandbox behavior

- Activating `Try it with sample data` once opens `/?demo=1`.
- At 390 px and desktop, the first demo screen already shows `Replay the complete CLI sample`, two redacted imports, five contract changes, two HTTP 204 loopback replays, and the Markdown report path.
- The persistent banner says `Demo — sample data, nothing is saved` and exposes `Reset demo` and `Start for real`.
- Editing the pinned sample to `{}` and activating Reset restores the payment webhook and recomputes the result.
- A context preloaded with `real:user-data` and `vr_theme` kept both values unchanged. The instrumented isolation claim observed zero demo storage reads/writes, no cookies, no cross-origin requests, and no request body containing fixture text.
- After one online visit, the demo reloaded and rendered its result offline.
- An independent CLI run from a fresh `/tmp/version-replay-review3-demo.*` working directory created `/tmp/version-replay-demo-8833-1787966330108/`, a redacted vault, and `version-replay-report.md`. It created no default `.version-replay` vault in the caller directory.

## Claims results

Every command in `.factory/claims.json` was run separately after `npm ci` in clean clone `/tmp/api-version-replay-review3.GF2NWM`.

| Claim ID | Command | Result |
| --- | --- | --- |
| `cli-demo-workflow` | `npm run test:claims -- cli-demo-workflow` | PASS |
| `primary-demo-workflow` | `npm run test:claims -- primary-demo-workflow` | PASS |
| `demo-output-paths` | `npm run test:claims -- demo-output-paths` | PASS |
| `vault-directory` | `npm run test:claims -- vault-directory` | PASS |
| `redaction-before-storage` | `npm run test:claims -- redaction-before-storage` | PASS |
| `loopback-only` | `npm run test:claims -- loopback-only` | PASS |
| `encrypted-storage` | `npm run test:claims -- encrypted-storage` | PASS |
| `capture-loopback` | `npm run test:claims -- capture-loopback` | PASS |
| `report-formats` | `npm run test:claims -- report-formats` | PASS |
| `contract-dimensions` | `npm run test:claims -- contract-dimensions` | PASS |
| `exact-replay` | `npm run test:claims -- exact-replay` | PASS |
| `fixture-formats` | `npm run test:claims -- fixture-formats` | PASS |
| `exit-codes` | `npm run test:claims -- exit-codes` | PASS |
| `no-provider-credentials` | `npm run test:claims -- no-provider-credentials` | PASS |
| `no-telemetry` | `npm run test:claims -- no-telemetry` | PASS |
| `mit-license` | `npm run test:claims -- mit-license` | PASS |
| `browser-demo-isolation` | `npm run test:claims -- browser-demo-isolation` | PASS |
| `browser-storage-scope` | `npm run test:claims -- browser-storage-scope` | PASS |
| `offline-demo` | `npm run test:claims -- offline-demo` | PASS |
| `route-metadata` | `npm run test:claims -- route-metadata` | PASS |

The inventory test confirms one `@claim:<id>` tag per declared claim. F-3-2 and F-3-3 are live claim-like statements outside that inventory; therefore the declared suite passing does not produce a zero-claim-gap result.

## Structure, routing, accessibility, and identity

- Home, demo, Privacy, and Terms expose distinct titles, descriptions, canonicals, one H1, one main landmark, OG data, Twitter cards, SVG/apple icons, and the shared header/footer. The designed noindex 404 has the same shell and recovery link.
- A random unknown route returns HTTP 404 and the designed `This page was not found` page. Home, demo, Privacy, Terms, robots, sitemap, icons, images, scripts, and styles return successfully.
- The link crawl found no HTTP error. Fragment validation found F-3-1.
- The three-repeat section navigation test passes: a hash navigation creates history, Back restores the top, focus returns to `#hero-title`, and the live region announces headings. Legal routes focus their H1.
- `npm run test:browser -- https://api-version-replay.sociobot.in/` passes desktop keyboard use, 390 px layout, offline reload, reduced motion, route behavior, console checks, and axe with zero violations.
- `/opt/fleet/lib/verify-url.sh` reports title, `lang=en`, one H1, a main landmark, complete alt text, labelled buttons, and zero console errors.
- Live responses include CSP, frame denial, `nosniff`, referrer policy, permissions policy, and HSTS. Requests observed during normal and demo flows were same-origin only.
- The concrete/moss palette, hard rules, compressed type, specimen image, terminal treatment, and report sheet follow `.factory/design.md`. The page is visually specific to a versioned local CLI and is not a generic SaaS template.

## Other quality gates

The clean clone passes `npm test`, `npm run typecheck`, `npm run build`, `cargo fmt --check`, and strict Clippy. `npm run build` produces `dist/bin/vr` and `dist/site/`; initial built JavaScript is 2.79 KB gzip plus a 0.44 KB style loader. Live/build SHA-256 hashes match for the home HTML, app JavaScript, and CSS.

## History verification

Every earlier `review-*`, `polish-*`, both verification reports, and the prior handoff were read. The live site and current code were checked, not the status labels alone.

| Earlier finding(s) | Current verification |
| --- | --- |
| F-1-1 | Closed: first screen states job, audience, first action, and facts at both sizes. |
| F-1-2 | Closed: the first demo screen now shows the complete CLI sample; CLI `vr demo` also works in a fresh temporary directory. |
| F-1-3 | Closed: demo mode bypasses real storage and external requests. |
| F-1-4 | Closed for declared claims: inventory exists, tag cardinality passes, and all commands pass. New omissions are F-3-2/F-1-60 and F-3-3. |
| F-1-5 | Closed: report copy and preview contain structured changes, not a fabricated replay result. |
| F-1-6 | Closed: unavailable checkout and paid-feature copy remain absent. |
| F-1-7 | Closed: unknown routes return the designed 404 with recovery. |
| F-1-8–F-1-59 | Closed: retained landing/README capabilities are listed and pass their clean-sandbox tests. |
| F-1-60 | **Reopened as F-3-2:** `no account system` remains live but absent from the exact claim inventory and test. |
| F-1-61–F-1-66 | Closed for the original statements: vault, redaction, encryption, network, browser storage, cookies, and analytics behavior pass. F-3-3 is a new retention statement. |
| F-1-67 | Closed: repeated Back, scroll, focus, and live-region checks pass. |
| F-1-68 | Closed: indexed routes have complete route metadata and product art; the 404 is intentionally noindex. |
| F-1-69 | Closed: routes share the header/footer shell, legal links, factory credit, and version. |
| F-1-70 | Closed: external-link names identify GitHub. The separate dead fragment is F-3-1. |
| F-1-71 | Closed: sitemap contains every indexable public route; unknown routes return 404. |
| F-1-72–F-1-98 | Closed: current headings, sentences, states, actions, and terminology pass the plain-words audit. |
| F-2-1 | Closed: `vault-directory` is listed and passes. |
| F-2-2 | Closed: plain demo output paths are listed and pass. |
| F-2-3 | Closed: README now gives an installation instruction, not an unsupported artifact promise. |
| F-2-4 | Closed: the settled Back/focus assertion passes three repetitions locally and live. |

## Missed leverage

No AI step is implied by this deterministic, privacy-first CLI. Adding model output would weaken reproducibility. The brief's obvious adjacent capabilities—fixture import, loopback capture, diff, replay, Markdown/JSON export, and isolated sample data—are present. No missing import, export, sync, or Sociobot-gateway feature was found.

## What would make this perfect

1. Close F-3-2/F-1-60 by removing or explicitly listing and testing the no-account statement.
2. Point the CLI-guide link at `#use-your-fixtures` and make the crawler validate fragments.
3. Remove the unverified `short-lived` log-retention assurance or replace it with a precise, evidenced policy.
4. Rerun the entire review from clean contexts. PASS requires zero findings and no unlisted claim.
