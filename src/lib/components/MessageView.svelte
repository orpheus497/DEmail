<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import { onMount } from "svelte";
  import { Paperclip } from "lucide-svelte";

  let currentMessageId: number | null = null;

  $: {
    if ($mailbox.selectedMessage && $mailbox.selectedMessage.id !== currentMessageId) {
      currentMessageId = $mailbox.selectedMessage.id;
      if (!$mailbox.selectedMessage.is_read) {
        mailbox.markRead($mailbox.selectedMessage.id);
      }
    }
  }

  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  }
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

      {#if $mailbox.selectedMessage.attachments && $mailbox.selectedMessage.attachments.length > 0}
        <div class="border-t pt-4">
          <div class="flex items-center gap-2 mb-2">
            <Paperclip class="h-4 w-4" />
            <span class="font-semibold">{$mailbox.selectedMessage.attachments.length} Attachment{$mailbox.selectedMessage.attachments.length > 1 ? 's' : ''}</span>
          </div>
          <div class="space-y-2">
            {#each $mailbox.selectedMessage.attachments as attachment}
              <div class="flex items-center justify-between p-2 border rounded-md">
                <div class="flex items-center gap-2">
                  <Paperclip class="h-3 w-3 text-muted-foreground" />
                  <div class="flex flex-col">
                    <span class="text-sm font-medium">{attachment.filename}</span>
                    <span class="text-xs text-muted-foreground">{formatFileSize(attachment.size_bytes)} • {attachment.mime_type}</span>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="flex h-full items-center justify-center">
      <div class="text-muted-foreground">Select a message to read</div>
    </div>
  {/if}
</div>