<script lang="ts">
  import type { ProjectFormat, ProjectSummary } from '$lib/api';
  import { statusChipClass } from '$lib/status';

  let {
    summary
  }: {
    summary: ProjectSummary;
  } = $props();

  const { project, primary_item, reference_count } = $derived(summary);
  const label = $derived(
    project.title ||
      primary_item.item.title ||
      primary_item.item.note.slice(0, 72) ||
      'Untitled project'
  );

  const formatLabel: Record<ProjectFormat, string> = {
    video: 'Video',
    article: 'Article',
    other: 'Other'
  };
</script>

<a
  href="/projects/{project.id}/"
  class="card-flat block bg-white p-4 transition-colors hover:border-border-strong dark:bg-bg-raised"
>
  <div class="flex items-start justify-between gap-3">
    <div class="min-w-0">
      <h3 class="text-sm font-semibold leading-snug">{label}</h3>
      {#if project.brief}
        <p class="mt-1 line-clamp-2 text-xs leading-relaxed text-text-muted">
          {project.brief}
        </p>
      {/if}
    </div>
    <span class={['status-chip shrink-0', statusChipClass(project.status)]}>
      {project.status}
    </span>
  </div>

  <div class="mt-3 flex flex-wrap gap-2 font-mono text-xs text-text-faint">
    <span>{formatLabel[project.format]}</span>
    <span>·</span>
    <span>{reference_count} reference{reference_count === 1 ? '' : 's'}</span>
  </div>
</a>
