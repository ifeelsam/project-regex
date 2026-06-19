# Project Regex — Design System

> Find the signal in the noise. A capture-to-creation tool for people who collect
> inspiration faster than they can use it. Save the spark, keep the reason it grabbed
> you, and turn it into finished work.

**Voice:** plain, confident, a little technical. The product name is a wink — a regular
expression finds the meaningful pattern hidden in a wall of text. Tagline:
*capture light · break down heavy · lose nothing.*

---

## 1. Logo & marks

All marks live in [`/assets`](./assets). The `.*` glyph reads as the regex token for
"match anything."

| File | Use |
|------|-----|
| `assets/icon-app.svg` | Primary app icon — vermilion `.*` on Ink, 60px-radius squircle. Drawn as vector (font-independent). |
| `assets/icon-app-paper.svg` | Light surface variant for light backgrounds. |
| `assets/icon-mono.svg` | Single-color mark, inherits `currentColor`. Use anywhere one color is needed. |
| `assets/favicon.svg` | 64px favicon (same artwork as app icon). |
| `assets/wordmark.svg` | Primary wordmark `project /regex/` (Ink, vermilion slashes). |
| `assets/wordmark-reversed.svg` | Wordmark on Ink for dark surfaces. |
| `assets/mark-mono-flag.svg` | Compact `/regex/g` mono mark — "the global flag." |

**Rules**
- The two slashes `/ … /` are always Vermilion. Never recolor them.
- Minimum clear space around any mark = the height of the `.*` dot.
- The icon glyph in `icon-app.svg` is vector — scale freely. Wordmarks use the live
  Bricolage / JetBrains Mono webfonts; **outline to paths** before handing off to anyone
  without the fonts (or for print).
- Never stretch, rotate, add shadows to, or re-space the wordmark.

---

## 2. Color

| Token | Hex | Role |
|-------|-----|------|
| **Vermilion** | `#E64A2E` | Primary action, the spark, the slashes. The only hot color. |
| **Ink** | `#1A1813` | Text, headlines, dark surfaces. |
| **Paper** | `#F1ECDF` | App background. |
| **Surface** | `#FCFAF3` | Cards, raised panels. |
| **Highlight** | `#F5C518` | **Reserved** — only ever marks *the note* (why something grabbed you). |
| **Sage** | `#3E6B3A` | Positive / "ready" status. |
| **Muted** | `#6E6A5C` | Secondary text. |

Supporting neutrals: borders `#E7E0D0` / `#DED7C6`, faint text `#8A8475` / `#B7AF9C`,
body text on paper `#3a352a` / `#56503F`.

**Status chips**

| Status | BG | Text |
|--------|----|----|
| inbox | `#ECE8DC` | `#6E6A5C` |
| brewing | `#F6E6C5` | `#9A6B16` |
| ready | `#DCEBD6` | `#3E6B3A` |
| producing | `#F8DBD2` | `#B23A1E` |
| shipped | `#D7E3E8` | `#34606F` |

> **Discipline:** Highlight gold never decorates UI chrome. It only ever appears as the
> text-highlight on a saved note. That restraint is what keeps the gold feeling like gold.

---

## 3. Type

| Role | Family | Weights | Notes |
|------|--------|---------|-------|
| Display | **Bricolage Grotesque** | 700 | Headlines, wordmark. Warm, slightly quirky. `letter-spacing: -.02em` to `-.03em`. |
| Text | **Hanken Grotesk** | 400 / 500 / 600 | Body, UI. Humanist, clean. |
| Mono | **JetBrains Mono** | 400 / 500 | The regex wink — labels, metadata, tags, timestamps. |

Load:
```html
<link href="https://fonts.googleapis.com/css2?family=Bricolage+Grotesque:opsz,wght@12..96,400;12..96,500;12..96,600;12..96,700;12..96,800&family=Hanken+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
```

**Scale** — Display 74/48/38 · Heading 24/22 · Body 21/16 · Meta 13 (mono). Line-height
`.98`–1.0 for display, 1.5 for body. Use `text-wrap: balance` on headlines,
`text-wrap: pretty` on body.

---

## 4. Components

- **Buttons** — Primary: Vermilion fill, white text, radius 10, soft vermilion shadow
  `0 6px 16px -8px rgba(230,74,46,.7)`. Secondary: 1px `#D8D0BE` border, transparent.
  Tertiary: text only, Muted.
- **Tags** — JetBrains Mono 12.5px, bg `#EDE8DB`, text `#56503F`, radius 7. Prefixed `#`.
- **Status chips** — pill (radius 999), 600 weight, colors per table above.
- **The note field** — the hero input. White, 1.5px Vermilion border, `0 0 0 4px rgba(230,74,46,.1)` focus ring. Always asks *"Why did this grab you?"* first; link & metadata are secondary and fill in on their own. **Saving never waits on a fetch.**
- **Idea card** — radius 16, 1px `#ECE5D6` border, shadow `0 14px 30px -22px rgba(26,24,19,.3)`. Media/gradient header + meta row (author · source, status chip) + title + highlighted note.

**Surfaces** — cards radius 16–18; app uses a faint Ink dot-grid:
`radial-gradient(circle at 1px 1px, rgba(26,24,19,.05) 1px, transparent 0)` at `24px`.

---

## 5. Screens

The brand sheet ([`Project Regex Brand.dc.html`](./Project%20Regex%20Brand.dc.html))
shows three reference layouts that define the boundary:

1. **Inbox + capture (light)** — nav rail, search, link-paste composer with the note field, two-up inbox list.
2. **Develop board (light)** — kanban of idea status columns; the "ready" card is Vermilion-outlined with a *Produce this →* CTA.
3. **Asset Library (dark)** — Ink surface (`#16150F`), gridded frames/clips/audio harvested from breakdowns.

Light is the default; dark (`#16150F` bg, `#211F18` panels, `#ECE7D9` text) is for media-dense, focus surfaces like the library.
