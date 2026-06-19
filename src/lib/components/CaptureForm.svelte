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

<section class="card p-4 sm:p-5">
  <div class="mb-4 flex items-center justify-between gap-3">
    <div>
      <h2 class="text-sm font-semibold">Capture</h2>
      <p class="mt-1 text-sm text-text-muted">
        Start with why it grabbed you — the link can wait.
      </p>
    </div>
    <button
      type="button"
      class="btn btn-secondary btn-sm"
      onclick={() => (thoughtMode = !thoughtMode)}
    >
      {thoughtMode ? 'Add a link' : 'Just a thought'}
    </button>
  </div>

  <form class="space-y-3" onkeydown={onKeydown} onsubmit={(e) => e.preventDefault()}>
    {#if !thoughtMode}
      <div class="link-field">
        <span class="meta-label shrink-0 text-[0.75rem]">paste link</span>
        <input
          class="min-w-0 flex-1 border-none bg-transparent text-sm text-text-body outline-none"
          placeholder="Optional — fills in on its own"
          bind:value={url}
        />
      </div>
    {/if}

    <label class="block">
      <span class="mb-2 block text-sm font-semibold">Why did this grab you?</span>
      <textarea
        class="field field-note min-h-28 leading-relaxed"
        placeholder="What idea did it spark?"
        bind:value={note}
      ></textarea>
    </label>

    <div class="flex flex-wrap items-center gap-2">
      {#each tags
        .split(',')
        .map((tag) => tag.trim())
        .filter(Boolean) as tag (tag)}
        <span class="tag">#{tag}</span>
      {/each}
      <input
        class="field max-w-xs flex-1 border-dashed py-2 font-mono text-xs"
        placeholder="# tags — comma separated"
        bind:value={tags}
      />
    </div>

    <div class="flex items-center justify-between gap-3 pt-1">
      <p class="font-mono text-xs text-text-faint">⌘↵ to save</p>
      <button
        type="button"
        class="btn btn-primary"
        disabled={saving || !note.trim()}
        onclick={save}
      >
        {saving ? 'Saving…' : 'Save to inbox'}
      </button>
    </div>
  </form>
</section>
