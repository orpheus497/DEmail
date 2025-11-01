<script lang="ts">
  import FolderList from "./lib/components/FolderList.svelte";
  import MessageList from "./lib/components/MessageList.svelte";
  import MessageView from "./lib/components/MessageView.svelte";

  // These would be populated from the mailboxStore
  let folders = [];
  let messages = [];
  let selectedMessage = null;

</script>

<main>
  <div class="sidebar">
    <FolderList {folders} />
  </div>
  <div class="message-list">
    <MessageList {messages} />
  </div>
  <div class="message-view">
    <MessageView message={selectedMessage} />
  </div>
</main>

<style>
  main {
    display: flex;
    height: 100vh;
  }

  .sidebar {
    width: 240px;
    background-color: #e0e0e0;
    padding: 1rem;
  }

  .message-list {
    flex: 1;
    background-color: #f5f5f5;
    padding: 1rem;
    overflow-y: auto;
  }

  .message-view {
    flex: 2;
    padding: 1rem;
    overflow-y: auto;
  }
</style>
