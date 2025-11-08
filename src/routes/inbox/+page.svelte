<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { mailbox } from "$lib/stores/mailboxStore";
  import { Button } from "$lib/components/ui/button";
  import * as Resizable from "$lib/components/ui/resizable";
  import AccountSwitcher from "$lib/components/AccountSwitcher.svelte";
  import FolderList from "$lib/components/FolderList.svelte";
  import MessageList from "$lib/components/MessageList.svelte";
  import MessageView from "$lib/components/MessageView.svelte";
  import { Settings } from "lucide-svelte";

  onMount(() => {
    mailbox.fetchAccounts();
  });
</script>

<div class="h-screen flex flex-col">
  <header class="border-b p-4 flex items-center justify-between">
    <h1 class="text-2xl font-bold">DEmail</h1>
    <Button variant="outline" size="sm" on:click={() => goto("/settings")}>
      <Settings class="h-4 w-4 mr-2" />
      Settings
    </Button>
  </header>

  {#if $mailbox.accounts.length === 0}
    <div class="flex flex-col items-center justify-center flex-1 gap-4 p-8">
      <div class="text-center space-y-4">
        <h2 class="text-2xl font-bold">Welcome to DEmail</h2>
        <p class="text-muted-foreground">
          Get started by configuring your email provider in Settings
        </p>
        <Button on:click={() => goto("/settings")}>Go to Settings</Button>
      </div>
    </div>
  {:else}
    <Resizable.PaneGroup direction="horizontal" class="flex-1">
      <Resizable.Pane defaultSize={20} minSize={15} maxSize={30}>
        <div class="flex flex-col h-full border-r">
          <div class="p-4 border-b">
            <AccountSwitcher />
          </div>
          <div class="flex-1 overflow-auto p-4">
            <h2 class="text-sm font-semibold mb-2">Folders</h2>
            <FolderList />
          </div>
        </div>
      </Resizable.Pane>

      <Resizable.Handle />

      <Resizable.Pane defaultSize={30} minSize={25} maxSize={50}>
        <div class="flex flex-col h-full border-r">
          <div class="p-4 border-b">
            <h2 class="text-lg font-semibold">Messages</h2>
          </div>
          <div class="flex-1 overflow-auto">
            <MessageList />
          </div>
        </div>
      </Resizable.Pane>

      <Resizable.Handle />

      <Resizable.Pane defaultSize={50}>
        <div class="flex flex-col h-full">
          <MessageView />
        </div>
      </Resizable.Pane>
    </Resizable.PaneGroup>
  {/if}

  {#if $mailbox.error}
    <div class="fixed bottom-4 right-4 p-4 bg-destructive text-destructive-foreground rounded-md shadow-lg max-w-md">
      Error: {$mailbox.error}
    </div>
  {/if}
</div>
