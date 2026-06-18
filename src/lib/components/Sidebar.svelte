<script lang="ts">
  import { page } from '$app/stores';
  import { navItems } from '$lib/nav';

  function isActive(href: string, pathname: string) {
    return pathname === href || pathname.startsWith(href.slice(0, -1));
  }
</script>

<nav class="flex h-full w-56 shrink-0 flex-col border-r border-border bg-bg-raised px-3 pb-5">
  <div class="titlebar-inset" data-tauri-drag-region aria-hidden="true"></div>

  <div class="mb-8 px-3">
    <p class="text-xs font-medium uppercase tracking-[0.14em] text-text-faint">Regex</p>
    <p class="mt-1 text-sm text-text-muted">Inspiration to output</p>
  </div>

  <ul class="flex flex-1 flex-col gap-0.5" role="list">
    {#each navItems as item (item.href)}
      <li>
        <a
          href={item.href}
          class={[
            'block rounded-[var(--radius-control)] px-3 py-2 text-sm transition-colors',
            isActive(item.href, $page.url.pathname)
              ? 'bg-accent-soft font-medium text-text'
              : 'text-text-muted hover:bg-bg-overlay hover:text-text'
          ]}
          aria-current={isActive(item.href, $page.url.pathname) ? 'page' : undefined}
        >
          {item.label}
        </a>
      </li>
    {/each}
  </ul>
</nav>
