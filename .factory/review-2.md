# Adversarial first-read review 2 — Version Replay

**Verdict: FAIL**

Reviewed 2026-08-29 UTC against the live site at <https://api-version-replay.sociobot.in/> and repository commit `51f6e33b676bbd5c77de6e7a80e3597dde9f22ac`. This was a new review, not a diff-only check.

## Cold first screen: 30-second result

Fresh Playwright contexts at 390 × 844 and 1440 × 900 loaded the site with no prior storage. Before scrolling, both viewports showed the same practical answer:

| Question | Answer from the first screen | Result |
| --- | --- | --- |
| What does it do? | Test old webhook versions against localhost. | Clear. |
| For whom? | Engineers checking an integration before upgrading a provider API. | Clear. |
| What should I click first? | `Try it with sample data`. | Clear. |

The exact visible copy is `Test old webhook versions against localhost`, `For engineers checking an integration before upgrading a provider API.`, and `Try it with sample data`. The initial mobile and desktop requests were same-origin only and produced no page or console errors.

## Findings, ordered by severity

### Blocking findings

#### F-1-2 — The one-click demo is still not a demo of the CLI job (re-opened)

- **Location/quote:** Home primary action: `Try it with sample data`; adjacent promise: `See saved versions, contract changes, and local replay results.` The first screen at `/?demo=1` is headed `Compare sample webhook versions` and shows only `Contract changed` with `3 type / 3 value` changes. The page itself says: `The browser sample compares JSON. Run the CLI demo for import and replay.`
- **Why this is blocking:** The researched product is a CLI that stores redacted version-labelled fixtures, replays them to localhost, and writes a report. The primary one-click action instead opens a browser-only JSON comparator. It does not show a stored fixture, redaction before storage, loopback replay, receiver result, or report. The adjacent promise specifically includes `local replay results`, which that first demo screen does not show. This is a half-fix of the earlier F-1-2, not evidence that a first-time visitor can try the real job in one click.
- **Evidence:** The isolated browser demo correctly shows its persistent `Demo — sample data, nothing is saved` banner, Reset and Start-for-real controls, realistic payment JSON, no storage reads/writes, no cookies, and same-origin requests only. Separately, the real CLI command works in a fresh temporary directory:

  ```text
  Version Replay sample
  Imported 2 redacted fixtures into /tmp/version-replay-demo-…/vault
  Compared 2024-04-10 → 2025-02-24: 5 contract changes
  Replayed 2024-04-10 → HTTP 204
  Replayed 2025-02-24 → HTTP 204
  Wrote Markdown report to /tmp/version-replay-demo-…/version-replay-report.md
  ```

  These are two separate experiences. A visitor following the advertised sample path does not see the latter.
- **Concrete fix:** Make the primary sample path open with the actual `vr demo` workflow already visible: a self-hosted terminal recording or a completed CLI-output panel that shows both imports/redaction, both loopback replays, and the written report path. Keep the browser comparator only as a clearly secondary `Compare sample JSON` tool. Change the action note to match the screen, or make the screen fulfil `local replay results`. Add a browser claim test that opens the primary sample URL and asserts the completed CLI workflow details and report path are present, plus the existing isolation checks.

### Minor findings

#### F-2-1 — One landing privacy/storage claim has no dedicated inventory entry

- **Location/quote:** Home product facts: `You choose the vault directory.`
- **Why this is a finding:** This is a visitor-relevant local-storage promise. No entry in `.factory/claims.json` names it, and the nearby `redaction-before-storage` test only proves redaction fields; it does not prove that a caller-selected vault directory is used. The claim inventory's `where: home facts` is not a substitute for testing this separate sentence.
- **Concrete fix:** Add a `vault-directory` claim with a clean temporary-directory test that runs `vr --vault <chosen-path> init/import`, asserts the expected files are created only under that path, and lists `home product facts, README` in `where`. Alternatively remove the sentence.

#### F-2-2 — The README promises normal demo output without a matching observable claim test

- **Location/quote:** README, Try the sample: `The command prints the temporary vault and report paths.`
- **Why this is a finding:** `cli-demo-workflow` runs `vr --json demo`; it asserts JSON fields and created files, but it does not exercise the normal command output that the README tells a visitor to use. This leaves the exact printed-path promise untested.
- **Concrete fix:** Add a `demo-output-paths` claim (or extend and rename the current claim so its scope is explicit) that invokes plain `vr demo`, parses both printed paths, and verifies they exist in a fresh temporary environment. List the README sentence in `where`.

#### F-2-3 — The README's build-artifact promise is unlisted

- **Location/quote:** README, Install: `Build the single binary:`
- **Why this is a finding:** This is a concrete build/distribution statement, but no claims entry or tagged test checks the promised single runnable CLI artifact from the documented installation route.
- **Concrete fix:** Either rewrite it as the instruction `Install the CLI from this checkout:` or add a `cli-install` claim that installs to an empty Cargo root and asserts `vr --help` works there. The test should be tagged and added to `.factory/claims.json`.

#### F-2-4 — The live browser sweep is timing-sensitive at Back navigation

- **Location/evidence:** The first run of `npm run test:browser -- https://api-version-replay.sociobot.in/` failed at `Back should restore the home position`. An immediate rerun passed. Five direct fresh-context repetitions showed the expected final state (`scrollY` 0–2 and focus on `#hero-title`) after a 100 ms wait.
- **Why this is a finding:** The live behaviour appears correct, but the verification gate can report a false failure because it asserts position as soon as the URL hash changes, before scroll restoration settles. A non-deterministic acceptance test cannot reliably prove the required back-button behaviour.
- **Concrete fix:** In `scripts/browser-smoke.mjs`, wait for `window.scrollY <= 5` and `document.activeElement.id === "hero-title"` before asserting, then run the routing check repeatedly in CI.

## Copy audit

Counts treat a hyphenated term and a code command as one word. Text in code examples is excluded unless it is a sentence shown to the reader. No audited landing or README sentence exceeds 22 words. The one misleading sample-action sentence is F-1-2 above; the inventory gaps are F-2-1 through F-2-3.

### Landing-page sentences

| # | Words | Sentence | Result |
| --- | ---: | --- | --- |
| L-01 | 18 | Test saved webhook versions against localhost with a local CLI that redacts, compares, replays, and reports contract changes. | Claim `cli-demo-workflow`. |
| L-02 | 6 | Test old webhook versions against localhost. | Clear H1. |
| L-03 | 10 | For engineers checking an integration before upgrading a provider API. | Clear audience. |
| L-04 | 9 | See saved versions, contract changes, and local replay results. | F-1-2. |
| L-05 | 7 | Runs locally · loopback replay only · MIT licensed. | Claims `loopback-only`, `mit-license`. |
| L-06 | 5 | You choose the vault directory. | F-2-1. |
| L-07 | 6 | Configured fields are removed before storage. | Claim `redaction-before-storage`. |
| L-08 | 4 | Public destinations are rejected. | Claim `loopback-only`. |
| L-09 | 9 | Use a saved webhook fixture for each provider version. | Claim `cli-demo-workflow`. |
| L-10 | 10 | Import JSON or capture one request on a loopback listener. | Claims `fixture-formats`, `capture-loopback`. |
| L-11 | 8 | List method, path, header, type, and value changes. | Claim `contract-dimensions`. |
| L-12 | 10 | Send either saved request to code running on your machine. | Claim `exact-replay`. |
| L-13 | 8 | Edit either sample webhook fixture, then compare them. | Instruction, not a product claim. |
| L-14 | 8 | Sample contents are not sent to a server. | Claim `browser-demo-isolation`. |
| L-15 | 5 | The browser sample compares JSON. | Accurate scope label. |
| L-16 | 8 | Run the CLI demo for import and replay. | Accurate instruction; exposes F-1-2. |
| L-17 | 3 | No comparison yet. | Clear empty state. |
| L-18 | 9 | Compare the fixtures to list type and value changes. | Clear empty-state instruction. |
| L-19 | 14 | `vr demo` creates a new temporary vault and leaves its report there for inspection. | Claim `cli-demo-workflow`. |
| L-20 | 7 | Recorded from the bundled payment webhook fixtures. | Claim `cli-demo-workflow`. |
| L-21 | 11 | The Markdown report names both versions and lists their structured changes. | Claim `report-formats`. |
| L-22 | 8 | It does not send replays to public hosts. | Claim `loopback-only`. |
| L-23 | 7 | It does not replace provider integration tests. | Useful limit, not a capability promise. |
| L-24 | 11 | The browser sample uses only the text shown on this page. | Claim `browser-demo-isolation`. |
| L-25 | 11 | Real webhook data belongs in the CLI, not this sample page. | Useful privacy guidance. |
| L-26 | 9 | Compare and replay saved webhook versions on your machine. | Claim `cli-demo-workflow`. |

Dynamic states also pass the 22-word cap: `Both webhook fixtures need JSON.` (5), `Fixtures match.` (2), `No type or value changes were found.` (8), `Comparison stopped.` (2), `Check commas, quotes, and closing braces.` (6), `Install command copied to the clipboard.` (6), `Demo — sample data, nothing is saved.` (6), and `Offline · sample still works.` (5).

### README sentences

| # | Words | Sentence | Result |
| --- | ---: | --- | --- |
| R-01 | 17 | Version Replay is a local CLI for engineers testing older webhook contracts before a provider API upgrade. | Claim `cli-demo-workflow`. |
| R-02 | 7 | It saves redacted fixtures by provider version. | Claim `cli-demo-workflow`. |
| R-03 | 12 | Compare changes, replay requests to localhost, and export Markdown or JSON reports. | Claims `exact-replay`, `report-formats`. |
| R-04 | 10 | The bundled demo creates a new temporary vault each time. | Claim `cli-demo-workflow`. |
| R-05 | 14 | It imports two payment webhook fixtures, compares them, replays both, and writes a report. | Claim `cli-demo-workflow`. |
| R-06 | 9 | The command prints the temporary vault and report paths. | F-2-2. |
| R-07 | 12 | The browser sample is available at `https://api-version-replay.sociobot.in/?demo=1`. | Verified live; route is covered by browser claims. |
| R-08 | 4 | Build the single binary. | F-2-3. |
| R-09 | 10 | Registry publishing and release archives are managed outside this repository. | Repository scope note. |
| R-10 | 9 | Create a vault, then import two saved webhook fixtures. | Clear instruction. |
| R-11 | 8 | Compare the versions and export a Markdown report. | Claim `report-formats`. |
| R-12 | 8 | Replay one saved request to a local service. | Claim `exact-replay`. |
| R-13 | 8 | Capture one incoming request on a loopback listener. | Claim `capture-loopback`. |
| R-14 | 7 | Place `--json` before `demo` to receive JSON. | Claim `cli-demo-workflow`. |
| R-15 | 11 | A changed diff exits `3`; a rejected replay response exits `4`. | Claim `exit-codes`. |
| R-16 | 9 | A fixture can contain `method`, `path`, `headers`, and `body`. | Claim `fixture-formats`. |
| R-17 | 11 | A plain JSON file becomes the body of a `POST /` fixture. | Claim `fixture-formats`. |
| R-18 | 12 | Default rules redact common identity, payment, cookie, and authorization fields before storage. | Claim `redaction-before-storage`. |
| R-19 | 6 | Add rules when creating a vault. | Clear instruction. |
| R-20 | 8 | Encrypted vaults use AES-256-GCM with an Argon2id-derived key. | Claim `encrypted-storage`. |
| R-21 | 13 | The passphrase stays in the environment and is not written to the vault. | Claim `encrypted-storage`. |
| R-22 | 5 | Replay accepts loopback destinations only. | Claim `loopback-only`. |
| R-23 | 12 | The CLI has no telemetry client and does not require provider credentials. | Claims `no-telemetry`, `no-provider-credentials`. |
| R-24 | 6 | Create the registry package without publishing. | Clear developer instruction. |
| R-25 | 7 | Use `dist/site` as the static deployment directory. | Clear deployment instruction. |

Headings name their sections (`Try the sample`, `Use your fixtures`, `Redaction and encryption`, and so on), rather than carrying a slogan. Actions name outcomes: `Try it with sample data`, `Compare sample fixtures`, `Reset sample fixtures`, `Reset demo`, `Start for real`, and `Copy install command`. The navigation label `Demo` is a location link, not an action button. Terminology is consistent: stored input is a `webhook fixture`, differences are `contract changes`, and output is a `report`.

## Demo, sandbox, and privacy checks

- The direct demo URL is `/?demo=1`; it loads completed realistic payment-fixture changes, includes the required persistent banner, and Reset restores the original fixtures.
- Instrumented `localStorage` and `sessionStorage` calls were empty throughout demo mode. The demo had no cookies and no cross-origin request or uploaded fixture body. Leaving demo returned to `/` and then read the normal `vr_theme` preference only.
- Browser demo reload after a first online visit passed offline through the existing claim test.
- The CLI demo was run from a new temporary working directory. It created a unique `/tmp/version-replay-demo-*` directory containing two bundled inputs, `vault/config.json`, redacted fixture files, and `version-replay-report.md`; it did not use a default vault.
- The browser isolation implementation is good, but F-1-2 remains because that browser sample is not the advertised CLI workflow.

## Claims and clean-clone results

I read `.factory/claims.json` and ran every listed command from a new clone at `/tmp/version-replay-review2.dTmGEs` after `npm ci`. All 17 declared commands passed:

`cli-demo-workflow`, `redaction-before-storage`, `loopback-only`, `encrypted-storage`, `capture-loopback`, `report-formats`, `contract-dimensions`, `exact-replay`, `fixture-formats`, `exit-codes`, `no-provider-credentials`, `no-telemetry`, `mit-license`, `browser-demo-isolation`, `browser-storage-scope`, `offline-demo`, and `route-metadata`.

The clean clone also completed `npm test`, `npm run typecheck`, `npm run build`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features -- -D warnings`; it produced both `dist/bin/vr` and `dist/site/`. F-2-1 through F-2-3 identify the remaining inventory/test coverage gaps. No provider key, Azure endpoint, or decorative AI feature was found.

## Structure, routing, accessibility, and identity

- Home, demo, Privacy, Terms, and the designed 404 all return the right page. A nonexistent route returns HTTP 404 with the 404 page; all discovered internal and GitHub links returned 200.
- The live response has CSP, `X-Frame-Options: DENY`, `nosniff`, referrer policy, and permissions policy. It has `lang=en`, one H1 and one main landmark on checked pages, canonical links, descriptions, OG/Twitter tags, an SVG favicon, apple icon, robots, and sitemap.
- The site’s hard-edged concrete/moss art, condensed type, status rails, and report treatment match `.factory/design.md`; it is recognizably product-specific rather than a generic SaaS template.
- The live browser sweep found no console errors, no mobile horizontal overflow at 390 px, visible keyboard focus, reduced-motion handling, and zero axe serious/critical findings. Back/focus behaviour succeeds after settling, but its test is timing-sensitive as recorded in F-2-4.

## History verification

Every earlier review, polish, verification, and handoff record was read. The practical fixes for F-1-1, F-1-3 through F-1-98 are present in the live site and code: first-screen clarity, isolation, claims inventory, removed checkout, real 404, metadata, common shell, route focus, responsive layout, and terminology all verify. F-1-2 is re-opened above because the primary sample route still demonstrates only JSON comparison rather than the real CLI job.

## Missed leverage

No AI feature is implied by this deterministic, privacy-first local CLI; adding one would not improve the stated job. Capture, import, replay, and report export already exist. The missing leverage is instead the fully representative one-click CLI sample described in F-1-2.

## What would make this perfect

1. Make `Try it with sample data` visibly run/show the complete isolated CLI demo, not only a browser JSON diff.
2. Add the three missing claim tests or remove the three unsupported promises.
3. Make the Back-navigation browser assertion wait for the settled scroll/focus state, then repeat it in CI.
4. Rerun this entire review from clean contexts. Only a zero-finding result can change the verdict to PASS.
