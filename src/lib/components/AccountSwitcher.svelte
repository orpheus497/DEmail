'''<script lang="ts">
  import { onMount } from "svelte";
  import { mailbox } from "$lib/stores/mailboxStore";
  import * as Select from "$lib/components/ui/select";
  import { CaretSort, Check } from "radix-icons-svelte";
  import { Button } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";

  let accounts = mailbox.accounts;

  onMount(() => {
    mailbox.fetchAccounts();
  });

  function handleAccountChange(accountId: number) {
    const account = $mailbox.accounts.find((acc) => acc.id === accountId);
    if (account) {
      mailbox.selectAccount(account);
    }
  }
</script>

<Select.Root onValueChange={(val) => handleAccountChange(Number(val))}>
  <Select.Trigger class="w-full">
    <Select.Value placeholder="Select an account" />
  </Select.Trigger>
  <Select.Content>
    {#each $accounts as account}
      <Select.Item value={account.id.toString()}>
        <div class="flex items-center">
          <span>{account.display_name}</span>
          <span class="ml-auto text-muted-foreground">{account.email_address}</span>
        </div>
      </Select.Item>
    {/each}
  </Select.Content>
</Select.Root>
''