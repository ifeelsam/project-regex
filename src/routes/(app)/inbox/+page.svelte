<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import CaptureForm from '$lib/components/CaptureForm.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import InboxItemCard from '$lib/components/InboxItemCard.svelte';
  import { api, type Item, type SearchHit, type Tag } from '$lib/api';

  type InboxRow = { item: Item; tags: Tag[] };

  let rows = $state<InboxRow[]>([]);
  let searchHits = $state<SearchHit[]>([]);
  let allTags = $state<Tag[]>([]);
  let query = $state('');
  let activeTag = $state('');
  let message = $state('');
  let loading = $state(true);
  let debounce: ReturnType<typeof setTimeout> | undefined;

  const showingSearch = $derived(query.trim().length > 0);

  const visibleRows = $derived.by(() => {
    const source: InboxRow[] = showingSearch
      ? searchHits.map((hit) => ({ item: hit.item, tags: hit.tags }))
      : rows;

    if (!activeTag) return source;
    return source.filter((row) => row.tags.some((tag) => tag.name === activeTag));
  });

  async function refresh() {
    loading = true;
    try {
      rows = await api.listInboxItems('inbox');
      allTags = await api.listTags();

      if (query.trim()) {
        searchHits = await api.searchItems(query.trim());
      } else {
        searchHits = [];
      }
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not load the inbox.';
    } finally {
      loading = false;
    }
  }

  function scheduleSearch() {
    clearTimeout(debounce);
    debounce = setTimeout(() => {
      void refresh();
    }, 180);
  }

  onMount(() => {
    void refresh();

    let unlisten: (() => void) | undefined;
    void listen<{ item: Item }>('item-enriched', () => {
      void refresh();
    }).then((off) => {
      unlisten = off;
    });

    return () => {
      unlisten?.();
    };
  });
</script>

<div class="mx-auto flex max-w-4xl flex-col gap-6">
  <CaptureForm
    onsaved={(text) => {
      message = text;
      void refresh();
    }}
  />

  {#if message}
    <p class="banner" role="status">{message}</p>
  {/if}

  <section class="space-y-4">
    <label class="search-field block">
      <span class="font-mono text-[0.8125rem] text-text-faint">/</span>
      <input
        placeholder="Search notes, titles, authors, transcripts…"
        bind:value={query}
        oninput={scheduleSearch}
      />
    </label>

    {#if allTags.length}
      <div class="flex flex-wrap gap-2">
        <button
          type="button"
          class={['tag-filter', !activeTag && 'tag-filter-active']}
          onclick={() => (activeTag = '')}
        >
          All
        </button>
        {#each allTags as tag (tag.id)}
          <button
            type="button"
            class={['tag-filter', activeTag === tag.name && 'tag-filter-active']}
            onclick={() => (activeTag = tag.name)}
          >
            #{tag.name}
          </button>
        {/each}
      </div>
    {/if}

    {#if loading}
      <p class="text-sm text-text-muted">Loading…</p>
    {:else if visibleRows.length === 0}
      <EmptyState
        title={showingSearch ? 'No matches' : 'Nothing in the inbox yet'}
        description={showingSearch
          ? 'Try another word from a note, title, author, or transcript.'
          : 'Capture a link or a quick thought above. Your note is the gold.'}
      />
    {:else}
      <ul class="grid gap-3 sm:grid-cols-2" role="list">
        {#each visibleRows as row (row.item.id)}
          <li>
            <InboxItemCard {row} onchange={refresh} />
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</div>
