<script lang="ts">
  import { searchContacts } from '$lib/services/api';
  import type { Contact } from '$lib/types';
  import { createEventDispatcher } from 'svelte';
  import Input from '$lib/components/ui/input/index.svelte';
  import Label from '$lib/components/ui/label/index.svelte';
  import { mailbox } from '$lib/stores/mailboxStore';

  export let value: string = '';
  export let label: string;
  export let placeholder: string = '';
  export let required: boolean = false;
  export let id: string;

  const dispatch = createEventDispatcher<{ select: Contact }>();

  let suggestions: Contact[] = [];
  let showSuggestions = false;
  let selectedIndex = -1;
  let searchTimeout: number | null = null;

  async function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    value = target.value;

    // Clear existing timeout
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }

    // Don't search if input is too short or no account selected
    if (value.length < 2 || !$mailbox.selectedAccount) {
      suggestions = [];
      showSuggestions = false;
      return;
    }

    // Debounce search
    searchTimeout = window.setTimeout(async () => {
      try {
        if ($mailbox.selectedAccount) {
          suggestions = await searchContacts($mailbox.selectedAccount.id, value);
          showSuggestions = suggestions.length > 0;
          selectedIndex = -1;
        }
      } catch (error) {
        console.error('Failed to search contacts:', error);
        suggestions = [];
        showSuggestions = false;
      }
    }, 300);
  }

  function selectContact(contact: Contact) {
    if (contact.name) {
      value = `${contact.name} <${contact.email}>`;
    } else {
      value = contact.email;
    }
    showSuggestions = false;
    dispatch('select', contact);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!showSuggestions || suggestions.length === 0) return;

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, suggestions.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, -1);
        break;
      case 'Enter':
        if (selectedIndex >= 0) {
          event.preventDefault();
          selectContact(suggestions[selectedIndex]);
        }
        break;
      case 'Escape':
        event.preventDefault();
        showSuggestions = false;
        selectedIndex = -1;
        break;
    }
  }

  function handleBlur() {
    // Delay hiding to allow click on suggestion
    setTimeout(() => {
      showSuggestions = false;
      selectedIndex = -1;
    }, 200);
  }

  function handleFocus() {
    if (suggestions.length > 0) {
      showSuggestions = true;
    }
  }
</script>

<div class="relative">
  <Label for={id}>{label}{required ? ' *' : ''}</Label>
  <input
    {id}
    type="text"
    bind:value
    {placeholder}
    {required}
    on:input={handleInput}
    on:keydown={handleKeydown}
    on:blur={handleBlur}
    on:focus={handleFocus}
    autocomplete="off"
    class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 mt-1"
  />

  {#if showSuggestions && suggestions.length > 0}
    <div
      class="absolute z-50 w-full mt-1 bg-popover border rounded-md shadow-lg max-h-60 overflow-y-auto"
      role="listbox"
    >
      {#each suggestions as contact, index}
        <button
          type="button"
          class="w-full px-4 py-2 text-left hover:bg-accent flex flex-col {selectedIndex === index
            ? 'bg-accent'
            : ''}"
          on:click={() => selectContact(contact)}
          role="option"
          aria-selected={selectedIndex === index}
        >
          <div class="font-medium">
            {contact.name || contact.email}
          </div>
          {#if contact.name}
            <div class="text-sm text-muted-foreground">{contact.email}</div>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>
