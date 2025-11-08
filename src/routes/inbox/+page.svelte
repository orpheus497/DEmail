<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { mailbox } from "$lib/stores/mailboxStore";
  import Button from "$lib/components/ui/button/index.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
  import AccountSwitcher from "$lib/components/AccountSwitcher.svelte";
  import FolderList from "$lib/components/FolderList.svelte";
  import MessageList from "$lib/components/MessageList.svelte";
  import MessageView from "$lib/components/MessageView.svelte";
  import SearchBar from "$lib/components/SearchBar.svelte";
  import ComposeEmail from "$lib/components/ComposeEmail.svelte";
  import BulkActionToolbar from "$lib/components/BulkActionToolbar.svelte";
  import KeyboardShortcutsHelp from "$lib/components/KeyboardShortcutsHelp.svelte";
  import { Settings, Pencil, RefreshCw, HelpCircle } from "lucide-svelte";

  let composeOpen = false;
  let composeMode: 'compose' | 'reply' | 'replyAll' | 'forward' = 'compose';
  let composeMessageId: number | null = null;
  let refreshing = false;
  let showKeyboardHelp = false;
  let messageListRef: MessageList;
  let searchBarRef: SearchBar;
  let selectedMessageIds: number[] = [];
  let currentMessageIndex = -1;

  // Phase 5: Memoized computed values
  $: hasMessages = $mailbox.messages.length > 0;
  $: hasSelectedAccount = !!$mailbox.selectedAccount;
  $: canRefresh = hasSelectedAccount && !refreshing;
  $: canCompose = hasSelectedAccount;

  onMount(() => {
    mailbox.fetchAccounts();
  });

  function handleCompose() {
    if ($mailbox.selectedAccount) {
      composeMode = 'compose';
      composeMessageId = null;
      composeOpen = true;
    }
  }

  function handleReply() {
    if ($mailbox.selectedMessage) {
      composeMode = 'reply';
      composeMessageId = $mailbox.selectedMessage.id;
      composeOpen = true;
    }
  }

  function handleReplyAll() {
    if ($mailbox.selectedMessage) {
      composeMode = 'replyAll';
      composeMessageId = $mailbox.selectedMessage.id;
      composeOpen = true;
    }
  }

  function handleForward() {
    if ($mailbox.selectedMessage) {
      composeMode = 'forward';
      composeMessageId = $mailbox.selectedMessage.id;
      composeOpen = true;
    }
  }

  async function handleRefresh() {
    if ($mailbox.selectedAccount && !refreshing) {
      refreshing = true;
      try {
        await mailbox.refreshAccount();
      } finally {
        refreshing = false;
      }
    }
  }

  function handleSearch(event: CustomEvent<string>) {
    mailbox.searchInMessages(event.detail);
  }

  function handleEmailSent() {
    composeOpen = false;
  }

  function handleSelectionChange(event: CustomEvent<number[]>) {
    selectedMessageIds = event.detail;
  }

  function handleClearSelection() {
    if (messageListRef) {
      messageListRef.clearSelection();
      selectedMessageIds = [];
    }
  }

  // Phase 3: Keyboard shortcuts
  function handleKeydown(event: KeyboardEvent) {
    // Ignore if user is typing in an input field
    const target = event.target as HTMLElement;
    if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
      return;
    }

    // Phase 5: Use memoized value
    if (!hasMessages && event.key !== '?' && event.key !== 'c') {
      return;
    }

    const messages = $mailbox.messages;

    switch (event.key) {
      case '?':
        event.preventDefault();
        showKeyboardHelp = !showKeyboardHelp;
        break;

      case 'c':
        event.preventDefault();
        handleCompose();
        break;

      case 'j':
      case 'ArrowDown':
        event.preventDefault();
        navigateNextMessage();
        break;

      case 'k':
      case 'ArrowUp':
        event.preventDefault();
        navigatePreviousMessage();
        break;

      case 's':
        event.preventDefault();
        toggleStarCurrentMessage();
        break;

      case 'r':
        if (event.ctrlKey || event.metaKey) {
          // Don't prevent default browser refresh
          return;
        }
        event.preventDefault();
        handleReply();
        break;

      case 'a':
        event.preventDefault();
        handleReplyAll();
        break;

      case 'f':
        event.preventDefault();
        handleForward();
        break;

      case 'e':
      case '#':
        event.preventDefault();
        deleteCurrentMessage();
        break;

      case 'x':
        event.preventDefault();
        toggleSelectCurrentMessage();
        break;

      case '/':
        event.preventDefault();
        focusSearch();
        break;

      case '*':
        // Wait for second key
        event.preventDefault();
        setTimeout(() => {
          const handleSecondKey = (e: KeyboardEvent) => {
            if (e.key === 'a') {
              e.preventDefault();
              selectAllMessages();
            } else if (e.key === 'n') {
              e.preventDefault();
              deselectAllMessages();
            }
            window.removeEventListener('keydown', handleSecondKey);
          };
          window.addEventListener('keydown', handleSecondKey, { once: true });
        }, 10);
        break;

      case 'Escape':
        event.preventDefault();
        if (showKeyboardHelp) {
          showKeyboardHelp = false;
        } else {
          handleClearSelection();
        }
        break;
    }
  }

  function navigateNextMessage() {
    const messages = $mailbox.messages;
    if (messages.length === 0) return;

    if (currentMessageIndex < messages.length - 1) {
      currentMessageIndex++;
      mailbox.selectMessage(messages[currentMessageIndex]);
    }
  }

  function navigatePreviousMessage() {
    const messages = $mailbox.messages;
    if (messages.length === 0) return;

    if (currentMessageIndex > 0) {
      currentMessageIndex--;
      mailbox.selectMessage(messages[currentMessageIndex]);
    } else if (currentMessageIndex === -1 && messages.length > 0) {
      currentMessageIndex = 0;
      mailbox.selectMessage(messages[0]);
    }
  }

  function toggleStarCurrentMessage() {
    if ($mailbox.selectedMessage) {
      if ($mailbox.selectedMessage.is_starred) {
        mailbox.unstarMessage($mailbox.selectedMessage.id);
      } else {
        mailbox.starMessage($mailbox.selectedMessage.id);
      }
    }
  }

  function deleteCurrentMessage() {
    if ($mailbox.selectedMessage) {
      if (confirm('Delete this message?')) {
        mailbox.deleteMessage($mailbox.selectedMessage.id);
      }
    }
  }

  function toggleSelectCurrentMessage() {
    if ($mailbox.selectedMessage && messageListRef) {
      const messageId = $mailbox.selectedMessage.id;
      const currentSelection = messageListRef.getSelectedMessages();

      if (currentSelection.includes(messageId)) {
        // Already selected, deselect it
        const newSelection = currentSelection.filter(id => id !== messageId);
        selectedMessageIds = newSelection;
      } else {
        // Not selected, select it
        selectedMessageIds = [...currentSelection, messageId];
      }
    }
  }

  function focusSearch() {
    // Find the search input and focus it
    const searchInput = document.querySelector('input[type="search"], input[placeholder*="Search"]') as HTMLInputElement;
    if (searchInput) {
      searchInput.focus();
    }
  }

  function selectAllMessages() {
    if (messageListRef && $mailbox.messages.length > 0) {
      selectedMessageIds = $mailbox.messages.map(m => m.id);
    }
  }

  function deselectAllMessages() {
    handleClearSelection();
  }

  // Update current message index when selection changes
  $: if ($mailbox.selectedMessage) {
    currentMessageIndex = $mailbox.messages.findIndex(m => m.id === $mailbox.selectedMessage?.id);
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="h-screen flex flex-col">
  <header class="border-b p-4 flex items-center justify-between">
    <h1 class="text-2xl font-bold">DEmail</h1>
    <div class="flex items-center gap-2">
      <Button
        variant="default"
        size="sm"
        on:click={handleCompose}
        disabled={!canCompose}
      >
        <Pencil class="h-4 w-4 mr-2" />
        Compose
      </Button>
      <Button
        variant="outline"
        size="sm"
        on:click={handleRefresh}
        disabled={!canRefresh}
      >
        <RefreshCw class="h-4 w-4 mr-2 {refreshing ? 'animate-spin' : ''}" />
        Refresh
      </Button>
      <Button
        variant="ghost"
        size="sm"
        on:click={() => showKeyboardHelp = !showKeyboardHelp}
        title="Keyboard shortcuts (?)"
      >
        <HelpCircle class="h-4 w-4" />
      </Button>
      <Button variant="outline" size="sm" on:click={() => goto("/settings")}>
        <Settings class="h-4 w-4 mr-2" />
        Settings
      </Button>
    </div>
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
          <div class="p-4 border-b space-y-3">
            <h2 class="text-lg font-semibold">Messages</h2>
            <SearchBar on:search={handleSearch} />
          </div>
          <div class="flex-1 overflow-auto">
            <MessageList
              bind:this={messageListRef}
              on:selectionChange={handleSelectionChange}
            />
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

  {#if $mailbox.selectedAccount}
    <ComposeEmail
      accountId={$mailbox.selectedAccount.id}
      bind:open={composeOpen}
      mode={composeMode}
      messageId={composeMessageId}
      on:sent={handleEmailSent}
    />
  {/if}

  <!-- Phase 3: Bulk action toolbar -->
  <BulkActionToolbar
    {selectedMessageIds}
    on:clearSelection={handleClearSelection}
  />

  <!-- Phase 3: Keyboard shortcuts help -->
  <KeyboardShortcutsHelp bind:visible={showKeyboardHelp} />
</div>
