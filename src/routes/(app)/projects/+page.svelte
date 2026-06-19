<script lang="ts">
  import { onMount } from 'svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import ProjectCard from '$lib/components/ProjectCard.svelte';
  import { api, type ProjectSummary } from '$lib/api';

  let summaries = $state<ProjectSummary[]>([]);
  let loading = $state(true);
  let message = $state('');

  const active = $derived(summaries.filter((row) => row.project.status === 'active'));
  const shipped = $derived(summaries.filter((row) => row.project.status === 'shipped'));

  async function refresh() {
    loading = true;
    try {
      summaries = await api.listProjectSummaries();
    } catch (error) {
      message = error instanceof Error ? error.message : 'Could not load projects.';
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    void refresh();
  });
</script>

<div class="mx-auto max-w-4xl space-y-8">
  {#if message}
    <p class="banner" role="status">
      {message}
    </p>
  {/if}

  {#if loading}
    <p class="text-sm text-text-muted">Loading…</p>
  {:else if summaries.length === 0}
    <EmptyState
      title="No active projects"
      description="When a ready idea graduates, it becomes a project here — carrying its note, references, and eventually its breakdown assets."
    />
  {:else}
    <section class="space-y-3">
      <header class="flex items-center justify-between">
        <h2 class="text-sm font-medium text-text-muted">Active</h2>
        <span class="text-xs text-text-faint">{active.length}</span>
      </header>
      {#if active.length === 0}
        <p class="text-sm text-text-faint">No active projects right now.</p>
      {:else}
        <ul class="grid gap-3 sm:grid-cols-2" role="list">
          {#each active as summary (summary.project.id)}
            <li><ProjectCard {summary} /></li>
          {/each}
        </ul>
      {/if}
    </section>

    {#if shipped.length}
      <section class="space-y-3">
        <header class="flex items-center justify-between">
          <h2 class="text-sm font-medium text-text-muted">Shipped</h2>
          <span class="text-xs text-text-faint">{shipped.length}</span>
        </header>
        <ul class="grid gap-3 sm:grid-cols-2" role="list">
          {#each shipped as summary (summary.project.id)}
            <li><ProjectCard {summary} /></li>
          {/each}
        </ul>
      </section>
    {/if}
  {/if}
</div>
