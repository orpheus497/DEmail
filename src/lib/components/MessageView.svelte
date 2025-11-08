<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
</script>

<div class="p-4 h-full overflow-auto">
  {#if $mailbox.selectedMessage}
    <div class="flex flex-col gap-4">
      <div class="flex items-center justify-between border-b pb-4">
        <div class="flex flex-col gap-1">
          <div class="font-semibold text-lg">{$mailbox.selectedMessage.from_header}</div>
          <div class="text-sm text-muted-foreground">To: {$mailbox.selectedMessage.to_header}</div>
          {#if $mailbox.selectedMessage.cc_header}
            <div class="text-sm text-muted-foreground">Cc: {$mailbox.selectedMessage.cc_header}</div>
          {/if}
        </div>
        <div class="text-sm text-muted-foreground">
          {new Date($mailbox.selectedMessage.date * 1000).toLocaleString()}
        </div>
      </div>
      <div class="text-2xl font-bold">{$mailbox.selectedMessage.subject || '(No Subject)'}</div>
      <div class="prose prose-sm max-w-none">
        {#if $mailbox.selectedMessage.body_html}
          {@html $mailbox.selectedMessage.body_html}
        {:else if $mailbox.selectedMessage.body_plain}
          <pre class="whitespace-pre-wrap font-sans">{$mailbox.selectedMessage.body_plain}</pre>
        {:else}
          <div class="text-muted-foreground italic">No content available</div>
        {/if}
      </div>
    </div>
  {:else}
    <div class="flex h-full items-center justify-center">
      <div class="text-muted-foreground">Select a message to read</div>
    </div>
  {/if}
</div>