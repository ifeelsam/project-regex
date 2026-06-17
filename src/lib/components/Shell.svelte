<script lang="ts">
  import type { Snippet } from 'svelte';
  import Sidebar from './Sidebar.svelte';

  let {
    title,
    description,
    children
  }: {
    title: string;
    description?: string;
    children: Snippet;
  } = $props();
</script>

<div class="flex h-screen overflow-hidden bg-bg">
  <Sidebar />

  <div class="flex min-w-0 flex-1 flex-col">
    <header
      class="flex shrink-0 items-end justify-between border-b border-border px-8 pb-5 pt-8"
      data-tauri-drag-region
    >
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">{title}</h1>
        {#if description}
          <p class="mt-1 text-sm text-text-muted">{description}</p>
        {/if}
      </div>
    </header>

    <main class="min-h-0 flex-1 overflow-y-auto px-8 py-6">
      {@render children()}
    </main>
  </div>
</div>
