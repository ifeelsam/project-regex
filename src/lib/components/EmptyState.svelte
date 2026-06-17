<script lang="ts">
  import type { Snippet } from 'svelte';

  let {
    title,
    description,
    action
  }: {
    title: string;
    description: string;
    action?: Snippet;
  } = $props();
</script>

<div class="mx-auto flex max-w-md flex-col items-center px-6 py-16 text-center">
  <div
    class="mb-5 flex size-12 items-center justify-center rounded-full border border-border bg-bg-overlay"
    aria-hidden="true"
  >
    <span class="size-2 rounded-full bg-accent"></span>
  </div>
  <h2 class="text-lg font-semibold tracking-tight">{title}</h2>
  <p class="mt-2 text-sm leading-relaxed text-text-muted">{description}</p>
  {#if action}
    <div class="mt-6">
      {@render action()}
    </div>
  {/if}
</div>
