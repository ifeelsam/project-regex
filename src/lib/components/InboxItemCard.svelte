<script lang="ts">
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import type { Item, ItemStatus, Tag } from '$lib/api';
  import { statusChipClass } from '$lib/status';

  let {
    row,
    onchange
  }: {
    row: { item: Item; tags: Tag[] };
    onchange?: () => void;
  } = $props();

  const item = $derived(row.item);
  const tags = $derived(row.tags);

  let editing = $state(false);
  let draftNote = $state('');
  let busy = $state(false);

  function thumbSrc(path: string | null) {
    if (!path) return null;
    try {
      return convertFileSrc(path);
    } catch {
      return null;
    }
  }

  function thumbGradient(platform: string) {
    if (platform === 'manual') return 'from-[#F3EEE1] to-[#E8E2D4]';
    if (platform.includes('twitter') || platform === 'x') return 'from-[#3E6B3A] to-[#2c5128]';
    return 'from-[#E64A2E] to-[#C13A1F]';
  }

  async function openOriginal() {
    if (!item.url) return;
    await openUrl(item.url);
  }

  async function saveNote() {
    busy = true;
    try {
      await invoke('update_item_note', { id: item.id, note: draftNote });
      editing = false;
      onchange?.();
    } finally {
      busy = false;
    }
  }

  async function move(status: ItemStatus) {
    busy = true;
    try {
      await invoke('update_item_status', { id: item.id, status });
      onchange?.();
    } finally {
      busy = false;
    }
  }

  function startEdit() {
    draftNote = item.note;
    editing = true;
  }
</script>

<article class="card-flat overflow-hidden bg-white dark:bg-bg-raised">
  <div class="flex gap-3 p-3.5">
    {#if thumbSrc(item.thumbnail_path)}
      <img
        src={thumbSrc(item.thumbnail_path)}
        alt=""
        class="size-[3.375rem] shrink-0 rounded-lg object-cover"
      />
    {:else}
      <div
        class={[
          'flex size-[3.375rem] shrink-0 items-center justify-center rounded-lg bg-gradient-to-br',
          thumbGradient(item.platform)
        ]}
        aria-hidden="true"
      >
        {#if item.platform === 'manual'}
          <span class="font-mono text-lg text-[#CFC7B4]">.*</span>
        {/if}
      </div>
    {/if}

    <div class="min-w-0 flex-1">
      <div class="flex flex-wrap items-start justify-between gap-2">
        <div class="min-w-0">
          <h3 class="truncate text-[0.84375rem] font-semibold leading-snug">
            {item.title || item.note.slice(0, 72) || 'Untitled'}
          </h3>
          {#if item.author}
            <p class="mt-0.5 font-mono text-[0.6875rem] text-text-faint">
              {item.author}{#if item.platform !== 'manual'}
                · {item.platform}{/if}
            </p>
          {:else if item.platform === 'manual'}
            <p class="mt-0.5 font-mono text-[0.6875rem] text-text-faint">pure thought</p>
          {/if}
        </div>
        <span class={['status-chip shrink-0', statusChipClass(item.status)]}>
          {item.status}
        </span>
      </div>

      {#if editing}
        <textarea class="field mt-3 min-h-20 text-sm" bind:value={draftNote}></textarea>
        <div class="mt-2 flex gap-2">
          <button
            type="button"
            class="btn btn-primary btn-sm"
            disabled={busy}
            onclick={saveNote}
          >
            Save note
          </button>
          <button
            type="button"
            class="btn btn-secondary btn-sm"
            onclick={() => (editing = false)}>Cancel</button
          >
        </div>
      {:else}
        <p class="note-emphasis mt-2 line-clamp-2 text-xs leading-relaxed">
          <span class="note-highlight">{item.note}</span>
        </p>
      {/if}

      {#if tags.length}
        <div class="mt-2.5 flex flex-wrap gap-1.5">
          {#each tags as tag (tag.id)}
            <span class="tag">#{tag.name}</span>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <div class="flex flex-wrap gap-2 border-t border-border px-3.5 py-2.5">
    {#if item.url}
      <button type="button" class="btn btn-tertiary btn-sm" onclick={openOriginal}>
        Open original
      </button>
    {/if}
    <button type="button" class="btn btn-tertiary btn-sm" onclick={startEdit}>
      Edit note
    </button>
    <button
      type="button"
      class="btn btn-tertiary btn-sm"
      disabled={busy}
      onclick={() => move('brewing')}>Move to develop</button
    >
    <button
      type="button"
      class="btn btn-tertiary btn-sm"
      disabled={busy}
      onclick={() => move('archived')}>Archive</button
    >
  </div>
</article>
