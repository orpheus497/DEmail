'''<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import type { MessageHeader } from "$lib/types";

  let messages = mailbox.messages;

  function handleMessageSelect(message: MessageHeader) {
    mailbox.selectMessage(message);
  }
</script>

<div class="flex flex-col gap-2 p-4">
  {#each $messages as message}
    <button
      class="flex flex-col gap-1 rounded-md border p-3 text-left text-sm hover:bg-muted"
      on:click={() => handleMessageSelect(message)}
    >
      <div class="font-semibold">{message.from}</div>
      <div>{message.subject}</div>
      <div class="text-xs text-muted-foreground">
        {new Date(message.date * 1000).toLocaleString()}
      </div>
    </button>
  {/each}
</div>
''