<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api, type IdeaDetail, type ItemWithTags } from '$lib/api';
  import GraduateForm from '$lib/components/GraduateForm.svelte';

  let detail = $state<IdeaDetail | null>(null);
  let developNote = $state('');
  let tagDraft = $state('');
  let referenceQuery = $state('');
  let referenceHits = $state<ItemWithTags[]>([]);
  let message = $state('');
  let loading = $state(true);
  let savingNote = $state(false);
  let debounce: ReturnType<typeof setTimeout> | undefined;
  let referenceDebounce: ReturnType<typeof setTimeout> | undefined;

  const ideaId = $derived($page.params.id);

  function thumbSrc(path: string | null) {
    if (!path) return null;
    try {
      return convertFileSrc(path);
    } catch {
      return null;
    }
  }

  async function load() {
    if (!ideaId) return;
    loading = true;
    try {
      detail = await api.getIdeaDetail(ideaId);
      developNote = detail.item.develop_note;
      tagDraft = detail.tags.map((tag) => tag.name).join(', ');
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not load this idea.';
    } finally {
      loading = false;
    }
  }

  function scheduleSaveNote() {
    clearTimeout(debounce);
    debounce = setTimeout(() => {
      void saveDevelopNote();
    }, 500);
  }

  async function saveDevelopNote() {
    if (!ideaId || !detail) return;
    savingNote = true;
    try {
      const item = await api.updateDevelopNote(ideaId, developNote);
      detail = { ...detail, item };
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not save your thinking.';
    } finally {
      savingNote = false;
    }
  }

  async function saveTags() {
    if (!ideaId || !detail) return;
    try {
      const tags = await api.setItemTags(
        ideaId,
        tagDraft
          .split(',')
          .map((tag) => tag.trim())
          .filter(Boolean)
      );
      detail = { ...detail, tags };
      message = 'Tags updated.';
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not update tags.';
    }
  }

  async function setStatus(status: 'brewing' | 'ready' | 'archived') {
    if (!ideaId) return;
    try {
      await api.updateItemStatus(ideaId, status);
      message =
        status === 'ready'
          ? 'Marked ready to produce.'
          : status === 'brewing'
            ? 'Moved back to brewing.'
            : 'Archived.';
      await load();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not update status.';
    }
  }

  async function searchReferences() {
    const query = referenceQuery.trim();
    if (!query || !detail) {
      referenceHits = [];
      return;
    }
    try {
      const hits = await api.searchItems(query, 12);
      const attached = new Set(detail.references.map((row) => row.item.id));
      referenceHits = hits
        .filter((hit) => hit.item.id !== ideaId && !attached.has(hit.item.id))
        .map((hit) => ({ item: hit.item, tags: hit.tags }));
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not search references.';
    }
  }

  function scheduleReferenceSearch() {
    clearTimeout(referenceDebounce);
    referenceDebounce = setTimeout(() => {
      void searchReferences();
    }, 200);
  }

  async function attachReference(referenceId: string) {
    if (!ideaId) return;
    try {
      await api.addItemReference(ideaId, referenceId);
      referenceQuery = '';
      referenceHits = [];
      await load();
      message = 'Reference attached.';
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not attach reference.';
    }
  }

  async function detachReference(referenceId: string) {
    if (!ideaId) return;
    try {
      await api.removeItemReference(ideaId, referenceId);
      await load();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not remove reference.';
    }
  }

  async function openOriginal(url: string | null) {
    if (!url) return;
    await openUrl(url);
  }

  onMount(() => {
    void load();
  });
</script>

<div class="mx-auto max-w-3xl space-y-6">
  <a href="/develop/" class="text-sm text-text-muted hover:text-text">← Back to develop</a>

  {#if message}
    <p
      class="rounded-[var(--radius-control)] border border-border bg-bg-overlay px-4 py-3 text-sm text-text-muted"
      role="status"
    >
      {message}
    </p>
  {/if}

  {#if loading}
    <p class="text-sm text-text-muted">Loading…</p>
  {:else if detail}
    <header class="space-y-2">
      <h2 class="text-xl font-semibold tracking-tight">
        {detail.item.title || detail.item.note.slice(0, 80) || 'Untitled idea'}
      </h2>
      {#if detail.item.author}
        <p class="text-sm text-text-faint">{detail.item.author}</p>
      {/if}
      <div class="flex flex-wrap gap-2 pt-1">
        {#if detail.item.status !== 'ready'}
          <button
            type="button"
            class="rounded-[var(--radius-control)] bg-accent px-3 py-1.5 text-xs font-medium text-accent-contrast"
            onclick={() => setStatus('ready')}>Mark ready</button
          >
        {:else}
          <button
            type="button"
            class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs"
            onclick={() => setStatus('brewing')}>Back to brewing</button
          >
        {/if}
        <button
          type="button"
          class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs text-text-muted"
          onclick={() => setStatus('archived')}>Archive</button
        >
      </div>
    </header>

    {#if detail.item.status === 'ready' && ideaId}
      <GraduateForm
        ideaId={ideaId}
        defaultTitle={detail.item.title || detail.item.note.slice(0, 80)}
        defaultBrief={detail.item.develop_note}
        ondone={(text) => (message = text)}
      />
    {/if}

    <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
      <h3 class="text-sm font-medium">Original spark</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">{detail.item.note}</p>
      {#if detail.item.url}
        <button
          type="button"
          class="mt-3 text-xs text-accent hover:underline"
          onclick={() => openOriginal(detail?.item.url ?? null)}>Open original</button
        >
      {/if}
    </section>

    <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
      <div class="flex items-center justify-between gap-3">
        <h3 class="text-sm font-medium">Thinking space</h3>
        {#if savingNote}
          <span class="text-xs text-text-faint">Saving…</span>
        {/if}
      </div>
      <textarea
        class="mt-3 min-h-40 w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2.5 text-sm leading-relaxed"
        placeholder="Work the idea out — angles, structure, what you would actually make…"
        bind:value={developNote}
        oninput={scheduleSaveNote}
      ></textarea>
    </section>

    <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
      <h3 class="text-sm font-medium">Tags</h3>
      <div class="mt-3 flex gap-2">
        <input
          class="min-w-0 flex-1 rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
          placeholder="Comma separated"
          bind:value={tagDraft}
        />
        <button
          type="button"
          class="rounded-[var(--radius-control)] border border-border px-3 py-2 text-sm"
          onclick={saveTags}>Save</button
        >
      </div>
    </section>

    <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
      <h3 class="text-sm font-medium">References</h3>
      <p class="mt-1 text-sm text-text-muted">
        Attach other saved items that inform this idea.
      </p>

      <div class="mt-4 flex gap-2">
        <input
          class="min-w-0 flex-1 rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2 text-sm"
          placeholder="Search saved items to attach…"
          bind:value={referenceQuery}
          oninput={scheduleReferenceSearch}
        />
      </div>

      {#if referenceHits.length}
        <ul class="mt-3 space-y-2" role="list">
          {#each referenceHits as row (row.item.id)}
            <li
              class="flex items-center justify-between gap-3 rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2"
            >
              <div class="min-w-0">
                <p class="truncate text-sm">
                  {row.item.title || row.item.note.slice(0, 60) || 'Untitled'}
                </p>
                <p class="truncate text-xs text-text-faint">{row.item.note}</p>
              </div>
              <button
                type="button"
                class="shrink-0 text-xs text-accent hover:underline"
                onclick={() => attachReference(row.item.id)}>Attach</button
              >
            </li>
          {/each}
        </ul>
      {/if}

      {#if detail.references.length}
        <ul class="mt-4 space-y-3" role="list">
          {#each detail.references as row (row.item.id)}
            <li
              class="flex gap-3 rounded-[var(--radius-control)] border border-border bg-bg p-3"
            >
              {#if thumbSrc(row.item.thumbnail_path)}
                <img
                  src={thumbSrc(row.item.thumbnail_path)}
                  alt=""
                  class="size-12 shrink-0 rounded object-cover"
                />
              {/if}
              <div class="min-w-0 flex-1">
                <p class="text-sm font-medium">
                  {row.item.title || row.item.note.slice(0, 60) || 'Untitled'}
                </p>
                <p class="mt-1 line-clamp-2 text-xs text-text-muted">{row.item.note}</p>
              </div>
              <div class="flex shrink-0 flex-col gap-1">
                {#if row.item.url}
                  <button
                    type="button"
                    class="text-xs text-text-muted hover:text-text"
                    onclick={() => openOriginal(row.item.url)}>Open</button
                  >
                {/if}
                <button
                  type="button"
                  class="text-xs text-text-muted hover:text-text"
                  onclick={() => detachReference(row.item.id)}>Remove</button
                >
              </div>
            </li>
          {/each}
        </ul>
      {:else}
        <p class="mt-4 text-sm text-text-faint">No references attached yet.</p>
      {/if}
    </section>
  {/if}
</div>
