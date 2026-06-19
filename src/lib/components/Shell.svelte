<script lang="ts">
  import type { Snippet } from 'svelte';
  import Sidebar from './Sidebar.svelte';

  let {
    title,
    description,
    variant = 'default',
    children
  }: {
    title: string;
    description?: string;
    variant?: 'default' | 'library';
    children: Snippet;
  } = $props();
</script>

<div class="flex h-screen overflow-hidden bg-bg">
  <Sidebar />

  <div class={['flex min-w-0 flex-1 flex-col', variant === 'library' && 'surface-library']}>
    <header
      class="flex shrink-0 items-end justify-between border-b border-border px-8 pb-5"
      style="padding-top: var(--spacing-titlebar)"
      data-tauri-drag-region
    >
      <div>
        <h1 class="heading-page">{title}</h1>
        {#if description}
          <p class="mt-1.5 max-w-prose text-sm text-text-muted text-pretty">{description}</p>
        {/if}
      </div>
    </header>

    <main class="min-h-0 flex-1 overflow-y-auto px-8 py-6">
      {@render children()}
    </main>
  </div>
</div>
