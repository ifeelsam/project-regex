# Regex

A local-first app that turns scattered inspiration into finished creative work (videos,
articles). Ideas that would otherwise die in chat apps get captured, developed, graduated
into projects, broken down into reusable assets, and shipped — all on your machine.

**The pipeline:** sources → capture inbox → develop → graduation gate → breakdown →
asset library → finished piece.

## Principles

- **The note is the gold.** Capture _why_ something grabbed you, not just the media.
- **Capture light, break down heavy.** Saving is instant; the expensive media pipeline only
  runs after an idea is committed to production.
- **Search is the killer feature.** Full-text search across notes, titles, authors,
  transcripts, and asset labels is instant and central.
- **Personal use only.** Regex never bypasses platform protections or DRM.

## Stack

- **Shell:** Tauri 2 (native OS webview + Rust backend)
- **Frontend:** SvelteKit 2 + Svelte 5 (runes) + TypeScript (strict) + Tailwind v4, SPA mode
- **Core:** Rust — all data access, the media pipeline, and platform integration
- **Database:** SQLite (`sqlx`) with FTS5 for search
- **Media pipeline (desktop only):** `yt-dlp`, `ffmpeg`, `whisper.cpp` sidecars

Desktop (macOS) is the full app including the breakdown engine. Mobile is a capture + develop
client over the same Rust core.

## Prerequisites

- Node.js 20+ and npm
- Rust (stable) via [rustup](https://rustup.rs)
- Platform deps for Tauri 2 — see the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

## Setup

```bash
npm install
```

## Develop

```bash
npm run tauri dev
```

This launches the SvelteKit dev server and opens the native window.

To run only the frontend in a browser (no native APIs):

```bash
npm run dev
```

## Build

```bash
npm run tauri build
```

Produces a macOS app bundle in `src-tauri/target/release/bundle`.

## Quality

```bash
npm run lint          # prettier + eslint
npm run check         # svelte-check / typescript
cd src-tauri && cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

## Media tools (desktop breakdown)

The breakdown engine shells out to `yt-dlp`, `ffmpeg`, and `whisper.cpp`. These are **not**
committed to the repo. Fetch them into `src-tauri/binaries/` named with Tauri's target-triple
suffix (e.g. `ffmpeg-aarch64-apple-darwin`). See [`docs/media-tools.md`](docs/media-tools.md)
for the exact steps. The app degrades gracefully and tells you what is missing if a tool
isn't present.

## License / use

Personal use only. Regex processes content you have the right to use (your own work, or
public content kept for personal reference). It does not circumvent any platform's
protections or DRM.
