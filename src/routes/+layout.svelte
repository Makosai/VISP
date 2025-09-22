<script lang="ts">
  import '../app.css';
  import { Button } from '$lib/components/ui/button';
  import { goto } from '$app/navigation';

  const navTabs = [
    { label: 'Video', href: '/video' },
    { label: 'Image', href: '/image' },
    { label: 'Sound', href: '/sound' },
    { label: 'Post', href: '/post' }
  ];

  let { children } = $props();
</script>

<div class="flex flex-col min-h-screen bg-background text-foreground">
  <!-- Top Bar -->
  <header class="sticky top-0 z-20 w-full border-b border-border">
    <nav class="flex justify-center items-center gap-2 py-2">
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
  </header>

  {@render children()}
</div>
