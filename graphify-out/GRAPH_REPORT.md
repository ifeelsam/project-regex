# Graph Report - project-regex  (2026-06-18)

## Corpus Check
- 40 files · ~16,544 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 171 nodes · 201 edges · 24 communities (18 shown, 6 thin omitted)
- Extraction: 89% EXTRACTED · 11% INFERRED · 0% AMBIGUOUS · INFERRED: 23 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c75cdd91`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
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

## God Nodes (most connected - your core abstractions)
1. `Regex` - 10 edges
2. `capture()` - 9 edges
3. `run_enrichment()` - 6 edges
4. `set_status()` - 6 edges
5. `get()` - 5 edges
6. `connect_memory()` - 5 edges
7. `create()` - 5 edges
8. `search()` - 5 edges
9. `fetch()` - 5 edges
10. `fetch_open_graph()` - 5 edges

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

## Communities (24 total, 6 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.14
Nodes (18): capture(), dedupes_on_url(), find_by_url(), get(), ItemWithTags, list(), list_with_tags(), set_status() (+10 more)

### Community 1 - "Community 1"
Cohesion: 0.11
Nodes (16): Build, code:bash (npm install), code:bash (npm run tauri dev), code:bash (npm run dev), code:bash (npm run tauri build), code:bash (npm run lint          # prettier + eslint), Develop, License / use (+8 more)

### Community 2 - "Community 2"
Cohesion: 0.14
Nodes (11): @tauri-apps/api/core, @tauri-apps/api/event, @tauri-apps/plugin-opener, $lib/components/CaptureForm.svelte, $lib/components/EmptyState.svelte, $lib/components/InboxItemCard.svelte, $lib/components/Shell.svelte, ./Sidebar.svelte (+3 more)

### Community 3 - "Community 3"
Cohesion: 0.14
Nodes (14): api, call(), CapturedOn, CaptureItemInput, CaptureItemResult, humanError(), Item, ItemStatus (+6 more)

### Community 4 - "Community 4"
Cohesion: 0.17
Nodes (10): CapturedOn, CaptureItemInput, CaptureItemResult, Item, ItemStatus, Platform, Project, ProjectFormat (+2 more)

### Community 6 - "Community 6"
Cohesion: 0.24
Nodes (8): excerpt(), RawHit, search(), search_finds_note_text(), get_item_tags(), get_or_create(), set_item_tags(), search_items()

### Community 7 - "Community 7"
Cohesion: 0.27
Nodes (8): download_thumbnail(), enrich_item(), ItemEnrichedEvent, run_enrichment(), fetch_auto_transcript(), fetch_youtube_captions(), parses_youtube_watch_url(), youtube_id()

### Community 8 - "Community 8"
Cohesion: 0.57
Nodes (6): fetch(), fetch_oembed(), fetch_open_graph(), fetch_youtube_oembed(), LinkMetadata, meta_content()

### Community 9 - "Community 9"
Cohesion: 0.47
Nodes (4): create(), get(), graduate_item(), create_project()

### Community 11 - "Community 11"
Cohesion: 0.47
Nodes (4): applyTheme(), setTheme(), Theme, toggleTheme()

### Community 12 - "Community 12"
Cohesion: 0.4
Nodes (4): Notes, Phases, Progress, Risk spikes

### Community 13 - "Community 13"
Cohesion: 0.4
Nodes (3): AppState, run(), main()

## Knowledge Gaps
- **45 isolated node(s):** `config`, `AppState`, `ItemEnrichedEvent`, `Platform`, `CapturedOn` (+40 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `capture()` connect `Community 0` to `Community 6`?**
  _High betweenness centrality (0.104) - this node is a cross-community bridge._
- **Why does `run_enrichment()` connect `Community 7` to `Community 8`, `Community 0`?**
  _High betweenness centrality (0.093) - this node is a cross-community bridge._
- **Why does `update_metadata()` connect `Community 0` to `Community 7`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `capture()` (e.g. with `capture_item()` and `get_item_tags()`) actually correct?**
  _`capture()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `run_enrichment()` (e.g. with `fetch()` and `update_metadata()`) actually correct?**
  _`run_enrichment()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `set_status()` (e.g. with `update_item_status()` and `assert_transition()`) actually correct?**
  _`set_status()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `config`, `AppState`, `ItemEnrichedEvent` to the rest of the system?**
  _45 weakly-connected nodes found - possible documentation gaps or missing edges._