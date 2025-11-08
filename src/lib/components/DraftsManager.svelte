<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import { getDrafts, deleteDraft } from '$lib/services/api';
  import type { Draft } from '$lib/types';
  import Button from '$lib/components/ui/button/index.svelte';
  import { FileText, Trash2, Mail, Loader2 } from 'lucide-svelte';

  export let accountId: number;
  export let visible = false;

  const dispatch = createEventDispatcher<{
    loadDraft: Draft;
    close: void;
  }>();

  let drafts: Draft[] = [];
  let loading = false;
  let error: string | null = null;

  $: if (visible && accountId) {
    loadDrafts();
  }

  async function loadDrafts() {
    loading = true;
    error = null;
    try {
      drafts = await getDrafts(accountId);
      // Sort by updated_at descending (most recent first)
      drafts.sort((a, b) => b.updated_at - a.updated_at);
    } catch (e) {
      error = `Failed to load drafts: ${String(e)}`;
    } finally {
      loading = false;
    }
  }

  async function handleDelete(draftId: number) {
    if (!confirm('Delete this draft?')) return;

    try {
      await deleteDraft(draftId);
      await loadDrafts();
    } catch (e) {
      error = `Failed to delete draft: ${String(e)}`;
    }
  }

  function handleLoadDraft(draft: Draft) {
    dispatch('loadDraft', draft);
    handleClose();
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
      return 'Today ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
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

  function truncateText(text: string, maxLength: number): string {
    if (!text) return '';
    return text.length > maxLength ? text.substring(0, maxLength) + '...' : text;
  }
</script>

{#if visible}
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
    on:click={handleClose}
    role="dialog"
    aria-modal="true"
    aria-labelledby="drafts-title"
  >
    <div
      class="bg-background rounded-lg shadow-lg max-w-3xl w-full max-h-[80vh] flex flex-col"
      on:click|stopPropagation
    >
      <div class="flex items-center justify-between p-6 border-b">
        <h2 id="drafts-title" class="text-2xl font-bold flex items-center gap-2">
          <FileText class="h-6 w-6" />
          Drafts ({drafts.length})
        </h2>
        <Button variant="ghost" size="sm" on:click={handleClose}>
          <Mail class="h-4 w-4" />
        </Button>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        {#if loading}
          <div class="flex items-center justify-center p-12">
            <div class="flex items-center gap-2 text-muted-foreground">
              <Loader2 class="h-5 w-5 animate-spin" />
              <span>Loading drafts...</span>
            </div>
          </div>
        {:else if error}
          <div class="p-4 bg-destructive/10 text-destructive rounded-md">
            {error}
          </div>
        {:else if drafts.length === 0}
          <div class="flex flex-col items-center justify-center p-12 text-center">
            <FileText class="h-16 w-16 text-muted-foreground mb-4" />
            <h3 class="text-lg font-semibold mb-2">No drafts</h3>
            <p class="text-sm text-muted-foreground">Your saved drafts will appear here</p>
          </div>
        {:else}
          <div class="space-y-3">
            {#each drafts as draft}
              <div class="border rounded-lg p-4 hover:bg-accent/50 transition-colors">
                <div class="flex items-start justify-between gap-4">
                  <button
                    class="flex-1 text-left space-y-2"
                    on:click={() => handleLoadDraft(draft)}
                  >
                    <div class="flex items-center gap-2">
                      <Mail class="h-4 w-4 text-muted-foreground flex-shrink-0" />
                      <div class="flex-1 min-w-0">
                        <div class="font-medium truncate">
                          {draft.subject || '(No Subject)'}
                        </div>
                        <div class="text-sm text-muted-foreground truncate">
                          To: {draft.to_header || 'Not specified'}
                        </div>
                      </div>
                    </div>

                    {#if draft.body_plain}
                      <div class="text-sm text-muted-foreground line-clamp-2">
                        {truncateText(draft.body_plain, 150)}
                      </div>
                    {/if}

                    <div class="flex items-center gap-4 text-xs text-muted-foreground">
                      <span>Updated: {formatDate(draft.updated_at)}</span>
                      {#if draft.cc_header}
                        <span>• CC: {draft.cc_header}</span>
                      {/if}
                      {#if draft.bcc_header}
                        <span>• BCC included</span>
                      {/if}
                    </div>
                  </button>

                  <Button
                    variant="ghost"
                    size="sm"
                    on:click={() => handleDelete(draft.id)}
                    title="Delete draft"
                  >
                    <Trash2 class="h-4 w-4" />
                  </Button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>

      <div class="border-t p-4 bg-muted/30">
        <p class="text-xs text-muted-foreground text-center">
          Click on a draft to continue editing
        </p>
      </div>
    </div>
  </div>
{/if}
