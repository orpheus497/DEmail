<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import { Mail, MailOpen, Star, Trash2, X } from "lucide-svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ clearSelection: void }>();

  export let selectedMessageIds: number[] = [];

  $: selectedCount = selectedMessageIds.length;
  $: hasSelection = selectedCount > 0;

  async function handleMarkRead() {
    if (selectedMessageIds.length > 0) {
      await mailbox.bulkMarkRead(selectedMessageIds);
      dispatch('clearSelection');
    }
  }

  async function handleMarkUnread() {
    if (selectedMessageIds.length > 0) {
      await mailbox.bulkMarkUnread(selectedMessageIds);
      dispatch('clearSelection');
    }
  }

  async function handleStar() {
    if (selectedMessageIds.length > 0) {
      await mailbox.bulkStar(selectedMessageIds);
      dispatch('clearSelection');
    }
  }

  async function handleUnstar() {
    if (selectedMessageIds.length > 0) {
      await mailbox.bulkUnstar(selectedMessageIds);
      dispatch('clearSelection');
    }
  }

  async function handleDelete() {
    if (selectedMessageIds.length > 0 && confirm(`Delete ${selectedCount} message${selectedCount > 1 ? 's' : ''}?`)) {
      await mailbox.bulkDelete(selectedMessageIds);
      dispatch('clearSelection');
    }
  }

  function handleClearSelection() {
    dispatch('clearSelection');
  }
</script>

{#if hasSelection}
  <div
    class="fixed bottom-4 left-1/2 -translate-x-1/2 bg-card border rounded-lg shadow-lg p-3 z-50 flex items-center gap-2 animate-in fade-in slide-in-from-bottom-4"
    role="toolbar"
    aria-label="Bulk actions toolbar"
  >
    <div class="flex items-center gap-2 px-2 border-r pr-4">
      <span class="text-sm font-medium">{selectedCount} selected</span>
      <Button
        variant="ghost"
        size="sm"
        on:click={handleClearSelection}
        aria-label="Clear selection"
      >
        <X class="h-4 w-4" />
      </Button>
    </div>

    <div class="flex items-center gap-1">
      <Button
        variant="ghost"
        size="sm"
        on:click={handleMarkRead}
        title="Mark as read"
      >
        <MailOpen class="h-4 w-4" />
      </Button>

      <Button
        variant="ghost"
        size="sm"
        on:click={handleMarkUnread}
        title="Mark as unread"
      >
        <Mail class="h-4 w-4" />
      </Button>

      <Button
        variant="ghost"
        size="sm"
        on:click={handleStar}
        title="Star messages"
      >
        <Star class="h-4 w-4" />
      </Button>

      <Button
        variant="ghost"
        size="sm"
        on:click={handleUnstar}
        title="Unstar messages"
      >
        <Star class="h-4 w-4 fill-current" />
      </Button>

      <div class="border-l h-6 mx-2"></div>

      <Button
        variant="ghost"
        size="sm"
        on:click={handleDelete}
        title="Delete messages"
        class="text-destructive hover:text-destructive"
      >
        <Trash2 class="h-4 w-4" />
      </Button>
    </div>
  </div>
{/if}
