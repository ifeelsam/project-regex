<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { api, type Breakdown, type BreakdownProgress, type MediaToolsStatus } from '$lib/api';

  let {
    projectId,
    itemId,
    oncomplete
  }: {
    projectId: string;
    itemId: string;
    oncomplete?: () => void;
  } = $props();

  let breakdowns = $state<Breakdown[]>([]);
  let progress = $state<BreakdownProgress | null>(null);
  let tools = $state<MediaToolsStatus | null>(null);
  let confirmed = $state(false);
  let busy = $state(false);
  let message = $state('');

  const active = $derived(
    breakdowns.find((row) => row.status === 'queued' || row.status === 'running') ?? null
  );

  const latest = $derived(breakdowns[0] ?? null);

  async function refresh() {
    breakdowns = await api.listProjectBreakdowns(projectId);
    tools = await api.checkMediaTools();
  }

  async function start() {
    if (!confirmed || busy) return;
    busy = true;
    message = '';
    try {
      await api.startBreakdown(itemId, projectId, true);
      await refresh();
      message = 'Breakdown started.';
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not start breakdown.';
    } finally {
      busy = false;
    }
  }

  async function cancel() {
    if (!active) return;
    try {
      await api.cancelBreakdown(active.id);
      message = 'Cancelling…';
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not cancel.';
    }
  }

  onMount(() => {
    void refresh();

    const unlisten = listen<BreakdownProgress>('breakdown-progress', (event) => {
      if (event.payload.project_id !== projectId) return;
      progress = event.payload;
      if (event.payload.step === 'done' || event.payload.step === 'failed') {
        void refresh();
        oncomplete?.();
      }
    });

    return () => {
      void unlisten.then((off) => off());
    };
  });
</script>

<section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
  <h3 class="text-sm font-medium">Breakdown</h3>
  <p class="mt-1 text-sm text-text-muted">
    Extract frames, clips, audio, a transcript, and structure — only after you commit to
    producing.
  </p>

  {#if tools && (!tools.yt_dlp || !tools.ffmpeg)}
    <p class="mt-3 text-sm text-warn">
      Media tools missing. Install yt-dlp and ffmpeg per docs/media-tools.md before running a
      full breakdown.
    </p>
  {/if}

  <label class="mt-4 flex items-start gap-2 text-sm text-text-muted">
    <input type="checkbox" class="mt-0.5" bind:checked={confirmed} />
    <span
      >I confirm this is for personal use — content I have the right to reference, not
      protected or DRM-locked material.</span
    >
  </label>

  <div class="mt-4 flex flex-wrap gap-2">
    <button
      type="button"
      class="rounded-[var(--radius-control)] bg-accent px-4 py-2 text-sm font-medium text-accent-contrast disabled:opacity-40"
      disabled={busy || !confirmed || !!active}
      onclick={start}
    >
      {active ? 'Running…' : 'Run breakdown'}
    </button>
    {#if active}
      <button
        type="button"
        class="rounded-[var(--radius-control)] border border-border px-4 py-2 text-sm"
        onclick={cancel}>Cancel</button
      >
    {/if}
  </div>

  {#if progress && (progress.step !== 'done' || active)}
    <div class="mt-4 space-y-2">
      <div class="flex items-center justify-between text-xs text-text-faint">
        <span class="capitalize">{progress.step}</span>
        <span>{Math.round(progress.progress * 100)}%</span>
      </div>
      <div class="h-1.5 overflow-hidden rounded-full bg-bg">
        <div
          class="h-full bg-accent transition-all"
          style="width: {Math.max(4, progress.progress * 100)}%"
        ></div>
      </div>
      <p class="text-sm text-text-muted">{progress.message}</p>
    </div>
  {/if}

  {#if latest?.status === 'done'}
    <p class="mt-3 text-sm text-accent">Breakdown complete — assets are in the library.</p>
  {:else if latest?.status === 'failed' && latest.error}
    <p class="mt-3 text-sm text-danger">{latest.error}</p>
  {/if}

  {#if message}
    <p class="mt-3 text-sm text-text-muted" role="status">{message}</p>
  {/if}
</section>
