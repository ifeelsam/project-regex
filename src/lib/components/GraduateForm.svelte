<script lang="ts">
  import { goto } from '$app/navigation';
  import { api, type ProjectFormat } from '$lib/api';

  let {
    ideaId,
    defaultTitle = '',
    defaultBrief = '',
    ondone
  }: {
    ideaId: string;
    defaultTitle?: string;
    defaultBrief?: string;
    ondone?: (message: string) => void;
  } = $props();

  let title = $state(defaultTitle);
  let brief = $state(defaultBrief);
  let format = $state<ProjectFormat>('video');
  let busy = $state(false);
  let error = $state('');

  async function produce() {
    if (!title.trim() || busy) return;
    busy = true;
    error = '';
    try {
      const project = await api.graduateItem(ideaId, title.trim(), brief.trim(), format);
      ondone?.('Project created. Time to produce.');
      await goto(`/projects/${project.id}/`);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Could not create the project.';
    } finally {
      busy = false;
    }
  }
</script>

<section class="card-ready rounded-[var(--radius-card)] bg-white p-5 dark:bg-bg-raised">
  <h3 class="text-sm font-semibold">Produce this</h3>
  <p class="mt-1 text-sm text-text-muted">
    Graduate this ready idea into a project. Its references come with it.
  </p>

  <div class="mt-4 space-y-3">
    <label class="block">
      <span class="meta-label mb-1 block">Project title</span>
      <input class="field" bind:value={title} />
    </label>

    <label class="block">
      <span class="meta-label mb-1 block">Brief</span>
      <textarea
        class="field min-h-20 leading-relaxed"
        placeholder="What are you making, and for whom?"
        bind:value={brief}
      ></textarea>
    </label>

    <label class="block">
      <span class="meta-label mb-1 block">Format</span>
      <select class="field" bind:value={format}>
        <option value="video">Video</option>
        <option value="article">Article</option>
        <option value="other">Other</option>
      </select>
    </label>

    {#if error}
      <p class="text-sm text-danger">{error}</p>
    {/if}

    <button
      type="button"
      class="btn btn-primary w-full sm:w-auto"
      disabled={busy || !title.trim()}
      onclick={produce}
    >
      {busy ? 'Creating project…' : 'Produce this →'}
    </button>
  </div>
</section>
