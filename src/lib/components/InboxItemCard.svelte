<script lang="ts">
  import { convertFileSrc, invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import type { Item, ItemStatus, Tag } from '$lib/api';

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

<article class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-4">
  <div class="flex gap-4">
    {#if thumbSrc(item.thumbnail_path)}
      <img
        src={thumbSrc(item.thumbnail_path)}
        alt=""
        class="size-16 shrink-0 rounded-[var(--radius-control)] object-cover"
      />
    {:else}
      <div
        class="size-16 shrink-0 rounded-[var(--radius-control)] border border-border bg-bg-overlay"
        aria-hidden="true"
      ></div>
    {/if}

    <div class="min-w-0 flex-1">
      <div class="flex flex-wrap items-start justify-between gap-2">
        <div class="min-w-0">
          <h3 class="truncate text-sm font-medium">
            {item.title || item.note.slice(0, 72) || 'Untitled'}
          </h3>
          {#if item.author}
            <p class="mt-0.5 text-xs text-text-faint">{item.author}</p>
          {/if}
        </div>
        <span class="rounded-full border border-border px-2 py-0.5 text-xs text-text-muted">
          {item.status}
        </span>
      </div>

      {#if editing}
        <textarea
          class="mt-3 min-h-20 w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
          bind:value={draftNote}
        ></textarea>
        <div class="mt-2 flex gap-2">
          <button
            type="button"
            class="rounded-[var(--radius-control)] bg-accent px-3 py-1.5 text-xs font-medium text-accent-contrast"
            disabled={busy}
            onclick={saveNote}>Save note</button
          >
          <button
            type="button"
            class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs"
            onclick={() => (editing = false)}>Cancel</button
          >
        </div>
      {:else}
        <p class="mt-2 text-sm leading-relaxed text-text-muted">{item.note}</p>
      {/if}

      {#if tags.length}
        <div class="mt-3 flex flex-wrap gap-1.5">
          {#each tags as tag (tag.id)}
            <span class="rounded-full bg-accent-soft px-2 py-0.5 text-xs text-text"
              >{tag.name}</span
            >
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <div class="mt-4 flex flex-wrap gap-2 border-t border-border pt-3">
    {#if item.url}
      <button
        type="button"
        class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs text-text-muted hover:text-text"
        onclick={openOriginal}>Open original</button
      >
    {/if}
    <button
      type="button"
      class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs text-text-muted hover:text-text"
      onclick={startEdit}>Edit note</button
    >
    <button
      type="button"
      class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs text-text-muted hover:text-text"
      disabled={busy}
      onclick={() => move('brewing')}>Move to develop</button
    >
    <button
      type="button"
      class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs text-text-muted hover:text-text"
      disabled={busy}
      onclick={() => move('archived')}>Archive</button
    >
  </div>
</article>
