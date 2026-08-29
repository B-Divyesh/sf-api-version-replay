# Adversarial first-read review 5 — Version Replay

**Verdict: PASS**

Reviewed 2026-08-29 UTC against live `https://api-version-replay.sociobot.in/` and a clean clone of commit `c0eb688492b43addc5f9d7d5c528d7cf5f2c7f8f`. This review re-ran the complete checklist, all declared claim commands separately, and the earlier-finding verification. There are zero blocking, major, minor, or untested-claim findings.

## Cold first screen: 30-second result

Fresh Playwright contexts at 390 × 844 and 1440 × 900 loaded the page without console errors. No scroll occurred before this assessment.

| Question | Answer visible on the first screen | Result |
| --- | --- | --- |
| What does this do? | “Test old webhook versions against localhost.” | Clear: a local CLI compares older webhook versions and replays them locally. |
| For whom? | “For engineers checking an integration before upgrading a provider API.” | Clear and matches the brief. |
| What should I click first? | “Try it with sample data” beside “See saved versions, contract changes, and local replay results.” | Clear single primary action and stated outcome. |

At 390 px, the action, outcome, and three facts remain in the viewport. The first action is not obscured by the command-copy control. The desktop screenshot retains the same hierarchy.

## Demo and sandbox verification

The first action opens `/?demo=1` in one navigation. Its first view has the H1 “Replay the complete CLI sample” and immediately shows a completed realistic `vr demo` recording: two redacted imports, five contract changes, two loopback HTTP 204 replays, and a Markdown report path. The browser comparator below it is populated and has six visible change rows.

The persistent banner reads “Demo — sample data, nothing is saved” and includes `Reset demo` and `Start for real`. After changing a sample fixture, `Reset demo` restored the bundled fixture and recomputed the six rows. `Start for real` returned to `/`; only then did the normal page read its optional `vr_theme` preference.

In a new 390 px context, instrumented `localStorage` methods recorded zero demo reads and zero demo writes. IndexedDB/cookies were empty. The complete demo request log contained only same-origin document, script, stylesheet, and image requests; no request body held fixture text and no cross-origin request occurred. The declared offline claim also passed from a fresh clean-clone test.

For the CLI path, `vr demo` is covered by the isolated temporary-vault claim test. It imports bundled fixtures, redacts them, compares them, replays them to a loopback receiver, writes a report, and prints existing vault/report paths. It does not use the default vault.

## Claims: clean-clone results

Clean clone: `/tmp/api-version-replay-review5.FdatZe`. After `npm ci`, every entry in `.factory/claims.json` was run as its own command and passed:

| Claim id | Result |
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
| `route-metadata` | PASS |

The inventory/tag cardinality test passed. Each retained capability, privacy statement, quantitative result, and route-metadata statement has a listed clean-sandbox test. Cross-checking the live landing page and README found each claim-like statement covered by the applicable inventory entry; no unlisted claim remains.

Additional clean-clone checks passed:

```sh
npm test
npm run typecheck
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --locked
npm run test:browser -- https://api-version-replay.sociobot.in/
```

`npm run build` produced `dist/site` and `dist/bin/vr`. The site build reported 7.33 kB main JavaScript (2.90 kB gzip) and 18.35 kB CSS (4.53 kB gzip).

## Copy audit

Counting uses alphanumeric tokens with internal hyphens as one word. Code blocks, terminal-recording output, and compact labels are audited separately below. No prose sentence exceeds 22 words. No banned marketing adjective, unexplained metaphor, inconsistent product term, or non-result-naming action was found.

### Landing-page prose and metadata

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| L-01 | 18 | Test saved webhook versions against localhost with a local CLI that redacts, compares, replays, and reports contract changes. | Pass |
| L-02 | 9 | Save, compare, and replay old webhook versions against localhost. | Pass |
| L-03 | 6 | Test old webhook versions against localhost. | Pass |
| L-04 | 10 | For engineers checking an integration before upgrading a provider API. | Pass |
| L-05 | 9 | See saved versions, contract changes, and local replay results. | Pass |
| L-06 | 2 | Runs locally. | Pass; supported by `cli-demo-workflow`. |
| L-07 | 3 | Loopback replay only. | Pass; supported by `loopback-only`. |
| L-08 | 2 | MIT licensed. | Pass; supported by `mit-license`. |
| L-09 | 5 | You choose the vault directory. | Pass |
| L-10 | 6 | Configured fields are removed before storage. | Pass |
| L-11 | 4 | Public destinations are rejected. | Pass |
| L-12 | 14 | `vr demo` creates a new temporary vault and leaves its report there for inspection. | Pass |
| L-13 | 7 | Recorded from the bundled payment webhook fixtures. | Pass |
| L-14 | 9 | Use a saved webhook fixture for each provider version. | Pass |
| L-15 | 10 | Import JSON or capture one request on a loopback listener. | Pass |
| L-16 | 8 | List method, path, header, type, and value changes. | Pass |
| L-17 | 10 | Send either saved request to code running on your machine. | Pass |
| L-18 | 8 | Edit either sample webhook fixture, then compare them. | Pass |
| L-19 | 8 | Sample contents are not sent to a server. | Pass |
| L-20 | 5 | The browser sample compares JSON. | Pass |
| L-21 | 8 | Run the CLI demo for import and replay. | Pass |
| L-22 | 3 | No comparison yet. | Pass: useful empty state. |
| L-23 | 9 | Compare the fixtures to list type and value changes. | Pass: tells the next action. |
| L-24 | 11 | The Markdown report names both versions and lists their structured changes. | Pass |
| L-25 | 8 | It does not send replays to public hosts. | Pass |
| L-26 | 7 | It does not replace provider integration tests. | Pass: clear limit. |
| L-27 | 11 | The browser sample uses only the text shown on this page. | Pass |
| L-28 | 11 | Real webhook data belongs in the CLI, not this sample page. | Pass: clear limit. |
| L-29 | 9 | Compare and replay saved webhook versions on your machine. | Pass |

The completed terminal recording is evidence, not slogan copy. Its lines are compact observable outputs; the longest is 12 words and its five-change count and paths are tested. The dynamic error/status sentences are likewise short and actionable: “Both webhook fixtures need JSON.”, “Fixtures match.”, “No type or value changes were found.”, “The fixtures could not be compared.”, “Comparison stopped.”, “Check commas, quotes, and closing braces.”, “Install command copied to the clipboard.”, and “Copy this command: …”.

All headings name their sections, including “Replay the complete CLI sample”, “Save, compare, then replay”, “Compare two sample webhook fixtures”, “Export contract changes”, and “What it does not do”. The primary and secondary controls name outcomes: `Try it with sample data`, `Copy install command`, `Compare sample fixtures`, `Reset sample fixtures`, `Reset demo`, `Start for real`, and `Return home`. The theme control’s accessible label is the imperative `Use dark theme` / `Use light theme`.

### README prose

| ID | Words | Sentence | Result |
| --- | ---: | --- | --- |
| R-01 | 17 | Version Replay is a local CLI for engineers testing older webhook contracts before a provider API upgrade. | Pass |
| R-02 | 7 | It saves redacted fixtures by provider version. | Pass |
| R-03 | 12 | Compare changes, replay requests to localhost, and export Markdown or JSON reports. | Pass |
| R-04 | 10 | The bundled demo creates a new temporary vault each time. | Pass |
| R-05 | 14 | It imports two payment webhook fixtures, compares them, replays both, and writes a report. | Pass |
| R-06 | 9 | The command prints the temporary vault and report paths. | Pass |
| R-07 | 12 | The browser sample is available at `https://api-version-replay.sociobot.in/?demo=1`. | Pass |
| R-08 | 6 | Install the CLI from this checkout. | Pass: instruction. |
| R-09 | 10 | Registry publishing and release archives are managed outside this repository. | Pass: scope note. |
| R-10 | 9 | Create a vault, then import two saved webhook fixtures. | Pass: instruction. |
| R-11 | 8 | Compare the versions and export a Markdown report. | Pass: instruction. |
| R-12 | 8 | Replay one saved request to a local service. | Pass: instruction. |
| R-13 | 8 | Capture one incoming request on a loopback listener. | Pass: instruction. |
| R-14 | 7 | Place `--json` before `demo` to receive JSON. | Pass |
| R-15 | 11 | A changed diff exits 3; a rejected replay response exits 4. | Pass |
| R-16 | 9 | A fixture can contain `method`, `path`, `headers`, and `body`. | Pass |
| R-17 | 11 | A plain JSON file becomes the body of a `POST /` fixture. | Pass |
| R-18 | 12 | Default rules redact common identity, payment, cookie, and authorization fields before storage. | Pass |
| R-19 | 6 | Add rules when creating a vault. | Pass: instruction. |
| R-20 | 8 | Encrypted vaults use AES-256-GCM with an Argon2id-derived key. | Pass |
| R-21 | 13 | The passphrase stays in the environment and is not written to the vault. | Pass |
| R-22 | 5 | Replay accepts loopback destinations only. | Pass |
| R-23 | 12 | The CLI has no telemetry client and does not require provider credentials. | Pass |
| R-24 | 6 | Create the registry package without publishing. | Pass: instruction. |
| R-25 | 8 | Use `dist/site` as the static deployment directory. | Pass: instruction. |
| R-26 | 6 | The factory manages infrastructure and DNS. | Pass: scope note. |
| R-27 | 6 | See the privacy policy and terms. | Pass: navigation instruction. |

README headings are literal and independently meaningful: “Try the sample”, “Use your fixtures”, “Fixture format”, “Redaction and encryption”, “Develop and verify”, and “Deploy the documentation site”. Terminology remains consistent: webhook fixture, version, contract changes, report, browser sample, demo, vault, and loopback/localhost.

## Structure, accessibility, routes, and identity

- Home, Demo, Privacy, Terms, and the designed 404 have one H1, a `<main>`, language metadata, distinct plain-language titles, descriptions, canonicals, social-image metadata, favicon, Apple touch icon, and matching theme color. The unknown live route returned HTTP 404 with the designed recovery page.
- The response policy supplies a same-origin CSP, `frame-ancestors 'none'`, `nosniff`, strict referrer policy, HSTS, and permissions policy. The live page had no console errors.
- `robots.txt` and `sitemap.xml` returned 200. The sitemap lists home, demo, privacy, and terms; the 404 is correctly noindex.
- Crawl results: all discovered same-origin links returned 200; the external GitHub source and guide fragment returned 200. There are no dead links.
- The live browser suite passed desktop keyboard behavior, 390 px Home/Demo in both themes, mobile keyboard-scrollable terminal/table regions, offline reload, route focus and announcements, hash history, and full-document Back navigation. Axe reported zero serious or critical violations.
- The shared header has a skip link, wordmark-to-home link, Demo, How it works, and Privacy. The shared footer includes its local-product description, Privacy, Terms, Param Factory attribution, and version.
- The concrete/moss system is visibly product-specific: material specimen art, hard rules/shadows, compressed terminal-like typography, moss compatibility accent, and a matching broken-stack 404. It is not a generic SaaS card/gradient surface.

## Earlier finding verification

Every historical item was checked against the current live product and source, not accepted merely because a prior document marked it closed.

| Earlier IDs | Current verification | Result |
| --- | --- | --- |
| F-1-1, F-1-72–F-1-98 | Cold first-read, complete copy audit, terminology, labelled assets, literal headings, named actions, and footer were rechecked live. | Fixed |
| F-1-2, F-1-3, F-1-5 | One-click browser demo, isolated storage/request behavior, CLI `vr demo`, and report truthfulness were re-run. | Fixed |
| F-1-4, F-1-8–F-1-66 | Inventory cardinality and all twenty clean-sandbox claim commands passed; live/README claims were cross-checked. | Fixed |
| F-1-6, F-1-69–F-1-71 | No unavailable paid path remains; crawl, shared shell, 404, sitemap, and external-link names pass. | Fixed |
| F-1-7, F-1-67, F-1-68 | Live unknown route is a complete HTTP 404; route metadata, Back/focus/announcement, and social metadata pass. | Fixed |
| F-2-1, F-2-2, F-2-3, F-2-4 | Caller-selected vault, printable demo paths, checkout-free installation instruction, and repeated routing test pass. | Fixed |
| F-3-1, F-3-2, F-3-3 | The GitHub guide fragment resolves; the unlisted account-system and retention promises remain absent. | Fixed |
| F-4-1, F-4-2, F-4-3, F-4-4, F-4-5 | Cross-document Back focus, mobile overflow focusability, 404 metadata, Terms wording, and product-result-before-workflow order all pass live. | Fixed |

## Missed leverage

No missing obvious capability was found. The brief’s core loop—store redacted fixtures, compare versions, replay to localhost, and report changes—has import, capture, replay, Markdown/JSON export, encrypted storage, and a demonstrable sample. An AI step would not improve this deterministic local CLI workflow and would add privacy/cost complexity without being implied by the brief. No decorative or credential-embedding AI feature exists.

## What would make this perfect

The current product meets the requested standard. Maintain the same claim discipline when adding formats, providers, or automation: first add a fresh-sandbox claim test, keep the demo isolated, and preserve the direct first-screen explanation.
