<script lang="ts">
  import { onMount } from 'svelte';
  import { getDrafts, deleteDraft } from '$lib/services/api';
  import type { Draft } from '$lib/types';
  import Button from '$lib/components/ui/button/index.svelte';
  import { FileEdit, Trash2 } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  export let accountId: number;

  let drafts: Draft[] = [];
  let loading = false;
  let error: string | null = null;

  const dispatch = createEventDispatcher<{ edit: Draft }>();

  onMount(async () => {
    await loadDrafts();
  });

  async function loadDrafts() {
    loading = true;
    error = null;
    try {
      drafts = await getDrafts(accountId);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleDeleteDraft(draftId: number) {
    try {
      await deleteDraft(draftId);
      await loadDrafts();
    } catch (e) {
      error = String(e);
    }
  }

  function handleEditDraft(draft: Draft) {
    dispatch('edit', draft);
  }

  function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString();
  }
</script>

<div class="flex flex-col h-full">
  <div class="p-4 border-b">
    <h2 class="text-xl font-bold">Drafts</h2>
  </div>

  <div class="flex-1 overflow-auto p-4">
    {#if loading}
      <div class="flex items-center justify-center p-8">
        <p class="text-muted-foreground">Loading drafts...</p>
      </div>
    {:else if error}
      <div class="p-4 bg-destructive/10 text-destructive rounded-md">
        Error: {error}
      </div>
    {:else if drafts.length === 0}
      <div class="flex items-center justify-center p-8">
        <p class="text-muted-foreground">No drafts saved</p>
      </div>
    {:else}
      <div class="space-y-2">
        {#each drafts as draft}
          <div class="border rounded-lg p-4 hover:bg-accent/50 transition-colors">
            <div class="flex items-start justify-between gap-4">
              <div class="flex-1 min-w-0">
                <h3 class="font-semibold truncate">{draft.subject || '(No Subject)'}</h3>
                <p class="text-sm text-muted-foreground">To: {draft.to_addresses}</p>
                <p class="text-xs text-muted-foreground mt-1">
                  Last saved: {formatDate(draft.updated_at)}
                </p>
              </div>
              <div class="flex gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  on:click={() => handleEditDraft(draft)}
                >
                  <FileEdit class="h-4 w-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  on:click={() => handleDeleteDraft(draft.id)}
                >
                  <Trash2 class="h-4 w-4" />
                </Button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
