<script lang="ts">
  import { X } from "lucide-svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ close: void }>();

  export let visible = false;

  type Shortcut = {
    keys: string;
    description: string;
  };

  type ShortcutGroup = {
    title: string;
    shortcuts: Shortcut[];
  };

  const shortcutGroups: ShortcutGroup[] = [
    {
      title: "Navigation",
      shortcuts: [
        { keys: "j / ↓", description: "Next message" },
        { keys: "k / ↑", description: "Previous message" },
        { keys: "Enter", description: "Open selected message" },
        { keys: "g i", description: "Go to inbox" },
        { keys: "/", description: "Search messages" },
      ],
    },
    {
      title: "Actions",
      shortcuts: [
        { keys: "s", description: "Star/unstar message" },
        { keys: "e", description: "Archive message" },
        { keys: "r", description: "Reply to message" },
        { keys: "a", description: "Reply all" },
        { keys: "f", description: "Forward message" },
        { keys: "c", description: "Compose new message" },
      ],
    },
    {
      title: "Selection",
      shortcuts: [
        { keys: "x", description: "Select/deselect message" },
        { keys: "* a", description: "Select all messages" },
        { keys: "* n", description: "Deselect all messages" },
        { keys: "Ctrl/Cmd + Click", description: "Multi-select" },
        { keys: "Shift + Click", description: "Range select" },
      ],
    },
    {
      title: "General",
      shortcuts: [
        { keys: "?", description: "Show keyboard shortcuts" },
        { keys: "Escape", description: "Close dialog / Clear selection" },
        { keys: "Ctrl/Cmd + R", description: "Refresh" },
      ],
    },
  ];

  function handleClose() {
    visible = false;
    dispatch('close');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      handleClose();
    }
  }
</script>

{#if visible}
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
    on:click={handleClose}
    on:keydown={handleKeydown}
    role="dialog"
    aria-modal="true"
    aria-labelledby="shortcuts-title"
  >
    <div
      class="bg-card border rounded-lg shadow-lg max-w-2xl w-full max-h-[80vh] overflow-auto"
      on:click|stopPropagation
    >
      <div class="sticky top-0 bg-card border-b p-4 flex items-center justify-between">
        <h2 id="shortcuts-title" class="text-lg font-semibold">Keyboard Shortcuts</h2>
        <Button
          variant="ghost"
          size="sm"
          on:click={handleClose}
          aria-label="Close"
        >
          <X class="h-4 w-4" />
        </Button>
      </div>

      <div class="p-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          {#each shortcutGroups as group}
            <div>
              <h3 class="font-semibold mb-3 text-sm text-muted-foreground uppercase tracking-wide">
                {group.title}
              </h3>
              <div class="space-y-2">
                {#each group.shortcuts as shortcut}
                  <div class="flex items-center justify-between">
                    <span class="text-sm">{shortcut.description}</span>
                    <kbd class="px-2 py-1 text-xs font-mono bg-muted border rounded">
                      {shortcut.keys}
                    </kbd>
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="border-t p-4 bg-muted/30">
        <p class="text-xs text-muted-foreground text-center">
          Press <kbd class="px-1 py-0.5 text-xs font-mono bg-background border rounded">?</kbd> anytime to view this help
        </p>
      </div>
    </div>
  </div>
{/if}
