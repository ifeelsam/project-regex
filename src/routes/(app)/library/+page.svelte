<script lang="ts">
  import EmptyState from '$lib/components/EmptyState.svelte';
</script>

<div class="mx-auto max-w-4xl">
  <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
    <div class="flex gap-1.5">
      <span class="tag bg-accent text-white">frames</span>
      <span class="tag">clips</span>
      <span class="tag">audio</span>
    </div>
    <p class="font-mono text-[0.6875rem] text-text-faint">from breakdowns</p>
  </div>

  <label class="search-field mb-6 block max-w-xl">
    <span class="font-mono text-[0.75rem] text-text-faint">/</span>
    <input placeholder="Search asset labels…" disabled />
  </label>

  <EmptyState
    title="No assets yet"
    description="After you graduate an idea and run a breakdown, frames, clips, audio, and transcripts land here — searchable and ready to reuse."
  />
</div>
