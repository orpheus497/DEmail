<script lang="ts">
  import { onMount } from 'svelte';
  import { mailbox } from '$lib/stores/mailboxStore';
  import * as Select from '$lib/components/ui/select';
  import Button from '$lib/components/ui/button/index.svelte';
  import { cn } from '$lib/utils';

  onMount(() => {
    mailbox.fetchAccounts();
  });

  function handleAccountChange(value: string | undefined) {
    if (!value) return;
    const accountId = Number(value);
    const account = $mailbox.accounts.find((acc) => acc.id === accountId);
    if (account) {
      mailbox.selectAccount(account);
    }
  }
</script>

<Select.Root onValueChange={handleAccountChange}>
  <Select.Trigger class="w-full">
    <Select.Value placeholder="Select an account" />
  </Select.Trigger>
  <Select.Content>
    {#each $mailbox.accounts as account}
      <Select.Item value={account.id.toString()}>
        <div class="flex items-center gap-2">
          <span>{account.display_name}</span>
          <span class="text-xs text-muted-foreground">{account.email_address}</span>
        </div>
      </Select.Item>
    {/each}
  </Select.Content>
</Select.Root>
