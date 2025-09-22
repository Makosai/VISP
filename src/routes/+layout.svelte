<script lang="ts">
  import '../app.css';
  import { Button } from '$lib/components/ui/button';
  import { goto } from '$app/navigation';
  import { ModeWatcher, toggleMode } from 'mode-watcher';
  import SunIcon from '@lucide/svelte/icons/sun';
  import MoonIcon from '@lucide/svelte/icons/moon';

  const navTabs = [
    { label: 'Video', href: '/video' },
    { label: 'Image', href: '/image' },
    { label: 'Sound', href: '/sound' },
    { label: 'Post', href: '/post' }
  ];

  let { children } = $props();
</script>

<ModeWatcher />
<div class="flex flex-col min-h-screen bg-background text-foreground">
  <!-- Top Bar -->
  <header class="sticky top-0 z-20 w-full border-b border-border">
    <div class="lead"></div>
    <nav class="nav-items">
      <div class="flex gap-1">
        {#each navTabs as tab}
          <Button
            size="sm"
            variant="ghost"
            class="rounded"
            onclick={() => goto(tab.href)}
            aria-label={`Go to ${tab.label} section`}>
            {tab.label}
          </Button>
        {/each}
      </div>
    </nav>
    <div class="actions">
      <Button onclick={toggleMode} variant="outline" size="icon">
        <SunIcon
          class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0" />
        <MoonIcon
          class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100" />
        <span class="sr-only">Toggle theme</span>
      </Button>
    </div>
  </header>

  {@render children?.()}
</div>

<style>
  .lead {
    height: 100%;
    width: 70px;
    background-color: var(--foreground);
    opacity: 0.045;
  }
  .nav-items {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.5rem;
  }
  .actions {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.5rem;
    width: 70px;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
    background-color: var(--background);
    height: 53px;
  }
</style>
