<script lang="ts">
  import { api, type Item, type ItemStatus, type SearchHit } from '$lib/api';
  import { statusChipClass } from '$lib/status';

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
    <p class="banner">{message}</p>
  {/if}

  <section class="card-flat space-y-3 p-5">
    <h2 class="text-sm font-semibold">Quick capture</h2>
    <input class="field" placeholder="Link (optional)" bind:value={url} />
    <textarea
      class="field field-note min-h-24"
      placeholder="Why did this grab you?"
      bind:value={note}
    ></textarea>
    <input
      class="field font-mono text-sm"
      placeholder="Tags (comma separated)"
      bind:value={tags}
    />
    <button
      type="button"
      class="btn btn-primary"
      disabled={busy || !note.trim()}
      onclick={capture}
    >
      Save
    </button>
  </section>

  <section class="card-flat space-y-3 p-5">
    <div class="flex flex-wrap items-center gap-3">
      <label class="search-field min-w-48 flex-1">
        <span class="font-mono text-[0.8125rem] text-text-faint">/</span>
        <input
          placeholder="Search notes, titles, authors…"
          bind:value={searchQuery}
          oninput={() => refresh()}
        />
      </label>
      <select class="field w-auto" bind:value={statusFilter} onchange={() => refresh()}>
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
            class="rounded-[var(--radius-control)] border border-border bg-white px-3 py-3 text-sm dark:bg-bg-overlay"
          >
            <p class="font-semibold">
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
            class="flex items-start justify-between gap-4 rounded-[var(--radius-control)] border border-border bg-white px-3 py-3 text-sm dark:bg-bg-overlay"
          >
            <div>
              <p class="font-semibold">{item.title || item.note.slice(0, 60) || 'Untitled'}</p>
              <p class="mt-1 text-text-muted">{item.note}</p>
              <span class={['status-chip mt-2', statusChipClass(item.status)]}
                >{item.status}</span
              >
            </div>
            <div class="flex shrink-0 flex-col gap-1">
              <button
                type="button"
                class="btn btn-secondary btn-sm"
                onclick={() => move(item, 'brewing')}
              >
                Develop
              </button>
              <button
                type="button"
                class="btn btn-tertiary btn-sm"
                onclick={() => move(item, 'archived')}
              >
                Archive
              </button>
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</div>
