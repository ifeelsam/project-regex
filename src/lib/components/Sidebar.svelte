<script lang="ts">
  import { page } from '$app/stores';
  import { navItems } from '$lib/nav';

  function isActive(href: string, pathname: string) {
    return pathname === href || pathname.startsWith(href.slice(0, -1));
  }
</script>

<nav
  class="flex h-full w-[11.5rem] shrink-0 flex-col border-r border-border bg-bg-raised px-3.5 pb-5"
>
  <div class="titlebar-inset" data-tauri-drag-region aria-hidden="true"></div>

  <div class="mb-6 flex items-center gap-2.5 px-2 pb-4">
    <div class="brand-mark size-6 text-[0.7rem]">.*</div>
    <span class="brand-wordmark text-sm">
      <span class="brand-slash">/</span>regex<span class="brand-slash">/</span>
    </span>
  </div>

  <ul class="flex flex-1 flex-col gap-1" role="list">
    {#each navItems as item (item.href)}
      <li>
        <a
          href={item.href}
          class={['nav-item', isActive(item.href, $page.url.pathname) && 'nav-item-active']}
          aria-current={isActive(item.href, $page.url.pathname) ? 'page' : undefined}
        >
          {item.label}
        </a>
      </li>
    {/each}
  </ul>

  <p class="px-2 pt-4 font-mono text-[0.6875rem] leading-relaxed text-text-faint">
    capture light · break down heavy
  </p>
</nav>
