<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { api, type ProjectDetail, type ProjectFormat, type Asset } from '$lib/api';
  import BreakdownPanel from '$lib/components/BreakdownPanel.svelte';

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
  <a href="/projects/" class="text-sm text-text-muted hover:text-text">← Back to projects</a>

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
      <h2 class="text-xl font-semibold tracking-tight">{detail.project.title}</h2>
      <div class="flex flex-wrap gap-2 text-xs text-text-faint">
        <span>{formatLabel[detail.project.format]}</span>
        <span>·</span>
        <span>{detail.project.status}</span>
      </div>
      {#if detail.project.brief}
        <p class="text-sm leading-relaxed text-text-muted">{detail.project.brief}</p>
      {/if}
    </header>

    {#if projectId}
      <BreakdownPanel
        {projectId}
        itemId={detail.primary_item.item.id}
        oncomplete={load}
      />
    {/if}

    {#if assets.length}
      <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
        <h3 class="text-sm font-medium">Assets</h3>
        <p class="mt-1 text-sm text-text-muted">
          {assets.length} extracted asset{assets.length === 1 ? '' : 's'} from breakdown.
        </p>
        <ul class="mt-3 space-y-2" role="list">
          {#each assets.slice(0, 8) as asset (asset.id)}
            <li class="flex items-center justify-between gap-3 text-sm">
              <span>{asset.label || asset.type}</span>
              <span class="text-xs text-text-faint">{asset.type}</span>
            </li>
          {/each}
        </ul>
        {#if assets.length > 8}
          <a href="/library/" class="mt-3 inline-block text-xs text-accent hover:underline"
            >View all in library</a
          >
        {/if}
      </section>
    {/if}

    <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
      <h3 class="text-sm font-medium">Graduated idea</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">
        {detail.primary_item.item.note}
      </p>
      {#if detail.primary_item.item.develop_note}
        <p class="mt-3 text-sm leading-relaxed text-text">
          {detail.primary_item.item.develop_note}
        </p>
      {/if}
      {#if detail.primary_item.tags.length}
        <div class="mt-3 flex flex-wrap gap-1.5">
          {#each detail.primary_item.tags as tag (tag.id)}
            <span class="rounded-full bg-accent-soft px-2 py-0.5 text-xs">{tag.name}</span>
          {/each}
        </div>
      {/if}
      {#if detail.primary_item.item.url}
        <button
          type="button"
          class="mt-3 text-xs text-accent hover:underline"
          onclick={() => openOriginal(detail?.primary_item.item.url ?? null)}
          >Open original</button
        >
      {/if}
    </section>

    <section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
      <h3 class="text-sm font-medium">References</h3>
      <p class="mt-1 text-sm text-text-muted">
        Items that informed this project when it graduated.
      </p>

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
              {#if row.item.url}
                <button
                  type="button"
                  class="shrink-0 text-xs text-text-muted hover:text-text"
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
