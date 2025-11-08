<script lang="ts">
  import { createEventDispatcher, onDestroy } from 'svelte';
  import { Search, X } from 'lucide-svelte';
  import Input from '$lib/components/ui/input/index.svelte';
  import Button from '$lib/components/ui/button/index.svelte';

  let query = '';
  let timeoutId: ReturnType<typeof setTimeout> | undefined;
  const dispatch = createEventDispatcher<{ search: string }>();

  // Phase 5: Optimized debouncing (300ms)
  function handleInput() {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    timeoutId = setTimeout(() => {
      dispatch('search', query);
    }, 300);
  }

  function clear() {
    query = '';
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    dispatch('search', '');
  }

  // Phase 5: Cleanup on destroy
  onDestroy(() => {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
  });
</script>

<div class="relative flex items-center gap-2">
  <div class="relative flex-1">
    <Search
      class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground pointer-events-none"
    />
    <Input bind:value={query} on:input={handleInput} placeholder="Search emails..." class="pl-9" />
  </div>
  {#if query}
    <Button variant="ghost" size="sm" on:click={clear}>
      <X class="h-4 w-4" />
    </Button>
  {/if}
</div>
