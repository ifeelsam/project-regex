<script lang="ts">
  import { onMount } from 'svelte';
  import DevelopBoard from '$lib/components/DevelopBoard.svelte';
  import { api, type ItemWithTags } from '$lib/api';

  let rows = $state<ItemWithTags[]>([]);
  let message = $state('');
  let loading = $state(true);

  async function refresh() {
    loading = true;
    try {
      rows = await api.listDevelopItems();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not load ideas.';
    } finally {
      loading = false;
    }
  }

  async function move(id: string, status: 'brewing' | 'ready') {
    try {
      await api.updateItemStatus(id, status);
      message =
        status === 'ready' ? 'Idea marked ready to produce.' : 'Idea moved back to brewing.';
      await refresh();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not update status.';
    }
  }

  onMount(() => {
    void refresh();
  });
</script>

<div class="mx-auto max-w-5xl">
  {#if message}
    <p class="banner mb-4" role="status">
      {message}
    </p>
  {/if}

  {#if loading}
    <p class="text-sm text-text-muted">Loading…</p>
  {:else}
    <DevelopBoard {rows} onmove={move} />
  {/if}
</div>
