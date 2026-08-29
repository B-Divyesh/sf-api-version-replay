# Adversarial review 4 handoff — Version Replay

## Outcome

**FAIL.** Review 4 is recorded in `.factory/review-4.md` against live <https://api-version-replay.sociobot.in/> and repository commit `7284df2b8cc6eca20d737912e8094fc63fefccbf`.

No product code was modified. The review and this handoff are the only intended repository changes.

## What was verified

- Cold first read at 390 × 844 and 1440 × 900.
- One-click browser Demo, completed CLI output, Reset, Start for real, storage isolation, same-origin requests, and live offline reload.
- Direct `vr demo` run from a fresh temporary working directory.
- All 20 claim commands separately from clean clone `/tmp/api-version-replay-review4.4ozSuZ`.
- Clean-clone `npm test`, `npm run typecheck`, and `npm run build`.
- Live link/fragment crawl, response headers, 404 status, metadata, route focus/history, mobile overflow, both themes, and Playwright axe checks.
- Every earlier review/polish finding against live behavior and current source.
- Landing and README copy, claims coverage, site skeleton, visual identity, and missed leverage.

## Remaining findings

- `F-4-1 / F-1-67` blocking: full-page Back navigation returns Home with focus on `BODY`.
- `F-4-2` blocking: mobile terminal/result overflow regions fail serious axe keyboard access.
- `F-4-3 / F-1-68` blocking: the 404 lacks a canonical and complete Twitter metadata.
- `F-4-4` minor: Terms makes an unlisted future-publication promise.
- `F-4-5` minor: the landing page explains the workflow before showing the product preview.

## Reproduce

```sh
npm ci
npm test
npm run typecheck
npm run build
npm run test:browser -- https://api-version-replay.sociobot.in/
```

The existing browser smoke passes but does not cover the failing full-page Back path or mobile axe scan. Reproduction details and concrete fixes are in `.factory/review-4.md`.
