<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import type { MessageHeader } from "$lib/types";
  import { Mail, MailOpen } from "lucide-svelte";

  let contextMenuMessage: MessageHeader | null = null;
  let contextMenuPosition = { x: 0, y: 0 };
  let showContextMenu = false;

  function handleMessageSelect(message: MessageHeader) {
    mailbox.selectMessage(message);
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

  function handleClickOutside(event: MouseEvent) {
    if (showContextMenu) {
      closeContextMenu();
    }
  }
</script>

<svelte:window on:click={handleClickOutside} />

<div class="flex flex-col gap-1 p-2">
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
        class="flex gap-2 items-start rounded-md border p-3 text-left text-sm hover:bg-accent transition-colors {$mailbox.selectedMessage?.id === message.id ? 'bg-accent' : ''}"
        on:click={() => handleMessageSelect(message)}
        on:contextmenu={(e) => handleContextMenu(e, message)}
      >
        <div class="mt-1">
          {#if message.is_read}
            <MailOpen class="h-4 w-4 text-muted-foreground" />
          {:else}
            <Mail class="h-4 w-4 text-primary" />
          {/if}
        </div>
        <div class="flex-1 min-w-0 flex flex-col gap-1">
          <div class="truncate {message.is_read ? '' : 'font-bold'}">{message.from}</div>
          <div class="truncate {message.is_read ? 'text-muted-foreground' : 'font-semibold'}">{message.subject || '(No Subject)'}</div>
          <div class="text-xs text-muted-foreground">
            {new Date(message.date * 1000).toLocaleDateString()} {new Date(message.date * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
          </div>
        </div>
      </button>
    {/each}
  {/if}
</div>

{#if showContextMenu && contextMenuMessage}
  <div
    class="fixed bg-popover border rounded-md shadow-md py-1 z-50"
    style="left: {contextMenuPosition.x}px; top: {contextMenuPosition.y}px;"
    on:click|stopPropagation
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
  </div>
{/if}