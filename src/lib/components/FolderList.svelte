<script lang="ts">
  import { mailbox } from '$lib/stores/mailboxStore';
  import type { Folder } from '$lib/types';

  // Phase 5: Use pagination when selecting folders
  function handleFolderSelect(folder: Folder) {
    mailbox.selectFolderWithPagination(folder);
  }
</script>

<div class="flex flex-col gap-1">
  {#each $mailbox.folders as folder}
    <button
      class="flex items-center gap-2 rounded-md p-2 text-sm font-medium hover:bg-accent text-left transition-colors {$mailbox
        .selectedFolder?.id === folder.id
        ? 'bg-accent'
        : ''}"
      on:click={() => handleFolderSelect(folder)}
    >
      <span>{folder.name}</span>
    </button>
  {/each}
</div>
