'''<script lang="ts">
  import { mailbox } from "$lib/stores/mailboxStore";
  import type { Folder } from "$lib/types";

  let folders = mailbox.folders;

  function handleFolderSelect(folder: Folder) {
    mailbox.selectFolder(folder);
  }
</script>

<div class="flex flex-col gap-2">
  {#each $folders as folder}
    <button
      class="flex items-center gap-2 rounded-md p-2 text-sm font-medium hover:bg-muted"
      on:click={() => handleFolderSelect(folder)}
    >
      <span>{folder.name}</span>
    </button>
  {/each}
</div>
''