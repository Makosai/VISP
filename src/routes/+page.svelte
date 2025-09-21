<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/ui/button';
</script>

<main class="container">
  <h1>VISP Test</h1>

  <Button onclick={() => goto('/video-cut')}>Video Test</Button>
</main>

<style>
</style>
