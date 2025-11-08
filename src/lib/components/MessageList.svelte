<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import type { MessageHeader } from "$lib/types";

  function handleMessageSelect(message: MessageHeader) {
    mailbox.selectMessage(message);
  }
</script>

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
        class="flex flex-col gap-1 rounded-md border p-3 text-left text-sm hover:bg-accent transition-colors {$mailbox.selectedMessage?.id === message.id ? 'bg-accent' : ''}"
        on:click={() => handleMessageSelect(message)}
      >
        <div class="font-semibold truncate">{message.from}</div>
        <div class="truncate {message.is_read ? 'text-muted-foreground' : 'font-medium'}">{message.subject || '(No Subject)'}</div>
        <div class="text-xs text-muted-foreground">
          {new Date(message.date * 1000).toLocaleString()}
        </div>
      </button>
    {/each}
  {/if}
</div>