# Graph Report - project-regex  (2026-06-18)

## Corpus Check
- 62 files · ~23,471 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 277 nodes · 370 edges · 29 communities (22 shown, 7 thin omitted)
- Extraction: 86% EXTRACTED · 14% INFERRED · 0% AMBIGUOUS · INFERRED: 50 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0340f925`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]
- [[_COMMUNITY_Community 9|Community 9]]
- [[_COMMUNITY_Community 10|Community 10]]
- [[_COMMUNITY_Community 11|Community 11]]
- [[_COMMUNITY_Community 12|Community 12]]
- [[_COMMUNITY_Community 13|Community 13]]
- [[_COMMUNITY_Community 14|Community 14]]
- [[_COMMUNITY_Community 15|Community 15]]
- [[_COMMUNITY_Community 17|Community 17]]
- [[_COMMUNITY_Community 18|Community 18]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]

## God Nodes (most connected - your core abstractions)
1. `capture()` - 12 edges
2. `run_inner()` - 10 edges
3. `Regex` - 10 edges
4. `get_item_tags()` - 8 edges
5. `set_status()` - 8 edges
6. `connect_memory()` - 8 edges
7. `graduate_item()` - 8 edges
8. `graduates_ready_idea_with_references()` - 7 edges
9. `Media tools for the breakdown engine` - 7 edges
10. `run_enrichment()` - 6 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `Build`  [INFERRED]
  src-tauri/build.rs → README.md
- `run()` --calls--> `connect()`  [INFERRED]
  src-tauri/src/lib.rs → src-tauri/src/db/pool.rs
- `run_enrichment()` --calls--> `fetch()`  [INFERRED]
  src-tauri/src/enrich.rs → src-tauri/src/metadata/mod.rs
- `run_enrichment()` --calls--> `update_metadata()`  [INFERRED]
  src-tauri/src/enrich.rs → src-tauri/src/db/items.rs
- `capture_item()` --calls--> `capture()`  [INFERRED]
  src-tauri/src/commands.rs → src-tauri/src/db/items.rs

## Communities (29 total, 7 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.09
Nodes (37): capture(), dedupes_on_url(), find_by_url(), get(), ItemWithTags, list(), list_with_tags(), set_status() (+29 more)

### Community 1 - "Community 1"
Cohesion: 0.11
Nodes (17): Build, code:bash (npm install), code:bash (npm run tauri dev), code:bash (npm run dev), code:bash (npm run tauri build), code:bash (npm run lint          # prettier + eslint), Develop, License / use (+9 more)

### Community 2 - "Community 2"
Cohesion: 0.13
Nodes (18): ready, @tauri-apps/api/core, @tauri-apps/api/event, $app/navigation, @tauri-apps/plugin-opener, shipped, $lib/components/BreakdownPanel.svelte, $lib/components/CaptureForm.svelte (+10 more)

### Community 3 - "Community 3"
Cohesion: 0.09
Nodes (23): api, Asset, AssetType, Breakdown, BreakdownProgress, BreakdownStatus, call(), CapturedOn (+15 more)

### Community 4 - "Community 4"
Cohesion: 0.17
Nodes (10): CapturedOn, CaptureItemInput, CaptureItemResult, Item, ItemStatus, Platform, Project, ProjectFormat (+2 more)

### Community 5 - "Community 5"
Cohesion: 0.1
Nodes (8): get_detail(), IdeaDetail, list_develop(), remove(), get_idea_detail(), list_develop_items(), list_project_summaries(), remove_item_reference()

### Community 6 - "Community 6"
Cohesion: 0.47
Nodes (5): excerpt(), RawHit, search(), search_finds_note_text(), search_items()

### Community 7 - "Community 7"
Cohesion: 0.27
Nodes (8): download_thumbnail(), enrich_item(), ItemEnrichedEvent, run_enrichment(), fetch_auto_transcript(), fetch_youtube_captions(), parses_youtube_watch_url(), youtube_id()

### Community 8 - "Community 8"
Cohesion: 0.57
Nodes (6): fetch(), fetch_oembed(), fetch_open_graph(), fetch_youtube_oembed(), LinkMetadata, meta_content()

### Community 9 - "Community 9"
Cohesion: 0.13
Nodes (16): BreakdownProgress, check_cancelled(), clear_cancel(), emit_progress(), find_source_file(), register_cancel(), run(), run_inner() (+8 more)

### Community 11 - "Community 11"
Cohesion: 0.47
Nodes (4): applyTheme(), setTheme(), Theme, toggleTheme()

### Community 12 - "Community 12"
Cohesion: 0.4
Nodes (4): Notes, Phases, Progress, Risk spikes

### Community 13 - "Community 13"
Cohesion: 0.4
Nodes (3): AppState, run(), main()

### Community 24 - "Community 24"
Cohesion: 0.12
Nodes (16): code:text (src-tauri/binaries/), code:text (yt-dlp-x86_64-apple-darwin), code:bash (curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/dow), code:bash (git clone https://github.com/ggml-org/whisper.cpp), code:bash (mkdir -p ~/Library/Application\ Support/app.regex.desktop/mo), Development stubs, ffmpeg, macOS (Apple Silicon) (+8 more)

### Community 25 - "Community 25"
Cohesion: 0.25
Nodes (8): Asset, AssetType, Breakdown, BreakdownStatus, create(), get(), now_iso(), set_status()

### Community 26 - "Community 26"
Cohesion: 0.25
Nodes (3): cancel_job(), cancel_breakdown(), MediaToolsStatus

## Knowledge Gaps
- **76 isolated node(s):** `config`, `MediaToolsStatus`, `AppState`, `ItemEnrichedEvent`, `BreakdownProgress` (+71 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **7 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `capture()` connect `Community 0` to `Community 6`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Why does `run_enrichment()` connect `Community 7` to `Community 8`, `Community 0`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Why does `update_metadata()` connect `Community 0` to `Community 7`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `capture()` (e.g. with `capture_item()` and `attaches_and_lists_references()`) actually correct?**
  _`capture()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `run_inner()` (e.g. with `insert()` and `insert_whisper()`) actually correct?**
  _`run_inner()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 7 inferred relationships involving `get_item_tags()` (e.g. with `list_for_idea()` and `list_with_tags()`) actually correct?**
  _`get_item_tags()` has 7 INFERRED edges - model-reasoned connections that need verification._
- **What connects `config`, `MediaToolsStatus`, `AppState` to the rest of the system?**
  _76 weakly-connected nodes found - possible documentation gaps or missing edges._