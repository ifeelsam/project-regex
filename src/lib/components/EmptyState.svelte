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
    class="mb-5 flex size-14 items-center justify-center rounded-2xl bg-[#f3eee1] dark:bg-bg-overlay"
    aria-hidden="true"
  >
    <span class="font-mono text-2xl text-[#cfc7b4]">.*</span>
  </div>
  <h2 class="heading-display text-xl">{title}</h2>
  <p class="mt-2 text-sm leading-relaxed text-text-muted text-pretty">{description}</p>
  {#if action}
    <div class="mt-6">
      {@render action()}
    </div>
  {/if}
</div>
