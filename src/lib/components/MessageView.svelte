'''<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";

  let selectedMessage = mailbox.selectedMessage;
</script>

<div class="p-4">
  {#if $selectedMessage}
    <div class="flex flex-col gap-4">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div class="flex flex-col">
            <div class="font-semibold">{$selectedMessage.from_header}</div>
            <div class="text-sm text-muted-foreground">{$selectedMessage.to_header}</div>
          </div>
        </div>
        <div class="text-sm text-muted-foreground">
          {new Date($selectedMessage.date * 1000).toLocaleString()}
        </div>
      </div>
      <div class="text-2xl font-bold">{$selectedMessage.subject}</div>
      <div class="prose max-w-none">
        {@html $selectedMessage.body_html}
      </div>
    </div>
  {:else}
    <div class="flex h-full items-center justify-center">
      <div class="text-muted-foreground">Select a message to read</div>
    </div>
  {/if}
</div>
''