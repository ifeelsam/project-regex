<script lang="ts">
  import { theme, toggleTheme } from '$lib/theme.svelte';
</script>

<section class="max-w-xl space-y-8">
  <div class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
    <h2 class="text-sm font-medium">Appearance</h2>
    <p class="mt-1 text-sm text-text-muted">Dark mode is on by default. Switch anytime.</p>
    <button
      type="button"
      class="mt-4 rounded-[var(--radius-control)] border border-border bg-bg-overlay px-4 py-2 text-sm text-text transition-colors hover:border-border-strong"
      onclick={toggleTheme}
    >
      Switch to {theme.value === 'dark' ? 'light' : 'dark'} mode
    </button>
  </div>

  <div class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
    <h2 class="text-sm font-medium">Personal use</h2>
    <p class="mt-2 text-sm leading-relaxed text-text-muted">
      Regex is for personal reference only. Save and process content you have the right to use
      — your own work, or public material kept for inspiration. The app never bypasses platform
      protections or DRM.
    </p>
  </div>
</section>
