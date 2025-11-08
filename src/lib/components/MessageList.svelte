<script lang="ts">
  import { mailbox } from '$lib/stores/mailboxStore';
  import type { MessageHeader } from '$lib/types';
  import { Mail, MailOpen, Star, Trash2, Check, Loader2 } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{ selectionChange: number[] }>();

  let contextMenuMessage: MessageHeader | null = null;
  let contextMenuPosition = { x: 0, y: 0 };
  let showContextMenu = false;

  // Phase 3: Multi-select functionality
  let selectedMessageIds = new Set<number>();
  let lastSelectedIndex = -1;

  // Phase 5: Infinite scroll
  let scrollContainer: HTMLDivElement;
  let isLoadingMore = false;

  export function getSelectedMessages(): number[] {
    return Array.from(selectedMessageIds);
  }

  export function clearSelection() {
    selectedMessageIds = new Set();
    dispatch('selectionChange', []);
  }

  function handleMessageSelect(message: MessageHeader, event?: MouseEvent) {
    // Multi-select with Ctrl/Cmd
    if (event && (event.ctrlKey || event.metaKey)) {
      event.preventDefault();
      toggleMessageSelection(message);
      return;
    }

    // Range select with Shift
    if (event && event.shiftKey && lastSelectedIndex >= 0) {
      event.preventDefault();
      handleRangeSelect(message);
      return;
    }

    // Normal selection - clear multi-select and view message
    selectedMessageIds = new Set();
    lastSelectedIndex = $mailbox.messages.findIndex((m) => m.id === message.id);
    mailbox.selectMessage(message);
  }

  function toggleMessageSelection(message: MessageHeader) {
    if (selectedMessageIds.has(message.id)) {
      selectedMessageIds.delete(message.id);
    } else {
      selectedMessageIds.add(message.id);
    }
    selectedMessageIds = selectedMessageIds; // Trigger reactivity
    lastSelectedIndex = $mailbox.messages.findIndex((m) => m.id === message.id);
    dispatch('selectionChange', Array.from(selectedMessageIds));
  }

  function handleRangeSelect(message: MessageHeader) {
    const currentIndex = $mailbox.messages.findIndex((m) => m.id === message.id);
    const start = Math.min(lastSelectedIndex, currentIndex);
    const end = Math.max(lastSelectedIndex, currentIndex);

    for (let i = start; i <= end; i++) {
      selectedMessageIds.add($mailbox.messages[i].id);
    }
    selectedMessageIds = selectedMessageIds; // Trigger reactivity
    dispatch('selectionChange', Array.from(selectedMessageIds));
  }

  function handleContextMenu(event: MouseEvent, message: MessageHeader) {
    event.preventDefault();
    contextMenuMessage = message;
    contextMenuPosition = { x: event.clientX, y: event.clientY };
    showContextMenu = true;
  }

  function closeContextMenu() {
    showContextMenu = false;
    contextMenuMessage = null;
  }

  function handleMarkAsRead() {
    if (contextMenuMessage) {
      mailbox.markRead(contextMenuMessage.id);
    }
    closeContextMenu();
  }

  function handleMarkAsUnread() {
    if (contextMenuMessage) {
      mailbox.markUnread(contextMenuMessage.id);
    }
    closeContextMenu();
  }

  // Phase 3: Star/unstar from context menu
  function handleStar() {
    if (contextMenuMessage) {
      mailbox.starMessage(contextMenuMessage.id);
    }
    closeContextMenu();
  }

  function handleUnstar() {
    if (contextMenuMessage) {
      mailbox.unstarMessage(contextMenuMessage.id);
    }
    closeContextMenu();
  }

  // Phase 3: Delete from context menu
  function handleDelete() {
    if (contextMenuMessage) {
      mailbox.deleteMessage(contextMenuMessage.id);
    }
    closeContextMenu();
  }

  function handleClickOutside(event: MouseEvent) {
    if (showContextMenu) {
      closeContextMenu();
    }
  }

  // Phase 5: Infinite scroll handler
  async function handleScroll(event: Event) {
    const target = event.target as HTMLDivElement;
    const scrollThreshold = 200; // Load more when within 200px of bottom

    if (
      !isLoadingMore &&
      $mailbox.hasMore &&
      !$mailbox.loading &&
      target.scrollHeight - target.scrollTop - target.clientHeight < scrollThreshold
    ) {
      isLoadingMore = true;
      await mailbox.loadMoreMessages();
      isLoadingMore = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div
  class="flex flex-col gap-1 p-2 h-full overflow-y-auto"
  bind:this={scrollContainer}
  on:scroll={handleScroll}
>
  {#if $mailbox.messages.length === 0}
    <div class="flex items-center justify-center p-8 text-sm text-muted-foreground">
      {#if $mailbox.selectedFolder}
        No messages in this folder
      {:else}
        Select a folder to view messages
      {/if}
    </div>
  {:else}
    {#each $mailbox.messages as message}
      <button
        class="flex gap-2 items-start rounded-md border p-3 text-left text-sm hover:bg-accent transition-colors {$mailbox
          .selectedMessage?.id === message.id
          ? 'bg-accent'
          : ''} {selectedMessageIds.has(message.id) ? 'ring-2 ring-primary' : ''}"
        on:click={(e) => handleMessageSelect(message, e)}
        on:contextmenu={(e) => handleContextMenu(e, message)}
      >
        <!-- Multi-select checkbox -->
        <div class="mt-1 flex items-center">
          <input
            type="checkbox"
            class="w-4 h-4 rounded border-gray-300 text-primary focus:ring-primary"
            checked={selectedMessageIds.has(message.id)}
            on:click|stopPropagation={() => toggleMessageSelection(message)}
            aria-label="Select message"
          />
        </div>

        <!-- Read/Unread icon -->
        <div class="mt-1">
          {#if message.is_read}
            <MailOpen class="h-4 w-4 text-muted-foreground" />
          {:else}
            <Mail class="h-4 w-4 text-primary" />
          {/if}
        </div>

        <div class="flex-1 min-w-0 flex flex-col gap-1">
          <div class="flex items-center gap-2">
            <div class="truncate {message.is_read ? '' : 'font-bold'} flex-1">{message.from}</div>
            {#if message.is_starred}
              <Star class="h-4 w-4 text-yellow-500 fill-yellow-500 flex-shrink-0" />
            {/if}
          </div>
          <div class="truncate {message.is_read ? 'text-muted-foreground' : 'font-semibold'}">
            {message.subject || '(No Subject)'}
          </div>
          <div class="text-xs text-muted-foreground">
            {new Date(message.date * 1000).toLocaleDateString()}
            {new Date(message.date * 1000).toLocaleTimeString([], {
              hour: '2-digit',
              minute: '2-digit',
            })}
          </div>
        </div>
      </button>
    {/each}

    <!-- Phase 5: Pagination info and loading indicator -->
    {#if $mailbox.totalMessages > 0}
      <div
        class="flex flex-col items-center justify-center p-4 text-xs text-muted-foreground gap-2"
      >
        <div>
          Showing {$mailbox.messages.length} of {$mailbox.totalMessages} messages
        </div>
        {#if isLoadingMore || $mailbox.loading}
          <div class="flex items-center gap-2">
            <Loader2 class="h-4 w-4 animate-spin" />
            <span>Loading more messages...</span>
          </div>
        {:else if $mailbox.hasMore}
          <div class="text-muted-foreground/70">Scroll down to load more</div>
        {/if}
      </div>
    {/if}
  {/if}
</div>

{#if showContextMenu && contextMenuMessage}
  <div
    class="fixed bg-popover border rounded-md shadow-md py-1 z-50 min-w-48"
    style="left: {contextMenuPosition.x}px; top: {contextMenuPosition.y}px;"
    role="menu"
    tabindex="-1"
    on:click|stopPropagation
    on:keydown={(e) => {
      if (e.key === 'Escape') {
        closeContextMenu();
      }
    }}
  >
    {#if contextMenuMessage.is_read}
      <button
        class="w-full px-4 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
        on:click={handleMarkAsUnread}
      >
        <Mail class="h-4 w-4" />
        Mark as Unread
      </button>
    {:else}
      <button
        class="w-full px-4 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
        on:click={handleMarkAsRead}
      >
        <MailOpen class="h-4 w-4" />
        Mark as Read
      </button>
    {/if}

    <div class="border-t my-1"></div>

    {#if contextMenuMessage.is_starred}
      <button
        class="w-full px-4 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
        on:click={handleUnstar}
      >
        <Star class="h-4 w-4" />
        Unstar
      </button>
    {:else}
      <button
        class="w-full px-4 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
        on:click={handleStar}
      >
        <Star class="h-4 w-4" />
        Star
      </button>
    {/if}

    <div class="border-t my-1"></div>

    <button
      class="w-full px-4 py-2 text-left text-sm hover:bg-accent flex items-center gap-2 text-destructive"
      on:click={handleDelete}
    >
      <Trash2 class="h-4 w-4" />
      Delete
    </button>
  </div>
{/if}
