<script lang="ts">
  import DevelopIdeaCard from '$lib/components/DevelopIdeaCard.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import type { ItemWithTags } from '$lib/api';

  let {
    rows,
    onmove
  }: {
    rows: ItemWithTags[];
    onmove?: (id: string, status: 'brewing' | 'ready') => void;
  } = $props();

  const brewing = $derived(rows.filter((row) => row.item.status === 'brewing'));
  const ready = $derived(rows.filter((row) => row.item.status === 'ready'));
</script>

{#if rows.length === 0}
  <EmptyState
    title="No ideas brewing"
    description="Move sparks from your inbox here when you want to think them through. Mark an idea ready when you are prepared to produce it."
  />
{:else}
  <div class="grid gap-6 lg:grid-cols-2">
    <section class="space-y-3">
      <header class="flex items-center gap-2">
        <span class="status-chip status-brewing">brewing</span>
        <span class="font-mono text-[0.6875rem] text-text-faint">{brewing.length}</span>
      </header>
      {#if brewing.length === 0}
        <p
          class="rounded-[var(--radius-card)] border border-dashed border-border px-4 py-8 text-center text-sm text-text-faint"
        >
          Nothing brewing yet.
        </p>
      {:else}
        <ul class="space-y-2.5" role="list">
          {#each brewing as row (row.item.id)}
            <li class="space-y-2">
              <DevelopIdeaCard {row} />
              <button
                type="button"
                class="btn btn-tertiary btn-sm"
                onclick={() => onmove?.(row.item.id, 'ready')}
              >
                Mark ready
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <section class="space-y-3">
      <header class="flex items-center gap-2">
        <span class="status-chip status-ready">ready</span>
        <span class="font-mono text-[0.6875rem] text-text-faint">{ready.length}</span>
      </header>
      {#if ready.length === 0}
        <p
          class="rounded-[var(--radius-card)] border border-dashed border-border px-4 py-8 text-center text-sm text-text-faint"
        >
          Ready ideas appear here when you are set to produce them.
        </p>
      {:else}
        <ul class="space-y-2.5" role="list">
          {#each ready as row (row.item.id)}
            <li class="space-y-2">
              <DevelopIdeaCard {row} ready />
              <button
                type="button"
                class="btn btn-tertiary btn-sm"
                onclick={() => onmove?.(row.item.id, 'brewing')}
              >
                Move back to brewing
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  </div>
{/if}
