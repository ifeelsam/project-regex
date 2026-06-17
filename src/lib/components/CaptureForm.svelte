<script lang="ts">
  import { api, type CaptureItemInput } from '$lib/api';

  let {
    onsaved
  }: {
    onsaved?: (message: string) => void;
  } = $props();

  let url = $state('');
  let note = $state('');
  let tags = $state('');
  let saving = $state(false);
  let thoughtMode = $state(false);

  async function save() {
    if (!note.trim() || saving) return;
    saving = true;

    try {
      const capturedOn = await api.defaultCapturedOn();
      const trimmedUrl = url.trim();
      const platform =
        thoughtMode || !trimmedUrl ? 'manual' : await api.detectPlatform(trimmedUrl);

      const input: CaptureItemInput = {
        url: thoughtMode ? null : trimmedUrl || null,
        platform,
        note: note.trim(),
        tags: tags
          .split(',')
          .map((tag) => tag.trim())
          .filter(Boolean),
        captured_on: capturedOn
      };

      const result = await api.captureItem(input);
      const message = result.created ? 'Saved to your inbox.' : 'That link was already saved.';

      if (result.created && result.item.url) {
        void api.enrichItem(result.item.id);
      }

      url = '';
      note = '';
      tags = '';
      thoughtMode = false;
      onsaved?.(message);
    } catch (error) {
      onsaved?.(error instanceof Error ? error.message : 'Could not save.');
    } finally {
      saving = false;
    }
  }

  function onKeydown(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
      event.preventDefault();
      void save();
    }
  }
</script>

<section class="rounded-[var(--radius-card)] border border-border bg-bg-raised p-5">
  <div class="mb-4 flex items-center justify-between gap-3">
    <div>
      <h2 class="text-sm font-medium">Capture</h2>
      <p class="mt-1 text-sm text-text-muted">
        Start with why it grabbed you — the link can wait.
      </p>
    </div>
    <button
      type="button"
      class="rounded-[var(--radius-control)] border border-border px-3 py-1.5 text-xs text-text-muted transition-colors hover:border-border-strong hover:text-text"
      onclick={() => (thoughtMode = !thoughtMode)}
    >
      {thoughtMode ? 'Add a link' : 'Just a thought'}
    </button>
  </div>

  <div class="space-y-3" onkeydown={onKeydown}>
    {#if !thoughtMode}
      <label class="block">
        <span class="sr-only">Link</span>
        <input
          class="w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2.5 text-sm placeholder:text-text-faint"
          placeholder="Paste a link (optional)"
          bind:value={url}
        />
      </label>
    {/if}

    <label class="block">
      <span class="sr-only">Why did this grab you?</span>
      <textarea
        class="min-h-28 w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2.5 text-sm placeholder:text-text-faint"
        placeholder="Why did this grab you? What idea did it spark?"
        bind:value={note}
      ></textarea>
    </label>

    <label class="block">
      <span class="sr-only">Tags</span>
      <input
        class="w-full rounded-[var(--radius-control)] border border-border bg-bg px-3 py-2.5 text-sm placeholder:text-text-faint"
        placeholder="Tags — comma separated"
        bind:value={tags}
      />
    </label>

    <div class="flex items-center justify-between gap-3 pt-1">
      <p class="text-xs text-text-faint">⌘↵ to save</p>
      <button
        type="button"
        class="rounded-[var(--radius-control)] bg-accent px-4 py-2 text-sm font-medium text-accent-contrast transition-opacity disabled:opacity-40"
        disabled={saving || !note.trim()}
        onclick={save}
      >
        {saving ? 'Saving…' : 'Save'}
      </button>
    </div>
  </div>
</section>
