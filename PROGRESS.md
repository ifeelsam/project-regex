# Progress

Living checklist for the build. Tick items as phases land; record spike findings inline.

## Phases

- [x] **Phase 0 — Foundations**
  - [x] Working app name picked (`Regex`)
  - [x] Tauri 2 + SvelteKit + TS scaffold
  - [x] Tailwind v4 wired
  - [x] `.gitignore`, README skeleton, lint/format, `PROGRESS.md`
  - [x] SvelteKit `adapter-static`, `ssr = false`
  - [x] `cargo build` / `cargo clippy` clean; frontend build + lint + check pass
- [x] **Phase 1 — Shell, navigation, design system**
  - [x] Design tokens in `app.css` (dark default, light override)
  - [x] Left nav: Inbox, Develop, Projects, Library, Settings
  - [x] Empty states for each area
- [x] **Phase 2 — Local database & core data layer**
  - [x] SQLite schema + migrations + FTS5 triggers
  - [x] Rust commands for items, tags, projects, search
  - [x] Dedupe on capture and status transitions
  - [x] Unit tests (dedupe, search, status, migrations via in-memory pool)
  - [x] Temporary debug screen at `/debug/`
- [x] **Phase 3 — Capture inbox**
  - [x] Capture form with note-first flow and thought mode
  - [x] Background metadata + auto transcript enrichment
  - [x] Inbox list with instant search and tag filters
- [x] **Phase 4 — Develop**
  - [x] Brewing / ready board with status moves
  - [x] Idea detail with original note and thinking space
  - [x] Reference attachments via search
  - [x] Tag editing on ideas
- [x] **Phase 5 — Graduation gate → projects**
  - [x] Produce-this flow on ready ideas (title, brief, format)
  - [x] Graduation links idea + references to a project
  - [x] Projects list (active / shipped) and project detail view
- [x] **Phase 6 — Breakdown engine (desktop)**
  - [x] Sidecar wiring (yt-dlp, ffmpeg, whisper) + docs + dev stubs
  - [x] Job queue with cancel + progress events
  - [x] Pipeline: download, keyframes, clips, audio, whisper, structure
  - [x] Assets saved to DB; breakdown panel on project detail
- [ ] **Phase 7 — Asset library**
- [ ] **Phase 8 — Polish (desktop)**
- [ ] **Phase 9 — Mobile capture client**
- [ ] **Phase 10 — Packaging**

## Risk spikes

1. **Mobile share-target capture** — _pending._
2. **Sidecar bundling across macOS architectures** — Tauri resolves `externalBin` by target
   triple (`aarch64-apple-darwin` / `x86_64-apple-darwin`). Stub scripts auto-generate at
   build time; real binaries documented in `docs/media-tools.md`. Pipeline degrades with a
   clear message when tools are missing.
3. **Source acquisition & ToS** — yt-dlp runs with `--max-filesize 500M`; failures surface as
   readable errors. Personal-use checkbox required before any breakdown starts.

## Notes

- Stack is fixed: Tauri 2, SvelteKit 2 / Svelte 5 runes, Tailwind v4, Rust core, SQLite + FTS5.
- Business logic lives in the Rust core (`src-tauri/src`) so it is shared and testable.
