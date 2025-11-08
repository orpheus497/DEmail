<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import { onMount } from "svelte";
  import { Paperclip, Download, Star, MessageSquare } from "lucide-svelte";
  import { downloadAttachment, getThread } from "$lib/services/api";
  import { save } from "@tauri-apps/api/dialog";
  import Button from "$lib/components/ui/button/index.svelte";
  import ThreadView from "$lib/components/ThreadView.svelte";
  import type { Thread } from "$lib/types";

  let currentMessageId: number | null = null;
  let downloadingAttachmentId: number | null = null;
  let threadInfo: Thread | null = null;
  let loadingThread = false;
  let showThreadView = false;

  $: {
    if ($mailbox.selectedMessage && $mailbox.selectedMessage.id !== currentMessageId) {
      currentMessageId = $mailbox.selectedMessage.id;
      if (!$mailbox.selectedMessage.is_read) {
        mailbox.markRead($mailbox.selectedMessage.id);
      }
      // Load thread info if message is part of a thread
      loadThreadInfo();
    }
  }

  async function loadThreadInfo() {
    if ($mailbox.selectedMessage?.thread_id) {
      try {
        loadingThread = true;
        threadInfo = await getThread($mailbox.selectedMessage.thread_id);
      } catch (error) {
        console.error('Failed to load thread info:', error);
        threadInfo = null;
      } finally {
        loadingThread = false;
      }
    } else {
      threadInfo = null;
    }
  }

  function handleStarToggle() {
    if ($mailbox.selectedMessage) {
      if ($mailbox.selectedMessage.is_starred) {
        mailbox.unstarMessage($mailbox.selectedMessage.id);
      } else {
        mailbox.starMessage($mailbox.selectedMessage.id);
      }
    }
  }

  function handleViewThread() {
    if ($mailbox.selectedMessage?.thread_id) {
      showThreadView = true;
    }
  }

  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  }

  async function handleDownloadAttachment(attachmentId: number, filename: string) {
    try {
      downloadingAttachmentId = attachmentId;
      const savePath = await save({
        defaultPath: filename,
        filters: [{
          name: 'All Files',
          extensions: ['*']
        }]
      });

      if (savePath) {
        await downloadAttachment(attachmentId, savePath);
      }
    } catch (error) {
      console.error('Failed to download attachment:', error);
    } finally {
      downloadingAttachmentId = null;
    }
  }
</script>

<div class="p-4 h-full overflow-auto">
  {#if $mailbox.selectedMessage}
    <div class="flex flex-col gap-4">
      <div class="flex items-center justify-between border-b pb-4">
        <div class="flex flex-col gap-1 flex-1">
          <div class="font-semibold text-lg">{$mailbox.selectedMessage.from_header}</div>
          <div class="text-sm text-muted-foreground">To: {$mailbox.selectedMessage.to_header}</div>
          {#if $mailbox.selectedMessage.cc_header}
            <div class="text-sm text-muted-foreground">Cc: {$mailbox.selectedMessage.cc_header}</div>
          {/if}
        </div>
        <div class="flex items-center gap-2">
          <div class="text-sm text-muted-foreground">
            {new Date($mailbox.selectedMessage.date * 1000).toLocaleString()}
          </div>
          <Button
            variant="ghost"
            size="sm"
            on:click={handleStarToggle}
            aria-label={$mailbox.selectedMessage.is_starred ? 'Unstar message' : 'Star message'}
          >
            <Star
              class="h-5 w-5 {$mailbox.selectedMessage.is_starred ? 'text-yellow-500 fill-yellow-500' : ''}"
            />
          </Button>
        </div>
      </div>

      <div class="flex items-center justify-between">
        <div class="text-2xl font-bold">{$mailbox.selectedMessage.subject || '(No Subject)'}</div>
      </div>

      <!-- Threading indicator -->
      {#if threadInfo && threadInfo.message_count > 1}
        <div class="bg-accent rounded-md p-3 flex items-center justify-between">
          <div class="flex items-center gap-2">
            <MessageSquare class="h-4 w-4 text-muted-foreground" />
            <span class="text-sm">
              This message is part of a conversation with {threadInfo.message_count} message{threadInfo.message_count > 1 ? 's' : ''}
            </span>
          </div>
          <Button
            variant="outline"
            size="sm"
            on:click={handleViewThread}
          >
            View Conversation
          </Button>
        </div>
      {/if}
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
                <Button
                  variant="ghost"
                  size="sm"
                  on:click={() => handleDownloadAttachment(attachment.id, attachment.filename)}
                  disabled={downloadingAttachmentId === attachment.id}
                >
                  <Download class="h-4 w-4" />
                </Button>
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

<!-- Thread View Modal -->
{#if $mailbox.selectedMessage?.thread_id}
  <ThreadView
    threadId={$mailbox.selectedMessage.thread_id}
    bind:visible={showThreadView}
  />
{/if}