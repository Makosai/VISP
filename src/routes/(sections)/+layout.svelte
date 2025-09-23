<script lang="ts">
  import { footerActionsStore } from '$lib';
  import { Button } from '$lib/components/ui/button';
  import { Separator } from '$lib/components/ui/separator';
  import type { Snippet } from 'svelte';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

{@render children?.()}

<footer class="footer-bar">
  <div class="footer-actions">
    {@render footerActions?.($footerActionsStore)}
  </div>
</footer>

{#snippet footerActions(actions: Footer.Action[])}
  {#each actions as action, i}
    {#if i > 0}
      <Separator orientation="vertical" />
    {/if}
    {#if action.type === 'button'}
      <Button
        class="footer-btn"
        onclick={action.onClick}
        disabled={action.disabled}
        size="sm"
        variant={action.active ? 'secondary' : 'ghost'}
        aria-current={action.active ? 'page' : undefined}>
        {action.label}
      </Button>
    {:else if action.type === 'link'}
      <Button
        class="footer-btn"
        href={action.href}
        target={action.target}
        rel="noopener noreferrer"
        disabled={action.disabled}
        size="sm"
        variant={action.active ? 'secondary' : 'ghost'}
        aria-current={action.active ? 'page' : undefined}>
        {action.label}
      </Button>
    {/if}
  {/each}
{/snippet}

<style>
  .footer-bar {
    position: sticky;
    bottom: 0;
    top: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--background);
    border-top: 1px solid var(--border);
    height: 42px;
    z-index: 10;
  }
  .footer-actions {
    display: flex;
    gap: 0.25rem;
    width: 100%;
    justify-content: center;
    align-items: center;
  }
</style>
