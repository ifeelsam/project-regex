<script lang="ts">
  import type { Item, Tag } from '$lib/api';
  import { statusChipClass } from '$lib/status';

  let {
    row,
    ready = false
  }: {
    row: { item: Item; tags: Tag[] };
    ready?: boolean;
  } = $props();

  const item = $derived(row.item);
  const label = $derived(item.title || item.note.slice(0, 80) || 'Untitled');
</script>

<a
  href="/develop/{item.id}/"
  class={[
    'block rounded-[0.6875rem] border bg-white p-3 transition-colors dark:bg-bg-raised',
    ready ? 'card-ready' : 'border-border hover:border-border-strong'
  ]}
>
  <div class="flex items-start justify-between gap-3">
    <div class="min-w-0">
      <h3 class="text-[0.8125rem] font-semibold leading-snug">{label}</h3>
      {#if item.note}
        <p class="mt-1 line-clamp-2 text-[0.71875rem] leading-relaxed text-text-muted">
          {item.note}
        </p>
      {/if}
    </div>
    <span class={['status-chip shrink-0', statusChipClass(item.status)]}>
      {item.status}
    </span>
  </div>

  {#if row.tags.length}
    <div class="mt-2.5 flex flex-wrap gap-1">
      {#each row.tags as tag (tag.id)}
        <span class="tag">#{tag.name}</span>
      {/each}
    </div>
  {/if}
</a>
