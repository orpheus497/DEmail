'''<script lang="ts">
  import "../app.css";
  import * as Resizable from "$lib/components/ui/resizable";
  import AccountSwitcher from "$lib/components/AccountSwitcher.svelte";
  import FolderList from "$lib/components/FolderList.svelte";
  import MessageList from "$lib/components/MessageList.svelte";
  import MessageView from "$lib/components/MessageView.svelte";
</script>

<div class="h-screen">
  <Resizable.PaneGroup direction="horizontal">
    <Resizable.Pane defaultSize={20}>
      <div class="p-4">
        <AccountSwitcher />
      </div>
      <div class="p-4">
        <FolderList />
      </div>
    </Resizable.Pane>
    <Resizable.Handle />
    <Resizable.Pane defaultSize={30}>
      <MessageList />
    </Resizable.Pane>
    <Resizable.Handle />
    <Resizable.Pane defaultSize={50}>
      <MessageView />
      <slot />
    </Resizable.Pane>
''