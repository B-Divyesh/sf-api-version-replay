# Version Replay visual thesis

## Direction: brutalist concrete and moss

Version Replay is a workshop instrument for inspecting old wire contracts, not a cloud dashboard. Its visual world borrows from stamped concrete test blocks, fluorescent survey marks, damp moss and terminal paper: weighty, local and visibly versioned. Hard edges and exposed rules convey determinism; a living moss accent marks the one path that still passes through the contract.

The site supports light and dark treatments. Light is poured concrete in daylight; dark is the same bench after hours. Neither changes the semantic hierarchy.

## Tokens

| Role | Light | Dark | Reason |
| --- | --- | --- | --- |
| background | `#D9D8CE` | `#171A16` | warm aggregate, never SaaS-white |
| surface | `#EEEDE3` | `#222720` | paper/specimen slab |
| raised surface | `#F8F7EF` | `#2B3028` | active work layer |
| text | `#171A16` | `#F0F0E7` | near-black/limestone |
| muted text | `#50564C` | `#B9C0B3` | readable annotations |
| rule | `#6B7065` | `#858D80` | exposed structural lines |
| moss accent | `#315E2D` | `#A9D66F` | viable contract path |
| accent contrast | `#FFFFFF` | `#13200F` | label on moss |
| success | `#245B37` | `#8ED09C` | replay received |
| warning | `#704E00` | `#F1C75B` | changed contract |
| danger | `#8A2820` | `#FF9389` | failed replay |

All body/text pairs are designed to meet WCAG AA at 4.5:1. State always has an icon or written label as well as color. Focus is a 3 px moss outline with a 2 px background offset.

## Type and spacing

- Display and body: `Arial Narrow`, `Roboto Condensed`, `Liberation Sans Narrow`, then system sans. Its compressed, labelled-crate character fits a CLI without loading a font.
- Code/data: `ui-monospace`, `SFMono-Regular`, `Cascadia Code`, `Liberation Mono`, monospace. Tabular figures are enabled.
- Scale: 14 / 16 / 20 / 28 / clamp(44, 7vw, 88) px; body is never below 16 px.
- Spacing uses a strict 4 px base: 4, 8, 12, 16, 24, 32, 48, 72. Borders are 2 px; shadows are hard 6 px offsets, used only where layers genuinely overlap.
- Desktop content maxes at 1180 px. Narrative measures stay under 70 characters. At 390 px, the comparison scene stacks and the decorative metadata rail drops.

## Interaction grammar

Buttons depress by translating into their hard shadow; commands have a copy affordance and immediate live-region confirmation. Tabs behave as tabs with arrow-key navigation. The demo is one sequential bench: choose a stored version, inspect the contract delta, then replay. Empty, failure and offline conditions preserve the same bench rather than opening modals.

## Motion policy

UI feedback lasts 160–220 ms and changes only opacity or transform. The hero's version sheets enter from their physical stack positions once; nothing loops. With `prefers-reduced-motion: reduce`, transforms and smooth scrolling are removed and all states switch instantly. The meaning remains in position, labels and rules.

## Asset plan and provenance

- Hero: one original raster still, generated for this product with `/opt/fleet/lib/gen-image.sh` using the factory image deployment, then locally cropped/optimized to WebP at no more than 300 KB. It depicts a small concrete contract specimen with moss tracing a payload seam, photographed like an archival lab object. It contains no text, logo, people or provider marks. Final filename: `site/public/version-specimen.webp`.
- Prompt: “Editorial product photograph for a developer-tool landing page. A single rough cast-concrete archive tile on a charcoal-black workbench, its top surface formed by three thin offset layers like versioned documents. One precise seam carries a narrow line of living green moss, suggesting a compatible data path through old layers. Sparse brass registration pins and subtle punched holes imply HTTP headers and JSON structure without readable text. Brutalist material study, tactile aggregate, restrained moss green, warm grey concrete, hard side light, deep controlled shadows, wide horizontal composition with generous dark negative space on the left, object weighted to the right. No text, no letters, no numbers, no logo, no UI screenshot, no neon, no gradient, no people, no watermark.”
- License/provenance: generated specifically for Version Replay in this build; owned project asset, not sourced from a stock library. Generation metadata is retained beside the original during production.
- Social card: `site/public/social-card.jpg`, cropped and darkened from the same original product asset at 1200 × 630. No third-party asset was introduced.
- Favicon and Apple touch icon: hand-authored geometric replay arrows in the product's charcoal and moss tokens.
- Interface icons are hand-authored CSS or Unicode symbols with visible text labels; there is no third-party icon pack.

## Why it fits

Concrete makes saved contracts feel fixed and inspectable; moss represents compatibility persisting through time. The visual contrast mirrors the job: compare rigid historical payloads, find the living path, and replay it locally with evidence.
