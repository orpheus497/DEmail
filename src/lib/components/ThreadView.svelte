<script lang="ts">
  import { mailbox } from '$lib/stores/mailboxStore';
  import type { Message } from '$lib/types';
  import { Paperclip, Star, ChevronDown, ChevronUp, X } from 'lucide-svelte';
  import Button from '$lib/components/ui/button/index.svelte';
  import { createEventDispatcher } from 'svelte';

  export let threadId: number;
  export let visible: boolean = false;

  const dispatch = createEventDispatcher<{ close: void }>();

  let messages: Message[] = [];
  let loading = true;
  let error: string | null = null;
  let expandedMessageIds = new Set<number>();

  // Load thread messages when visible
  $: if (visible && threadId) {
    loadThread();
  }

  async function loadThread() {
    loading = true;
    error = null;

    try {
      const { getThreadMessages } = await import('$lib/services/api');
      messages = await getThreadMessages(threadId);

      // Expand the most recent message by default
      if (messages.length > 0) {
        expandedMessageIds.add(messages[messages.length - 1].id);
      }
    } catch (e) {
      error = `Failed to load thread: ${String(e)}`;
      messages = [];
    } finally {
      loading = false;
    }
  }

  function toggleExpand(messageId: number) {
    if (expandedMessageIds.has(messageId)) {
      expandedMessageIds.delete(messageId);
    } else {
      expandedMessageIds.add(messageId);
    }
    expandedMessageIds = expandedMessageIds; // Trigger reactivity
  }

  function handleClose() {
    visible = false;
    dispatch('close');
  }

  function formatDate(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffDays = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));

    if (diffDays === 0) {
      return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } else if (diffDays === 1) {
      return 'Yesterday ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } else if (diffDays < 7) {
      return (
        date.toLocaleDateString([], { weekday: 'short' }) +
        ' ' +
        date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
      );
    } else {
      return (
        date.toLocaleDateString() +
        ' ' +
        date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
      );
    }
  }

  function handleStarMessage(messageId: number, isStarred: boolean) {
    if (isStarred) {
      mailbox.unstarMessage(messageId);
    } else {
      mailbox.starMessage(messageId);
    }

    // Update local state
    messages = messages.map((m) => (m.id === messageId ? { ...m, is_starred: !isStarred } : m));
  }
</script>

{#if visible}
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
    on:click={handleClose}
  >
    <div
      class="bg-background rounded-lg shadow-lg max-w-4xl w-full max-h-[90vh] flex flex-col"
      on:click|stopPropagation
    >
      <div class="flex items-center justify-between p-6 border-b">
        <h2 class="text-2xl font-bold">
          Conversation ({messages.length} message{messages.length !== 1 ? 's' : ''})
        </h2>
        <Button variant="ghost" size="sm" on:click={handleClose}>
          <X class="h-4 w-4" />
        </Button>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        {#if loading}
          <div class="flex items-center justify-center p-8">
            <div class="text-muted-foreground">Loading conversation...</div>
          </div>
        {:else if error}
          <div class="p-4 bg-destructive/10 text-destructive rounded-md">
            {error}
          </div>
        {:else if messages.length === 0}
          <div class="flex items-center justify-center p-8 text-muted-foreground">
            No messages in this conversation
          </div>
        {:else}
          <div class="space-y-4">
            {#each messages as message, index}
              {@const isExpanded = expandedMessageIds.has(message.id)}
              {@const isFirst = index === 0}
              {@const isLast = index === messages.length - 1}

              <div
                class="border rounded-lg overflow-hidden {isExpanded ? 'ring-2 ring-primary' : ''}"
              >
                <button
                  class="w-full p-4 text-left hover:bg-accent transition-colors flex items-center justify-between gap-4"
                  on:click={() => toggleExpand(message.id)}
                >
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 mb-1">
                      <span class="font-semibold truncate">{message.from_header}</span>
                      {#if message.is_starred}
                        <Star class="h-4 w-4 text-yellow-500 fill-yellow-500 flex-shrink-0" />
                      {/if}
                      {#if isFirst}
                        <span class="text-xs bg-primary/10 text-primary px-2 py-0.5 rounded"
                          >First</span
                        >
                      {/if}
                      {#if isLast}
                        <span class="text-xs bg-accent px-2 py-0.5 rounded">Latest</span>
                      {/if}
                    </div>
                    <div class="flex items-center gap-2 text-sm text-muted-foreground">
                      <span>To: {message.to_header}</span>
                      <span>•</span>
                      <span>{formatDate(message.date)}</span>
                    </div>
                    {#if !isExpanded}
                      <div class="text-sm text-muted-foreground mt-1 truncate">
                        {message.subject || '(No Subject)'}
                      </div>
                    {/if}
                  </div>
                  <div class="flex items-center gap-2 flex-shrink-0">
                    {#if message.has_attachments}
                      <Paperclip class="h-4 w-4 text-muted-foreground" />
                    {/if}
                    {#if isExpanded}
                      <ChevronUp class="h-5 w-5" />
                    {:else}
                      <ChevronDown class="h-5 w-5" />
                    {/if}
                  </div>
                </button>

                {#if isExpanded}
                  <div class="border-t p-4 bg-accent/5">
                    <div class="mb-4 flex items-center justify-between">
                      <div class="text-lg font-bold">{message.subject || '(No Subject)'}</div>
                      <Button
                        variant="ghost"
                        size="sm"
                        on:click={() => handleStarMessage(message.id, message.is_starred)}
                      >
                        <Star
                          class="h-5 w-5 {message.is_starred
                            ? 'text-yellow-500 fill-yellow-500'
                            : ''}"
                        />
                      </Button>
                    </div>

                    {#if message.cc_header}
                      <div class="text-sm text-muted-foreground mb-2">
                        Cc: {message.cc_header}
                      </div>
                    {/if}

                    <div class="prose prose-sm max-w-none mt-4">
                      {#if message.body_html}
                        {@html message.body_html}
                      {:else if message.body_plain}
                        <pre class="whitespace-pre-wrap font-sans">{message.body_plain}</pre>
                      {:else}
                        <div class="text-muted-foreground italic">No content available</div>
                      {/if}
                    </div>

                    {#if message.attachments && message.attachments.length > 0}
                      <div class="mt-4 pt-4 border-t">
                        <div class="flex items-center gap-2 mb-2">
                          <Paperclip class="h-4 w-4" />
                          <span class="font-semibold text-sm">
                            {message.attachments.length} Attachment{message.attachments.length > 1
                              ? 's'
                              : ''}
                          </span>
                        </div>
                        <div class="space-y-2">
                          {#each message.attachments as attachment}
                            <div
                              class="text-sm p-2 bg-background border rounded flex items-center gap-2"
                            >
                              <Paperclip class="h-3 w-3 text-muted-foreground" />
                              <span>{attachment.filename}</span>
                              <span class="text-muted-foreground"
                                >({Math.round(attachment.size_bytes / 1024)} KB)</span
                              >
                            </div>
                          {/each}
                        </div>
                      </div>
                    {/if}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
