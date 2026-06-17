<script lang="ts">
  import { api, type Item, type ItemStatus, type SearchHit } from '$lib/api';

  let note = $state('');
  let url = $state('');
  let tags = $state('');
  let searchQuery = $state('');
  let statusFilter = $state<ItemStatus | ''>('');
  let items = $state<Item[]>([]);
  let searchHits = $state<SearchHit[]>([]);
  let message = $state('');
  let busy = $state(false);

  async function refresh() {
    busy = true;
    message = '';
    try {
      items = await api.listItems(statusFilter || undefined);
      if (searchQuery.trim()) {
        searchHits = await api.searchItems(searchQuery);
      } else {
        searchHits = [];
      }
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not load items.';
    } finally {
      busy = false;
    }
  }

  async function capture() {
    busy = true;
    message = '';
    try {
      const capturedOn = await api.defaultCapturedOn();
      const platform = url.trim() ? await api.detectPlatform(url.trim()) : 'manual';
      const result = await api.captureItem({
        url: url.trim() || null,
        platform,
        note: note.trim(),
        tags: tags
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean),
        captured_on: capturedOn
      });
      message = result.created ? 'Saved a new item.' : 'That link was already saved.';
      note = '';
      url = '';
      tags = '';
      await refresh();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not save the item.';
    } finally {
      busy = false;
    }
  }

  async function move(item: Item, status: ItemStatus) {
    busy = true;
    message = '';
    try {
      await api.updateItemStatus(item.id, status);
      await refresh();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not update status.';
    } finally {
      busy = false;
    }
  }

  $effect(() => {
    void refresh();
  });
</script>

<div class="mx-auto max-w-3xl space-y-6">
  <p class="text-sm text-text-muted">
    Temporary data layer screen — create, list, search, and move items while the real inbox is
    built.
  </p>

  {#if message}
    <p
      class="rounded-[var(--radius-control)] border border-border bg-bg-raised px-4 py-3 text-sm"
    >
      {message}
    </p>
  {/if}

  <section
    class="space-y-3 rounded-[var(--radius-card)] border border-border bg-bg-raised p-5"
  >
    <h2 class="text-sm font-medium">Quick capture</h2>
    <input
      class="w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
      placeholder="Link (optional)"
      bind:value={url}
    />
    <textarea
      class="min-h-24 w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
      placeholder="Why did this grab you?"
      bind:value={note}
    ></textarea>
    <input
      class="w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
      placeholder="Tags (comma separated)"
      bind:value={tags}
    />
    <button
      type="button"
      class="rounded-[var(--radius-control)] bg-accent px-4 py-2 text-sm font-medium text-accent-contrast disabled:opacity-50"
      disabled={busy || !note.trim()}
      onclick={capture}
    >
      Save
    </button>
  </section>

  <section
    class="space-y-3 rounded-[var(--radius-card)] border border-border bg-bg-raised p-5"
  >
    <div class="flex flex-wrap items-center gap-3">
      <input
        class="min-w-48 flex-1 rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
        placeholder="Search notes, titles, authors…"
        bind:value={searchQuery}
        oninput={() => refresh()}
      />
      <select
        class="rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
        bind:value={statusFilter}
        onchange={() => refresh()}
      >
        <option value="">All statuses</option>
        <option value="inbox">Inbox</option>
        <option value="brewing">Brewing</option>
        <option value="ready">Ready</option>
        <option value="producing">Producing</option>
        <option value="archived">Archived</option>
      </select>
    </div>

    {#if searchQuery.trim() && searchHits.length}
      <ul class="space-y-2">
        {#each searchHits as hit (hit.item.id)}
          <li
            class="rounded-[var(--radius-control)] border border-border bg-bg px-3 py-3 text-sm"
          >
            <p class="font-medium">
              {hit.item.title || hit.item.note.slice(0, 60) || 'Untitled'}
            </p>
            <p class="mt-1 text-text-muted">{hit.snippet}</p>
          </li>
        {/each}
      </ul>
    {:else}
      <ul class="space-y-2">
        {#each items as item (item.id)}
          <li
            class="flex items-start justify-between gap-4 rounded-[var(--radius-control)] border border-border bg-bg px-3 py-3 text-sm"
          >
            <div>
              <p class="font-medium">{item.title || item.note.slice(0, 60) || 'Untitled'}</p>
              <p class="mt-1 text-text-muted">{item.note}</p>
              <p class="mt-1 text-xs text-text-faint">{item.status}</p>
            </div>
            <div class="flex shrink-0 flex-col gap-1">
              <button
                type="button"
                class="rounded border border-border px-2 py-1 text-xs"
                onclick={() => move(item, 'brewing')}>Develop</button
              >
              <button
                type="button"
                class="rounded border border-border px-2 py-1 text-xs"
                onclick={() => move(item, 'archived')}>Archive</button
              >
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</div>
