<script lang="ts">
  import type { Item, Tag } from '$lib/api';

  let {
    row
  }: {
    row: { item: Item; tags: Tag[] };
  } = $props();

  const item = $derived(row.item);
  const label = $derived(item.title || item.note.slice(0, 80) || 'Untitled');
</script>

<a
  href="/develop/{item.id}/"
  class="block rounded-[var(--radius-card)] border border-border bg-bg-raised p-4 transition-colors hover:border-border-strong"
>
  <div class="flex items-start justify-between gap-3">
    <div class="min-w-0">
      <h3 class="text-sm font-medium leading-snug">{label}</h3>
      {#if item.note}
        <p class="mt-1 line-clamp-2 text-xs leading-relaxed text-text-muted">{item.note}</p>
      {/if}
    </div>
    <span
      class="shrink-0 rounded-full border border-border px-2 py-0.5 text-[10px] uppercase tracking-wide text-text-faint"
    >
      {item.status}
    </span>
  </div>

  {#if row.tags.length}
    <div class="mt-3 flex flex-wrap gap-1">
      {#each row.tags as tag (tag.id)}
        <span class="rounded-full bg-accent-soft px-2 py-0.5 text-[10px] text-text"
          >{tag.name}</span
        >
      {/each}
    </div>
  {/if}
</a>
