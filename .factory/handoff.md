# Review 2 handoff — Version Replay

## Outcome

Completed the requested adversarial first-read review without modifying product code. The review is recorded in `.factory/review-2.md` and the verdict is **FAIL**.

## What was verified

- Fresh live browser contexts at 390 px and desktop confirmed the first screen clearly states the job, audience, and primary action.
- The `/?demo=1` browser sandbox has its banner, reset/exit controls, realistic fixture diff, no browser-storage access, no cookies, and same-origin requests only.
- `vr demo` was run from a fresh temporary working directory. It imported/redacted two fixtures, compared five changes, replayed both to loopback HTTP 204, and wrote a Markdown report in a unique temporary directory.
- Every command listed in `.factory/claims.json` passed from a clean clone, as did `npm test`, typecheck, build, rustfmt, and clippy. The clone produced `dist/bin/vr` and `dist/site/`.
- Checked live routes, 404, metadata, links, headers, keyboard/focus behaviour, 390 px layout, offline demo, and accessibility. Axe reported no serious or critical violations.

## Findings left

1. **Blocking, re-opened F-1-2:** The primary one-click sample opens a browser JSON comparator, not the complete CLI import/redaction/replay/report workflow it promises.
2. **Minor F-2-1 to F-2-3:** Three concrete copy promises do not have matching listed claim tests.
3. **Minor F-2-4:** The live browser routing assertion is timing-sensitive; one run failed before scroll restoration settled and a rerun passed.

## Re-run

```sh
npm ci
npm test
npm run typecheck
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
npm run test:browser -- https://api-version-replay.sociobot.in/
```

For the complete findings and evidence, read `.factory/review-2.md`.
