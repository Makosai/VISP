<script lang="ts">
  import '../app.css';
  import { Button } from '$lib/components/ui/button';
  import { goto } from '$app/navigation';
  import { ModeWatcher, toggleMode } from 'mode-watcher';
  import ShareIcon from '@lucide/svelte/icons/share-2';
  import FilmIcon from '@lucide/svelte/icons/film';
  import ContextMenubar from '$lib/components/ui/visp/ContextMenubar.svelte';
  import * as Avatar from '$lib/components/ui/avatar';

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
    <div class="lead">
      <ContextMenubar />
    </div>
    <nav class="nav-items">
      <div class="flex gap-1">
        {#each navTabs as tab}
          <Button
            size="sm"
            variant="ghost"
            onclick={() => goto(tab.href)}
            aria-label={`Go to ${tab.label} section`}>
            {tab.label}
          </Button>
        {/each}
      </div>
    </nav>
    <div class="actions">
      <Button size="icon" variant="ghost" aria-label="Share">
        <ShareIcon />
      </Button>
      <Button size="icon" variant="ghost" aria-label="Export">
        <FilmIcon />
      </Button>
    </div>
  </header>

  {@render children?.()}
</div>

<style>
  .lead {
    height: 100%;
    width: 13.125rem;
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
    width: 13.125rem;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding-inline: 0.75rem;
    background-color: var(--background);
    height: 42px;
    overflow-x: auto;
    overflow-y: hidden;
  }

  header::-webkit-scrollbar {
    width: 0px;
    height: 0px;
    background: transparent; /* make scrollbar transparent */
  }
</style>
