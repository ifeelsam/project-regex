<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api, type ProjectDetail, type ProjectFormat, type Asset } from '$lib/api';
  import BreakdownPanel from '$lib/components/BreakdownPanel.svelte';
  import { statusChipClass } from '$lib/status';

  let detail = $state<ProjectDetail | null>(null);
  let assets = $state<Asset[]>([]);
  let loading = $state(true);
  let message = $state('');

  const projectId = $derived($page.params.id);

  const formatLabel: Record<ProjectFormat, string> = {
    video: 'Video',
    article: 'Article',
    other: 'Other'
  };

  function thumbSrc(path: string | null) {
    if (!path) return null;
    try {
      return convertFileSrc(path);
    } catch {
      return null;
    }
  }

  async function load() {
    if (!projectId) return;
    loading = true;
    try {
      detail = await api.getProjectDetail(projectId);
      assets = await api.listProjectAssets(projectId);
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not load this project.';
    } finally {
      loading = false;
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
  <a href="/projects/" class="btn btn-tertiary px-0 text-sm">← Back to projects</a>

  {#if message}
    <p class="banner" role="status">{message}</p>
  {/if}

  {#if loading}
    <p class="text-sm text-text-muted">Loading…</p>
  {:else if detail}
    <header class="space-y-2">
      <div class="flex flex-wrap items-start justify-between gap-3">
        <h2 class="heading-display text-2xl">{detail.project.title}</h2>
        <span class={['status-chip', statusChipClass(detail.project.status)]}>
          {detail.project.status}
        </span>
      </div>
      <div class="flex flex-wrap gap-2 font-mono text-xs text-text-faint">
        <span>{formatLabel[detail.project.format]}</span>
      </div>
      {#if detail.project.brief}
        <p class="text-sm leading-relaxed text-text-muted text-pretty">
          {detail.project.brief}
        </p>
      {/if}
    </header>

    {#if projectId}
      <BreakdownPanel {projectId} itemId={detail.primary_item.item.id} oncomplete={load} />
    {/if}

    {#if assets.length}
      <section class="card-flat p-5">
        <h3 class="text-sm font-semibold">Assets</h3>
        <p class="mt-1 text-sm text-text-muted">
          {assets.length} extracted asset{assets.length === 1 ? '' : 's'} from breakdown.
        </p>
        <ul class="mt-3 space-y-2" role="list">
          {#each assets.slice(0, 8) as asset (asset.id)}
            <li class="flex items-center justify-between gap-3 text-sm">
              <span>{asset.label || asset.type}</span>
              <span class="font-mono text-xs text-text-faint">{asset.type}</span>
            </li>
          {/each}
        </ul>
        {#if assets.length > 8}
          <a href="/library/" class="btn btn-tertiary btn-sm mt-3 px-0">View all in library</a>
        {/if}
      </section>
    {/if}

    <section class="card-flat p-5">
      <h3 class="text-sm font-semibold">Graduated idea</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">
        <span class="note-highlight">{detail.primary_item.item.note}</span>
      </p>
      {#if detail.primary_item.item.develop_note}
        <p class="mt-3 text-sm leading-relaxed text-text-body">
          {detail.primary_item.item.develop_note}
        </p>
      {/if}
      {#if detail.primary_item.tags.length}
        <div class="mt-3 flex flex-wrap gap-1.5">
          {#each detail.primary_item.tags as tag (tag.id)}
            <span class="tag">#{tag.name}</span>
          {/each}
        </div>
      {/if}
      {#if detail.primary_item.item.url}
        <button
          type="button"
          class="btn btn-tertiary btn-sm mt-3 px-0"
          onclick={() => openOriginal(detail?.primary_item.item.url ?? null)}
          >Open original</button
        >
      {/if}
    </section>

    <section class="card-flat p-5">
      <h3 class="text-sm font-semibold">References</h3>
      <p class="mt-1 text-sm text-text-muted">
        Items that informed this project when it graduated.
      </p>

      {#if detail.references.length}
        <ul class="mt-4 space-y-3" role="list">
          {#each detail.references as row (row.item.id)}
            <li
              class="flex gap-3 rounded-[var(--radius-control)] border border-border bg-white p-3 dark:bg-bg-overlay"
            >
              {#if thumbSrc(row.item.thumbnail_path)}
                <img
                  src={thumbSrc(row.item.thumbnail_path)}
                  alt=""
                  class="size-12 shrink-0 rounded-lg object-cover"
                />
              {/if}
              <div class="min-w-0 flex-1">
                <p class="text-sm font-semibold">
                  {row.item.title || row.item.note.slice(0, 60) || 'Untitled'}
                </p>
                <p class="mt-1 line-clamp-2 text-xs text-text-muted">{row.item.note}</p>
              </div>
              {#if row.item.url}
                <button
                  type="button"
                  class="btn btn-tertiary btn-sm shrink-0"
                  onclick={() => openOriginal(row.item.url)}>Open</button
                >
              {/if}
            </li>
          {/each}
        </ul>
      {:else}
        <p class="mt-4 text-sm text-text-faint">No references were attached.</p>
      {/if}
    </section>
  {/if}
</div>
