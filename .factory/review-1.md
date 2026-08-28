# Adversarial first-read review 1 — Version Replay

**Verdict: FAIL**

Reviewed 2026-08-28 UTC against live `https://api-version-replay.sociobot.in/` and repository commit `d29c2bf91857b8cbbf1e293140442fa00a963e31` from an isolated detached worktree. The product is visually distinct and its existing unit, browser, accessibility, offline, and build checks pass. It still fails the required first-read, demo, claims, honesty, purchase, and routing checks.

## Cold first screen: 30-second result

No scrolling occurred before this assessment. Fresh Playwright contexts were used at 390 × 844 and 1440 × 900. Screenshots were captured at `/tmp/version-replay-mobile.png` and `/tmp/version-replay-desktop.png` during this review.

| Question | 390 px phone | Desktop | Result |
| --- | --- | --- | --- |
| What does this do? | “Re-run the wire. Before it snaps.” does not name a CLI, stored webhook fixtures, API-version comparison, or local replay. The paragraph eventually suggests webhook comparison and localhost replay. | Same ambiguity. | **BLOCKING** |
| For whom? | No audience is named. “your integration” is not the brief's engineer maintaining an older third-party API version. | Same. | **BLOCKING** |
| What should I click first? | The first control is an install-command copy button; “Open the comparison bench” is a second competing action. | The header also adds “Try it.” | **BLOCKING** |

The exact failing first-screen text is:

> “Re-run the wire. Before it snaps.”
>
> “Keep the exact webhook your integration understands. Redact it, label the provider version, compare the next contract, and replay both against localhost.”
>
> “Copy” / “Open the comparison bench” / “Try it”

## Findings, ordered by severity

### Blocking findings

#### F-1-1 — The first screen does not state the job, audience, and first action

- **Location/quote:** Home hero, quoted above.
- **Why this fails:** The headline is a wire/snapping metaphor. The supporting copy never says this is a CLI for engineers maintaining an integration pinned to an older third-party API version. Two actions compete and neither says “Try it with sample data.”
- **Concrete fix:** Use `Test old webhook versions against localhost` as the H1. Follow with `For engineers checking an integration before upgrading a provider API.` Use one primary action, `Try it with sample data`, with adjacent text: `See the saved versions, contract changes, and local replay result.` Put installation second.

#### F-1-2 — There is no compliant one-click demo of the CLI

- **Location/quote:** Home links “Try it” and “Open the comparison bench”; repository CLI help and README.
- **Evidence:** The links scroll to a browser-only JSON comparator. The first screen after scrolling shows two populated textareas but only says “Bench is ready”; the result requires another click. It does not run the CLI's capture/import/diff/replay/report job. `vr demo` run from an empty temporary directory exits 2 with `unrecognized subcommand 'demo'`. There is no self-hosted terminal recording, `.factory/demo.md`, persistent demo banner, `Reset demo`, or `Start for real`.
- **Concrete fix:** Ship `vr demo` with bundled realistic fixtures. It must create an isolated temporary vault, run import/diff/replay/report without setup, and print the output path. Add a first-screen `Try it with sample data` action and a self-hosted recording of that exact command. The browser route `/demo` must open directly on a completed realistic result with the required banner and reset/exit actions.

#### F-1-3 — The nominal demo route reads and writes real browser storage

- **Location:** Live `/demo` and `site/src/main.ts`.
- **Evidence:** `/demo` is only the home fallback. A fresh visit writes `vr_theme`, not a `demo:` key. With an existing `sb_license:api-version-replay` value, `/demo` reads it, calls `https://api.sociobot.in/.../verify?license=...`, and writes `sb_license_verdict:api-version-replay`. The banner is absent.
- **Why this fails:** Demo mode is not a separate storage namespace and can touch a visitor's real license state.
- **Concrete fix:** Implement a real demo mode that never reads or writes `vr_theme` or `sb_license:*`; use only `demo:*` keys or memory, suppress billing calls, and discard demo state on exit.

#### F-1-4 — The required claims inventory and claim tests do not exist

- **Location:** `.factory/claims.json` is absent; `rg '@claim:'` returns no tests.
- **Why this fails:** There are zero listed claims and therefore zero runnable claim tests. Passing general tests does not satisfy the claim contract. Every claim below is untested under the required clean demo entry point.
- **Concrete fix:** Add `.factory/claims.json`. Give every retained claim exactly one `@claim:<id>` test using `vr demo` or `/demo`; remove claims that cannot be observed. Run each listed command from a clean checkout.

#### F-1-5 — The site depicts a replay result that `vr report` cannot produce

- **Location/quote:** Home: “A report carries … the replay result into the pull request.” Preview: `Replay PASS / 204`.
- **Evidence:** In a fresh temporary vault, `vr report --name payment-failed --from 2024-04-10 --to 2025-02-24` emitted only contract differences. `ReportArgs` accepts no replay destination or replay-result input, and `markdown_report` contains no HTTP status.
- **Why this fails:** The primary proof artifact is depicted with data the product does not put in that artifact.
- **Concrete fix:** Either add a command that runs replay and records its destination/status in Markdown/JSON reports, with a claim test, or remove all replay-result copy and the `PASS / 204` row from the preview.

#### F-1-6 — The paid purchase action is dead

- **Location/quote:** “Buy the Pro unlock” → `https://api.sociobot.in/api/v1/products/api-version-replay/checkout`.
- **Evidence:** A live GET returns HTTP 404 with `{"error":"enabled factory product","status":404}`.
- **Why this fails:** A visitor cannot buy the advertised $29 product.
- **Concrete fix:** Register/enable the Sociobot product, verify the checkout redirect without embedding another provider, and add a non-spending link-health test. Until then, remove the buy action and purchasability copy.

#### F-1-7 — Unknown routes masquerade as the home page

- **Location:** `/this-route-does-not-exist` and `/404.html`.
- **Evidence:** Both return HTTP 200 with the home title and H1. There is no 404 source page and no `responseOverrides.404` rule.
- **Why this fails:** Broken links look successful, and visitors cannot recover from an invalid URL. This is broken routing.
- **Concrete fix:** Add a designed 404 page in the concrete/moss identity, return 404 through the host configuration, and link back home.

### Unlisted claim findings

Every row below is independently a finding because `.factory/claims.json` has no matching entry. The common failure is that a visitor is asked to rely on the statement without an observable clean-sandbox test. The fix for each is to add the named test (or remove the claim). Equivalent claims in multiple places may share an implementation test only if the inventory lists every location.

| ID | Exact quote/location | Required test |
| --- | --- | --- |
| F-1-8 | Meta description: “Capture, redact, compare, and replay old webhook contracts against localhost.” | CLI demo exercises all four outcomes. |
| F-1-9 | Meta description: “No tunnel or provider credentials required.” | Demo completes with those inputs/env vars absent. |
| F-1-10 | Hero: “Keep the exact webhook your integration understands.” | Import/capture and reload assert the stored request. |
| F-1-11 | Hero: “Redact it, label the provider version, compare the next contract, and replay both against localhost.” | End-to-end test asserts each verb and both replays. |
| F-1-12 | Hero: “No account · no tunnel · no telemetry.” | Clean env plus process/network request log. |
| F-1-13 | Proof strip: “Public destinations are refused.” | Attempt public IP, hostname, redirect, and DNS edge cases before network. |
| F-1-14 | Proof strip: “Sensitive fields never enter a fixture.” | Assert every documented default field/header is absent before disk write. |
| F-1-15 | Proof strip: “Markdown, JSON, and Pro JUnit.” | Export and parse each format in an isolated licensed fixture test. |
| F-1-16 | Workflow: “Every result points back to a saved wire contract.” | Assert identifiers/versions in every command result. |
| F-1-17 | Workflow: “Nothing depends on a provider sandbox staying available.” | Run the full demo with outbound network denied. |
| F-1-18 | Workflow: “Import JSON or catch one incoming request on a loopback listener.” | Import and capture tests assert the saved fixture. |
| F-1-19 | Workflow: “Version Replay applies body and header redaction before writing.” | Inspect disk during import and capture. |
| F-1-20 | Workflow: “Compare methods, paths, headers, primitive values, and inferred JSON types.” | Fixtures differ in every listed dimension and assertions cover each. |
| F-1-21 | Workflow: “A changed contract exits 3 in CI.” | Invoke the built binary and assert exit 3. |
| F-1-22 | Workflow: “Send the saved method, headers, and body to your running code.” | Loopback receiver asserts all three. |
| F-1-23 | Workflow: “Any non-loopback host is rejected before the network call.” | Request log proves no socket/request for rejected destinations. |
| F-1-24 | Demo: “Ready offline after first visit.” | Browser demo works after `context.setOffline(true)`. |
| F-1-25 | Demo help: “The contents do not leave this browser.” | Record the entire edit/compare/reset request log. |
| F-1-26 | Demo: “Zero uploads.” | Assert no request carries fixture content. |
| F-1-27 | Demo: “This comparison runs entirely in your browser.” | Block network after shell load and assert identical output. |
| F-1-28 | Report: “Reviewers see the contract—not a screenshot of a terminal.” | Generated report contains structured contract changes. |
| F-1-29 | Pricing: “Individual replay, redaction, encryption, diffing, and reports stay free.” | Clean unlicensed vault exercises every named feature. |
| F-1-30 | Pricing: “Pay once when migration work becomes a CI workflow.” | Billing fixture asserts one-time entitlement, no subscription. |
| F-1-31 | Pricing cards: `$0 forever`, `$29 once`, and all Free/Pro feature bullets. | Entitlement matrix test plus configured-price check. |
| F-1-32 | README: “Version Replay (`vr`) is a local-first CLI for engineers who need to prove that an integration still accepts an older third-party webhook or HTTP contract before they upgrade it.” | Clean CLI scenario proves the stated older-contract job. |
| F-1-33 | README: “It stores redacted, version-labelled fixtures on your laptop, compares their headers, values and JSON schemas, replays them only to localhost, and emits a reviewable Markdown or JSON report.” | End-to-end storage/diff/replay/export assertions. |
| F-1-34 | README: “No provider account, public tunnel or credentials are required.” | Run with clean environment and outbound network denied. |
| F-1-35 | README: “There is no telemetry.” | Process and browser network logs during the complete flow. |
| F-1-36 | README: “Build the single binary with Rust 1.82 or newer.” | CI matrix begins at Rust 1.82 and checks one output binary. |
| F-1-37 | README: “Common payment and identity fields and sensitive headers are redacted by default.” | Table-driven default redaction test. |
| F-1-38 | README: “An envelope may contain `method`, `path`, `headers`, and `body`; a plain JSON document is treated as the body of a `POST /` fixture.” | Parse both forms and assert defaults. |
| F-1-39 | README: “Replay the exact saved request to a local service.” | Loopback receiver compares stored and received request fields. |
| F-1-40 | README: “Version Replay refuses non-loopback destinations.” | Boundary test with no outbound request. |
| F-1-41 | README: “Capture the next request sent to a local listener, redact it, then stop.” | `capture --once` accepts one, saves redacted data, and exits. |
| F-1-42 | README: “Add `--json` before the subcommand for stable machine-readable output.” | Parse JSON for every supported subcommand and snapshot the schema. |
| F-1-43 | README exit-code sentence for `diff`, `replay`, and operational failures. | Built-binary tests assert 3, 4, and 1 for named cases. |
| F-1-44 | README: “Fixture contents use Argon2id key derivation and AES-256-GCM authenticated encryption; vault metadata stays readable.” | Inspect format/config and decrypt a known fixture. |
| F-1-45 | README: “The passphrase is never written to disk.” | Search all demo files for the supplied passphrase. |
| F-1-46 | README: “Losing it makes the fixture files unrecoverable.” | Wrong/missing passphrase cannot decrypt fixture content. |
| F-1-47 | README: “Body paths use dot notation and `*` for one segment.” | Nested wildcard redaction cases. |
| F-1-48 | README: “Header names are case-insensitive.” | Mixed-case header redaction cases. |
| F-1-49 | README: “Redaction happens before a fixture is written, including inside encrypted vaults.” | Observe both plain/encrypted writes for secret absence. |
| F-1-50 | README free-tier feature sentence. | Unlicensed feature matrix. |
| F-1-51 | README Pro-tier feature sentence. | Recorded valid-license fixture tests batch and JUnit. |
| F-1-52 | README: “Buy through the Sociobot-hosted checkout on the product site, then restore the token on any device.” | Checkout link and recorded restore flow; current link fails. |
| F-1-53 | README: “License checks are cached for 24 hours and never block free commands.” | Fake clock and offline/free-command test. |
| F-1-54 | README: “`npm test` runs Rust unit/integration tests plus site tests.” | CI inspects all invoked test suites. |
| F-1-55 | README: “`npm run build` creates the release binary in `dist/bin/vr` and the static landing/docs site in `dist/site/`.” | Clean build asserts both artifacts. |
| F-1-56 | README browser-acceptance sentence. | Each named check gets an explicit assertion and claim tag. |
| F-1-57 | README: “Everything in the vault stays local.” | Deny/log network during every non-license CLI command. |
| F-1-58 | README replay/redaction/encryption-at-rest sentence. | Split into three separately tagged observable tests. |
| F-1-59 | README: “MIT.” | Assert the shipped `LICENSE` is MIT. |
| F-1-60 | Privacy: “The CLI has no analytics, telemetry, account system, hosted relay, or automatic fixture upload.” | Full CLI request/process log with all commands. |
| F-1-61 | Privacy data-storage and pre-write-redaction statements. | Assert the only files and contents created in a temporary vault. |
| F-1-62 | Privacy encryption/passphrase statements. | Cryptographic format and filesystem-secret tests. |
| F-1-63 | Privacy: replay is loopback-only and Sociobot is contacted only for license verification/reconciliation. | Network allowlist test across every CLI command. |
| F-1-64 | Privacy: website stores theme/license/verdict and removes the token from the address bar. | Fresh-context localStorage and URL test. |
| F-1-65 | Privacy: the JSON demo stays in-browser and does not upload fixtures. | Demo request-body log. |
| F-1-66 | Privacy: no advertising cookies or behavioral analytics. | Cookie and request inventory after the full site flow. |

### Major and minor findings

#### F-1-67 — Route changes do not restore position, move focus, or announce the new page

- **Location:** Home `#bench`, `/privacy/`, `/terms/`.
- **Evidence:** After activating the bench link and after loading Privacy, `document.activeElement` was `BODY`; the H1 was not focused. Legal pages have no `aria-live` route announcer. From a fresh home load, opening `#bench` scrolled to about 2745 px; Back changed the URL to `/` but left the scroll position at about 2752 px instead of restoring the top. Forward returned to `#bench` at the same position.
- **Why this matters:** Keyboard and screen-reader users are not placed at the new content.
- **Concrete fix:** On History API route changes, focus a temporarily focusable H1 and announce its text. Preserve back/forward position and focus with an automated test.

#### F-1-68 — Canonical, social, and complete favicon metadata are absent

- **Location:** All live routes.
- **Evidence:** No canonical, Open Graph title/description/image, Twitter card, or apple-touch icon was present. Privacy and Terms have no favicon at all. Home has only a data-URI SVG favicon.
- **Concrete fix:** Add a per-route canonical; OG/Twitter tags; an original 1200 × 630 social image; SVG favicon; and 180 px apple-touch icon. Verify emitted production HTML.

#### F-1-69 — Header/footer skeleton is inconsistent and incomplete

- **Location:** Home versus Privacy/Terms.
- **Evidence:** Home header has Workflow/Try it/Pricing and theme control; legal headers omit Try it and the theme control. No header links to Privacy. Footers omit “Built by Param Factory” and a version/build id.
- **Concrete fix:** Use one header/footer component across routes, include Demo and Privacy within the four-link limit, and add factory attribution plus build id.

#### F-1-70 — External links are not consistently identified

- **Location:** Footer “Source”; report “Read the CLI guide” uses only a visual arrow hidden from assistive technology.
- **Why this matters:** The destination change is not conveyed consistently in link text or accessible names.
- **Concrete fix:** Use text such as `Read the CLI guide on GitHub (opens in a new site)` and `Source on GitHub`.

#### F-1-71 — The sitemap cannot list the required demo and 404 routes

- **Location:** `site/public/sitemap.xml` lists only `/`, `/privacy/`, and `/terms/`.
- **Concrete fix:** After implementing the real demo and 404, include the canonical public routes appropriate for indexing and test every listed URL.

## Copy audit

Counting method: an alphanumeric token is one word; a hyphenated term or date counts as one; slash-separated terms count separately. Markup is excluded. Sentence fragments, headings, and actions are audited separately after the sentence lists.

### Landing-page sentences

| # | Words | Sentence | Flag |
| --- | ---: | --- | --- |
| L-01 | 10 | Capture, redact, compare, and replay old webhook contracts against localhost. | F-1-8 |
| L-02 | 6 | No tunnel or provider credentials required. | F-1-9 |
| L-03 | 3 | Re-run the wire. | F-1-1 |
| L-04 | 3 | Before it snaps. | F-1-1 |
| L-05 | 7 | Keep the exact webhook your integration understands. | F-1-10 |
| L-06 | 15 | Redact it, label the provider version, compare the next contract, and replay both against localhost. | F-1-11 |
| L-07 | 4 | Public destinations are refused. | F-1-13 |
| L-08 | 6 | Sensitive fields never enter a fixture. | F-1-14 |
| L-09 | 9 | Every result points back to a saved wire contract. | F-1-16 |
| L-10 | 8 | Nothing depends on a provider sandbox staying available. | F-1-17 |
| L-11 | 7 | A migration artifact, not another mock server. | F-1-74 |
| L-12 | 11 | Import JSON or catch one incoming request on a loopback listener. | F-1-18 |
| L-13 | 9 | Version Replay applies body and header redaction before writing. | F-1-19 |
| L-14 | 10 | Compare methods, paths, headers, primitive values, and inferred JSON types. | F-1-20 |
| L-15 | 7 | A changed contract exits 3 in CI. | F-1-21 |
| L-16 | 11 | Send the saved method, headers, and body to your running code. | F-1-22 |
| L-17 | 9 | Any non-loopback host is rejected before the network call. | F-1-23 |
| L-18 | 6 | Find the break in two payloads. | F-1-79 |
| L-19 | 8 | Edit either JSON fixture, then compare them locally. | — |
| L-20 | 7 | The contents do not leave this browser. | F-1-25 |
| L-21 | 2 | Zero uploads. | F-1-26 |
| L-22 | 7 | This comparison runs entirely in your browser. | F-1-27 |
| L-23 | 3 | Bench is ready. | F-1-80 |
| L-24 | 10 | Compare the two specimens to reveal schema and value changes. | F-1-81 |
| L-25 | 5 | Attach evidence to the upgrade. | F-1-82 |
| L-26 | 17 | A report carries the tested versions, every wire-level change, and the replay result into the pull request. | F-1-5 |
| L-27 | 10 | Reviewers see the contract—not a screenshot of a terminal. | F-1-28 |
| L-28 | 3 | One useful tool. | F-1-84 |
| L-29 | 3 | One optional upgrade. | F-1-84 |
| L-30 | 9 | Individual replay, redaction, encryption, diffing, and reports stay free. | F-1-29 |
| L-31 | 9 | Pay once when migration work becomes a CI workflow. | F-1-30 |
| L-32 | 2 | One-time purchase. | F-1-31 |
| L-33 | 11 | Sociobot/Dodo is the merchant of record; refunds are handled there. | F-1-31 |
| L-34 | 3 | Already bought Pro? | — |
| L-35 | 17 | Paste your license token to restore this browser, then activate the same token with `vr license activate`. | — |
| L-36 | 6 | No license saved in this browser. | — |
| L-37 | 2 | Keep it. | F-1-86 |
| L-38 | 2 | Test it. | F-1-86 |
| L-39 | 2 | Then move. | F-1-86 |
| L-40 | 7 | Local migration insurance for versioned inbound contracts. | F-1-87 |

No landing sentence exceeds 22 words. That does not make the copy clear: the primary failures are metaphor, invented workshop language, inconsistent terms, and untested claims.

### README sentences

| # | Words | Sentence | Flag |
| --- | ---: | --- | --- |
| R-01 | 29 | Version Replay (`vr`) is a local-first CLI for engineers who need to prove that an integration still accepts an older third-party webhook or HTTP contract before they upgrade it. | F-1-88 |
| R-02 | 28 | It stores redacted, version-labelled fixtures on your laptop, compares their headers, values and JSON schemas, replays them only to localhost, and emits a reviewable Markdown or JSON report. | F-1-89 |
| R-03 | 9 | No provider account, public tunnel or credentials are required. | F-1-34 |
| R-04 | 4 | There is no telemetry. | F-1-35 |
| R-05 | 10 | Build the single binary with Rust 1.82 or newer: | F-1-36 |
| R-06 | 19 | Prebuilt release archives are intended to be attached by the factory after launch; this repository does not publish itself. | — |
| R-07 | 4 | Create a local vault. | — |
| R-08 | 12 | Common payment and identity fields and sensitive headers are redacted by default: | F-1-37 |
| R-09 | 5 | Import two saved webhook envelopes. | F-1-90 |
| R-10 | 22 | An envelope may contain `method`, `path`, `headers`, and `body`; a plain JSON document is treated as the body of a `POST /` fixture. | F-1-38 |
| R-11 | 7 | Compare versions and save the pull-request artifact: | F-1-91 |
| R-12 | 9 | Replay the exact saved request to a local service. | F-1-39 |
| R-13 | 5 | Version Replay refuses non-loopback destinations: | F-1-40 |
| R-14 | 13 | Capture the next request sent to a local listener, redact it, then stop: | F-1-41 |
| R-15 | 9 | Add `--json` before the subcommand for stable machine-readable output. | F-1-42 |
| R-16 | 21 | `diff` exits 3 when it finds a difference; `replay` exits 4 when localhost returns a non-2xx response; operational failures exit 1. | F-1-43 |
| R-17 | 11 | Set a passphrase in an environment variable and initialize with encryption. | — |
| R-18 | 14 | Fixture contents use Argon2id key derivation and AES-256-GCM authenticated encryption; vault metadata stays readable. | F-1-44 |
| R-19 | 7 | The passphrase is never written to disk. | F-1-45 |
| R-20 | 7 | Losing it makes the fixture files unrecoverable. | F-1-46 |
| R-21 | 9 | Body paths use dot notation and `*` for one segment. | F-1-47 |
| R-22 | 4 | Header names are case-insensitive. | F-1-48 |
| R-23 | 11 | Redaction happens before a fixture is written, including inside encrypted vaults. | F-1-49 |
| R-24 | 16 | The free CLI includes local capture, redaction, encrypted storage, individual replay, contract diff and report export. | F-1-50 |
| R-25 | 16 | A one-time Pro license adds batch replay across every saved version and JUnit output for CI. | F-1-51 |
| R-26 | 16 | Buy through the Sociobot-hosted checkout on the product site, then restore the token on any device: | F-1-52 |
| R-27 | 12 | License checks are cached for 24 hours and never block free commands. | F-1-53 |
| R-28 | 7 | Sociobot/Dodo is the merchant of record. | — |
| R-29 | 10 | `npm test` runs Rust unit/integration tests plus site tests. | F-1-54 |
| R-30 | 20 | `npm run build` creates the release binary in `dist/bin/vr` and the static landing/docs site in `dist/site/`. | F-1-55 |
| R-31 | 18 | To work on the site, use `npm run dev`; to build only it, use `npm run build:site`. | — |
| R-32 | 12 | Run the browser acceptance sweep against a built deployment or local preview. | — |
| R-33 | 23 | It checks desktop and 390 px mobile interaction, keyboard comparison, reduced motion, axe serious/critical findings, same-origin browsing, and an immediate offline reload: | F-1-92 |
| R-34 | 6 | Create the registry artifact without publishing: | — |
| R-35 | 6 | Everything in the vault stays local. | F-1-57 |
| R-36 | 18 | Replays are restricted to loopback hosts, default redaction is conservative, and the optional vault is encrypted at rest. | F-1-58 |
| R-37 | 7 | See the site’s privacy policy and terms. | — |
| R-38 | 11 | Security reports can be opened as private advisories in the repository. | — |
| R-39 | 6 | Do not attach real webhook payloads. | — |
| R-40 | 1 | MIT. | F-1-59 |
| R-41 | 2 | See LICENSE. | — |

The README average is 11.6 words, but R-01, R-02, and R-33 breach the 22-word cap.

### Copy flags and proposed rewrites

Each row is a separate copy finding.

| ID | Exact copy/location | Problem | Proposed rewrite |
| --- | --- | --- | --- |
| F-1-72 | `LOCAL CONTRACT INSTRUMENT / 0.1` | Decorative product lore; does not help a cold visitor. | Delete it, or use `LOCAL CLI · VERSION 0.1` only if the version is current. |
| F-1-73 | `SPECIMEN 024` / `CONTRACT / INTACT` | Invented specimen lore implies a status without evidence. | `Saved webhook fixture` / `Example data`. |
| F-1-74 | `THE THREE-CUT METHOD` and “A migration artifact, not another mock server.” | Invented method name, jargon, and competitor framing do not name the section. | `How Version Replay works` / `Save two webhook versions, compare them, then replay either one locally.` |
| F-1-75 | `Seal the fixture` | Metaphor. | `Save and redact a fixture`. |
| F-1-76 | `Expose the shift` | Vague metaphor. | `Compare version changes`. |
| F-1-77 | `Press it against localhost` | Metaphor. | `Replay it to localhost`. |
| F-1-78 | `LIVE CONTRACT BENCH / BROWSER-LOCAL` | “Bench” is brand lore and the label does not say this is only a simplified browser comparison. | `Browser sample · compares JSON only`. |
| F-1-79 | “Find the break in two payloads.” | “Break” is vague and “payloads” conflicts with “fixtures/specimens/contracts.” | `Compare two sample webhook fixtures.` |
| F-1-80 | “Bench is ready.” | The empty state names a metaphor, not what will appear. | `No comparison yet.` |
| F-1-81 | “Compare the two specimens to reveal schema and value changes.” | “Specimens” changes the name of the fixture. | `Compare the two fixtures to list type and value changes.` |
| F-1-82 | `THE REVIEW ARTIFACT` / “Attach evidence to the upgrade.” | Jargon and an indirect heading. | `Export a migration report`. |
| F-1-83 | `KEEP THE CORE FREE` | Slogan; it does not name pricing. | `Free and Pro features`. |
| F-1-84 | “One useful tool. One optional upgrade.” | Generic slogan that could describe any product. | `Use the CLI free, or pay $29 once for batch replay and JUnit.` |
| F-1-85 | `THE OLD CONTRACT IS STILL A CONTRACT` | Mood slogan with no new information. | Delete it. |
| F-1-86 | “Keep it. Test it. Then move.” | Mood slogan with no named object or result. | `Install Version Replay and test a saved webhook locally.` |
| F-1-87 | “Local migration insurance for versioned inbound contracts.” | “Insurance” is a metaphor; “inbound contracts” is jargon. | `Compare and replay saved webhook versions on your machine.` |
| F-1-88 | README R-01 (29 words) | Over 22 words and uses “local-first” and “prove” abstractly. | `Version Replay is a local CLI for engineers testing older webhook contracts before a provider API upgrade.` |
| F-1-89 | README R-02 (28 words) | Over 22 words, several ideas, and “reviewable” is subjective. | `It stores redacted fixtures by provider version on your laptop. Compare changes, replay requests to localhost, and export Markdown or JSON reports.` |
| F-1-90 | “Import two saved webhook envelopes.” | “Envelope” adds a fourth name for the same input. | `Import two saved webhook fixtures.` |
| F-1-91 | “Compare versions and save the pull-request artifact:” | “Artifact” is vague. | `Compare the versions and export a Markdown report:` |
| F-1-92 | README R-33 (23 words) | Over 22 words and “axe serious/critical” is tool jargon. | `It checks desktop and mobile use, keyboard controls, reduced motion, accessibility, same-origin requests, and offline reloads.` |
| F-1-93 | Button label `Copy` | The button does not name the copied result. | `Copy install command`. |
| F-1-94 | Header link `Try it` | It does not say what data or result opens. | `Try sample data`. |
| F-1-95 | Link `Open the comparison bench` | “Bench” is metaphorical and the action hides that no result is ready. | `Compare sample fixtures`. |
| F-1-96 | Button `Reset specimens` | “Specimens” is inconsistent and does not identify sample state. | `Reset sample fixtures`. |
| F-1-97 | Link `Install the CLI` | It only jumps back to a copy control; it does not install or open install instructions. | `Copy the install command` if it copies, or `View install steps` if it navigates. |
| F-1-98 | `webhook`, `HTTP contract`, `wire contract`, `fixture`, `envelope`, `payload`, `specimen`; and `report` / `artifact` | The same core data and output change names throughout the page and README. | Use `webhook fixture` for stored input, `contract changes` for the comparison, and `report` for exported output everywhere. |

No banned marketing adjective from the supplied list was found. The problem is product-specific metaphor and terminology drift rather than generic superlatives.

## Demo and sandbox evidence

- One-click path: partial only. “Open the comparison bench” reaches realistic payment webhook JSON after one click, but no comparison result is present until a second click.
- Product fidelity: fail. The browser comparator does not demonstrate CLI import, versioned local storage, replay, or report export.
- Required banner/actions: absent. No `Demo — sample data, nothing is saved`, `Reset demo`, or `Start for real`.
- Reset: the browser's `Reset specimens` restores both textarea values and focuses the first textarea. This isolated control works.
- Browser request log from a fresh comparison flow: same-origin HTML/JS/CSS/WebP only; no fixture upload was observed.
- Demo isolation: fail as F-1-3; `/demo` uses real storage keys and can make a real license-verification request.
- CLI sandbox: fail as F-1-2; `vr demo` does not exist.

## Claim-test results

`.factory/claims.json` could not be read because it does not exist. Therefore:

| Declared claims | Tests requested by inventory | Passed | Failed/untested |
| ---: | ---: | ---: | ---: |
| 0 | 0 | 0 | Every live/README claim is untested under the claims contract |

This is not a vacuous pass. F-1-4 and F-1-8 through F-1-66 remain blocking until the inventory and tagged tests exist.

General checks from the isolated checkout did pass:

- `npm ci`: pass, 0 reported vulnerabilities.
- `npm test`: pass; 3 Rust unit tests, 3 CLI integration tests, 4 site tests.
- `npm run typecheck`: pass.
- `npm run build`: pass; produced `dist/bin/vr` and `dist/site/`.
- `npm run test:browser -- https://api-version-replay.sociobot.in/`: pass; desktop keyboard/offline, 390 px no-overflow, and axe scan.
- `/opt/fleet/lib/verify-url.sh`: pass; title/lang/main/alt/button labels and zero console errors.

## Structure, accessibility, and visual identity

Confirmed working:

- Home title follows `Product — what it does` and is under 60 characters.
- Privacy and Terms have route-specific titles.
- All checked routes have `lang="en"`, one H1, and one main landmark.
- Home image has alt text; axe reported no serious/critical findings on Home, Privacy, or Terms.
- Existing keyboard comparison, visible focus styling, reduced-motion behavior, 390 px layout, and offline reload pass.
- Live CSP, clickjacking policy, permissions policy, `nosniff`, immutable asset caching, and no-cache service worker headers are present.
- Home, Privacy, Terms, robots, sitemap, and GitHub links return 200, and every in-page hash target exists. The only dead crawled action is the checkout link in F-1-6.
- The brutalist concrete/moss art, hard rules, condensed type, and green seam are recognizably product-specific and match `.factory/design.md`; this is not a generic SaaS template.

Still failing:

- F-1-6, F-1-7, and F-1-67 through F-1-71 cover the dead purchase link, 404 behavior, navigation focus, metadata, skeleton, external-link naming, and sitemap.

## History verification

No earlier `.factory/review-*.md` or `.factory/polish-*.md` files exist. The earlier `.factory/handoff.md` and verification reports describe one P1: response security/cache policy was not applied. That exact issue is fixed in live code and deployment:

- Home returns CSP, `X-Frame-Options: DENY`, Permissions-Policy, `nosniff`, and referrer policy.
- Hashed JS returns `Cache-Control: public, max-age=31536000, immutable`.
- `service-worker.js` returns `Cache-Control: no-cache`.
- `/_headers` now receives the site fallback rather than exposing the old raw config.

The handoff's known paid-license coverage gap is not resolved: the live checkout endpoint now verifies as a dead 404 link (F-1-6).

## Missed leverage

The obviously missing high-value step is not decorative AI. Contract comparison and replay need deterministic behavior, and the brief prioritizes local-only data; adding an AI feature would weaken that fit. Import and report export already exist. The missed leverage is a real `vr demo` workflow and, if the site keeps promising it, a single command/report path that includes the actual local replay result (F-1-2 and F-1-5). No provider keys were found in the product.

## What would make this perfect

1. Replace the metaphorical first screen with the job, audience, one sample-data action, and three tested facts.
2. Ship the real isolated CLI/browser demo, banner, reset/exit controls, and `.factory/demo.md`.
3. Add the complete claims inventory and tagged clean-sandbox tests; remove any claim that cannot be observed.
4. Make the report match its preview, or remove the replay-result promise.
5. Enable and test Sociobot checkout before advertising Pro.
6. Add the real 404, route focus/announcements, full metadata, consistent shell, attribution, build id, and updated sitemap.
7. Apply every copy rewrite/terminology decision above and rerun the entire review from a fresh context.

Until every finding is closed and every retained claim has passed its declared test, the verdict remains **FAIL**.
