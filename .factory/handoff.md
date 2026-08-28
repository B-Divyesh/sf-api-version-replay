# Version Replay adversarial review 1 handoff

## Outcome

Review verdict: **FAIL**. The complete evidence and 98 findings are in `.factory/review-1.md`.

No product code was changed. This work order changed only the review and this handoff.

## What was reviewed

- Cold live loads at 390 × 844 and 1440 × 900 before scrolling.
- Landing/README sentence inventory, word counts, headings, terminology, and action labels.
- Browser sample flow, reset, request log, offline behavior, storage namespaces, and `/demo` behavior.
- CLI behavior from temporary directories, including the missing `vr demo` command and actual report output.
- All live links, route status, metadata, 404 behavior, focus/history behavior, header/footer structure, and visual identity.
- `.factory/claims.json` and claim tags; both are absent.
- Earlier handoff/verification history; the prior response-policy/cache P1 remains fixed.
- Accessibility via the repository Playwright/axe sweep and the factory URL verifier.

## Verification run

From isolated detached worktree `/tmp/api-version-replay-review.KW9H0h` at `d29c2bf91857b8cbbf1e293140442fa00a963e31`:

```sh
npm ci
npm test
npm run typecheck
npm run build
```

All passed. The build produced `dist/bin/vr` and `dist/site/`.

Live checks:

```sh
npm run test:browser -- https://api-version-replay.sociobot.in/
/opt/fleet/lib/verify-url.sh https://api-version-replay.sociobot.in/ <temporary-evidence-dir>
```

Both passed their existing assertions. Additional Playwright/curl checks produced the failures documented in the review.

## Blocking gaps

- The first screen uses a metaphorical headline, omits the audience, and has no single sample-data action.
- There is no real isolated demo, `vr demo`, demo banner, `.factory/demo.md`, or demo storage namespace.
- `.factory/claims.json` and `@claim:` tests are missing; all live/README claims are unlisted.
- The report preview promises `Replay PASS / 204`, but real reports contain no replay result.
- The live Pro checkout link returns HTTP 404.
- Unknown routes return the home page with HTTP 200; there is no designed 404.

See `.factory/review-1.md` for exact quotes, rewrites, test requirements, and the remaining structure/copy findings.
