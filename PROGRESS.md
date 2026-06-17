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
- [ ] **Phase 3 — Capture inbox**
- [ ] **Phase 4 — Develop**
- [ ] **Phase 5 — Graduation gate → projects**
- [ ] **Phase 6 — Breakdown engine (desktop)**
- [ ] **Phase 7 — Asset library**
- [ ] **Phase 8 — Polish (desktop)**
- [ ] **Phase 9 — Mobile capture client**
- [ ] **Phase 10 — Packaging**

## Risk spikes

1. **Mobile share-target capture** — _pending._
2. **Sidecar bundling across macOS architectures** — _pending._
3. **Source acquisition & ToS** — _pending._

## Notes

- Stack is fixed: Tauri 2, SvelteKit 2 / Svelte 5 runes, Tailwind v4, Rust core, SQLite + FTS5.
- Business logic lives in the Rust core (`src-tauri/src`) so it is shared and testable.
