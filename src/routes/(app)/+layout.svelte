<script lang="ts">
  import { page } from '$app/stores';
  import Shell from '$lib/components/Shell.svelte';
  import { navItems } from '$lib/nav';

  let { children } = $props();

  const current = $derived(
    navItems.find(
      (item) =>
        $page.url.pathname === item.href ||
        $page.url.pathname.startsWith(item.href.slice(0, -1))
    ) ?? navItems[0]
  );

  const shellVariant = $derived(
    $page.url.pathname.startsWith('/library') ? 'library' : 'default'
  );
</script>

<Shell title={current.label} description={current.description} variant={shellVariant}>
  {@render children()}
</Shell>
