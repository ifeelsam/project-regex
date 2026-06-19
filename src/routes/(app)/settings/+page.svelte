<script lang="ts">
  import { theme, toggleTheme } from '$lib/theme.svelte';
</script>

<section class="max-w-xl space-y-6">
  <div class="card-flat p-5">
    <h2 class="text-sm font-semibold">Appearance</h2>
    <p class="mt-1 text-sm text-text-muted">
      Light is the default — warm paper, vermilion accents. Switch to dark for focus surfaces.
    </p>
    <button type="button" class="btn btn-secondary mt-4" onclick={toggleTheme}>
      Switch to {theme.value === 'dark' ? 'light' : 'dark'} mode
    </button>
  </div>

  <div class="card-flat p-5">
    <h2 class="text-sm font-semibold">Personal use</h2>
    <p class="mt-2 text-sm leading-relaxed text-text-muted text-pretty">
      Regex is for personal reference only. Save and process content you have the right to use
      — your own work, or public material kept for inspiration. The app never bypasses platform
      protections or DRM.
    </p>
  </div>

  <div class="card-flat p-5">
    <h2 class="text-sm font-semibold">Developer</h2>
    <p class="mt-1 text-sm text-text-muted">
      Temporary screen for exercising the local data layer.
    </p>
    <a href="/debug/" class="btn btn-tertiary mt-4 px-0">Open data debug screen</a>
  </div>
</section>
