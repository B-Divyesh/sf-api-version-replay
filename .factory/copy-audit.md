# Copy audit

Audited 2026-08-29 after perfection-loop round 3. Counts treat a hyphenated term as one word. No sentence exceeds 22 words. No supplied banned marketing word appears.

## Landing-page sentences

| Words | Sentence | Result |
| ---: | --- | --- |
| 18 | Test saved webhook versions against localhost with a local CLI that redacts, compares, replays, and reports contract changes. | Pass |
| 9 | Save, compare, and replay old webhook versions against localhost. | Pass |
| 6 | Test old webhook versions against localhost. | Pass |
| 10 | For engineers checking an integration before upgrading a provider API. | Pass |
| 9 | See saved versions, contract changes, and local replay results. | Pass |
| 5 | You choose the vault directory. | Pass; `@claim:vault-directory` |
| 6 | Configured fields are removed before storage. | Pass |
| 4 | Public destinations are rejected. | Pass |
| 9 | Use a saved webhook fixture for each provider version. | Pass |
| 10 | Import JSON or capture one request on a loopback listener. | Pass |
| 8 | List method, path, header, type, and value changes. | Pass |
| 10 | Send either saved request to code running on your machine. | Pass |
| 8 | Edit either sample webhook fixture, then compare them. | Pass |
| 9 | Sample contents are not sent to a server. | Pass |
| 5 | The browser sample compares JSON. | Pass |
| 8 | Run the CLI demo for import and replay. | Pass |
| 3 | No comparison yet. | Pass |
| 10 | Compare the fixtures to list type and value changes. | Pass |
| 14 | `vr demo` creates a new temporary vault and leaves its report there for inspection. | Pass; `@claim:cli-demo-workflow` |
| 7 | Recorded from the bundled payment webhook fixtures. | Pass |
| 11 | The Markdown report names both versions and lists their structured changes. | Pass |
| 8 | It does not send replays to public hosts. | Pass |
| 8 | It does not replace provider integration tests. | Pass |
| 11 | The browser sample uses only the text shown on this page. | Pass |
| 11 | Real webhook data belongs in the CLI, not this sample page. | Pass |
| 9 | Compare and replay saved webhook versions on your machine. | Pass |

## Dynamic interface sentences

| Words | Sentence | Result |
| ---: | --- | --- |
| 5 | Both webhook fixtures need JSON. | Pass |
| 2 | Fixtures match. | Pass |
| 8 | No type or value changes were found. | Pass |
| 2 | Comparison stopped. | Pass |
| 6 | Check commas, quotes, and closing braces. | Pass |
| 7 | Install command copied to the clipboard. | Pass |
| 6 | Demo — sample data, nothing is saved. | Pass |
| 5 | Offline · sample still works. | Pass |

## Headings, labels, actions, and status fragments

All are sentence case or compact terminal labels. Word counts range from one to eight. The primary action is `Try it with sample data`. Secondary actions name their result: `Copy install command`, `Compare sample fixtures`, `Reset sample fixtures`, `Reset demo`, `Start for real`, and `Return home`.

The demo H1 is `Replay the complete CLI sample`. The terminal recording is command output, not marketing prose. Its quantitative line reports five changes and is asserted by `@claim:primary-demo-workflow` and `@claim:cli-demo-workflow`.

## README changes checked in round 2

`The command prints the temporary vault and report paths.` has nine words and is proved by `@claim:demo-output-paths`. `Install the CLI from this checkout:` is an instruction, not a build-artifact promise.

The catalog sentence has 11 words, starts with `Test`, and is 78 characters: `Test saved webhook versions against localhost before upgrading a provider API.`

## Round 3 legal copy

The unsupported `no account system` clause was removed. `The project has no telemetry client.` has six words and maps to `@claim:no-telemetry`.

The unsupported retention adjective was removed. `The hosting provider may process request and security logs under its own retention policy.` has 13 words and makes no product-controlled retention promise.

## Terminology

| Concept | One term |
| --- | --- |
| Stored request input | webhook fixture |
| Provider identifier | version |
| Compared output | contract changes |
| Exported file | report |
| Browser experience | browser sample |
| Isolated one-click path | demo |
| CLI storage directory | vault |
| Request target | loopback / localhost |

Removed terms: specimen, payload, envelope, wire contract, bench, artifact, seal, press, migration insurance, and unlock.
