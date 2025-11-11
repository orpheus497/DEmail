<script lang="ts">
  import Button from '$lib/components/ui/button/index.svelte';
  import * as Select from '$lib/components/ui/select';
  import { ChevronLeft, ChevronRight } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  export let currentPage = 1;
  export let totalMessages = 0;
  export let pageSize = 50;

  let selectedPageSize: { value: string } | undefined = undefined;

  const dispatch = createEventDispatcher<{
    pageChange: { page: number; pageSize: number };
  }>();

  $: totalPages = Math.ceil(totalMessages / pageSize);
  $: startIndex = (currentPage - 1) * pageSize + 1;
  $: endIndex = Math.min(currentPage * pageSize, totalMessages);

  $: if (selectedPageSize) {
    const newPageSize = parseInt(selectedPageSize.value);
    dispatch('pageChange', { page: 1, pageSize: newPageSize });
  }

  function handlePreviousPage() {
    if (currentPage > 1) {
      const newPage = currentPage - 1;
      dispatch('pageChange', { page: newPage, pageSize });
    }
  }

  function handleNextPage() {
    if (currentPage < totalPages) {
      const newPage = currentPage + 1;
      dispatch('pageChange', { page: newPage, pageSize });
    }
  }
</script>

<div class="flex items-center justify-between p-2 border-t bg-background">
  <div class="flex items-center gap-2 text-sm text-muted-foreground">
    <span>
      {totalMessages === 0 ? 'No messages' : `${startIndex}-${endIndex} of ${totalMessages}`}
    </span>
  </div>

  <div class="flex items-center gap-2">
    <div class="flex items-center gap-2 text-sm">
      <span class="text-muted-foreground">Per page:</span>
      <Select.Root bind:selected={selectedPageSize}>
        <Select.Trigger class="w-20 h-8">
          <Select.Value placeholder={pageSize.toString()} />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="25">25</Select.Item>
          <Select.Item value="50">50</Select.Item>
          <Select.Item value="100">100</Select.Item>
          <Select.Item value="200">200</Select.Item>
        </Select.Content>
      </Select.Root>
    </div>

    <div class="flex items-center gap-1">
      <Button variant="ghost" size="sm" on:click={handlePreviousPage} disabled={currentPage <= 1}>
        <ChevronLeft class="h-4 w-4" />
      </Button>
      <span class="text-sm text-muted-foreground px-2">
        Page {currentPage} of {totalPages || 1}
      </span>
      <Button
        variant="ghost"
        size="sm"
        on:click={handleNextPage}
        disabled={currentPage >= totalPages}
      >
        <ChevronRight class="h-4 w-4" />
      </Button>
    </div>
  </div>
</div>
