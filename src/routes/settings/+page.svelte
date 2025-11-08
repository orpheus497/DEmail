<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import ThemeToggle from "$lib/components/ThemeToggle.svelte";
  import SignatureManager from "$lib/components/SignatureManager.svelte";
  import {
    getOauthProviderConfigs,
    saveOauthProviderConfig,
    addAccount,
    getAccounts,
  } from "$lib/services/api";
  import type { OAuthProviderConfig, Account } from "$lib/types";
  import { open } from "@tauri-apps/api/shell";

  let providerConfigs: Record<string, OAuthProviderConfig> = {
    google: { client_id: "", client_secret: "" },
    microsoft: { client_id: "", client_secret: "" },
  };
  let emailToAdd = "";
  let saveStatus: string | null = null;
  let accounts: Account[] = [];
  let selectedAccountForSignatures: Account | null = null;

  onMount(async () => {
    try {
      const configs = await getOauthProviderConfigs();
      providerConfigs = { ...providerConfigs, ...configs };

      // Load accounts for signature management
      accounts = await getAccounts();
      if (accounts.length > 0) {
        selectedAccountForSignatures = accounts[0];
      }
    } catch (e) {
      console.error("Failed to load configs:", e);
    }
  });

  async function handleSave(provider: string) {
    try {
      const config = providerConfigs[provider];
      if (config && config.client_id && config.client_secret) {
        await saveOauthProviderConfig(provider, config);
        saveStatus = `${provider} configuration saved successfully!`;
        setTimeout(() => {
          saveStatus = null;
        }, 3000);
      }
    } catch (e) {
      saveStatus = `Failed to save ${provider} configuration`;
    }
  }

  async function handleAddAccount() {
    if (!emailToAdd) return;
    try {
      const authUrl = await addAccount(emailToAdd);
      await open(authUrl);
    } catch (e) {
      console.error("Failed to add account:", e);
    }
  }
</script>

<div class="p-8 max-w-3xl">
  <div class="flex items-center justify-between mb-8">
    <h1 class="text-3xl font-bold">Settings</h1>
    <Button variant="outline" on:click={() => goto("/")}>Back to Inbox</Button>
  </div>

  {#if saveStatus}
    <div class="mb-4 p-4 bg-accent rounded-md">
      {saveStatus}
    </div>
  {/if}

  <div class="space-y-8">
    <section>
      <h2 class="text-xl font-semibold mb-4">Appearance</h2>
      <p class="text-sm text-muted-foreground mb-4">
        Customize the look and feel of DEmail.
      </p>
      <div class="border rounded-lg p-6">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="font-medium">Theme</h3>
            <p class="text-sm text-muted-foreground">Toggle between light and dark mode</p>
          </div>
          <ThemeToggle />
        </div>
      </div>
    </section>

    <section>
      <h2 class="text-xl font-semibold mb-4">OAuth Provider Configuration</h2>
      <p class="text-sm text-muted-foreground mb-6">
        Configure OAuth credentials for your email providers. You need to create OAuth applications in Google Cloud Console or Azure Portal.
      </p>

      <div class="space-y-6">
        <div class="border rounded-lg p-6">
          <h3 class="font-medium text-lg mb-4">Google</h3>
          <div class="space-y-4">
            <div>
              <Label for="google-client-id">Client ID</Label>
              <Input
                id="google-client-id"
                bind:value={providerConfigs.google.client_id}
                placeholder="Enter Google OAuth Client ID"
                class="mt-1"
              />
            </div>
            <div>
              <Label for="google-client-secret">Client Secret</Label>
              <Input
                id="google-client-secret"
                type="password"
                bind:value={providerConfigs.google.client_secret}
                placeholder="Enter Google OAuth Client Secret"
                class="mt-1"
              />
            </div>
            <Button on:click={() => handleSave("google")}>Save Google Config</Button>
          </div>
        </div>

        <div class="border rounded-lg p-6">
          <h3 class="font-medium text-lg mb-4">Microsoft</h3>
          <div class="space-y-4">
            <div>
              <Label for="microsoft-client-id">Client ID</Label>
              <Input
                id="microsoft-client-id"
                bind:value={providerConfigs.microsoft.client_id}
                placeholder="Enter Microsoft OAuth Client ID"
                class="mt-1"
              />
            </div>
            <div>
              <Label for="microsoft-client-secret">Client Secret</Label>
              <Input
                id="microsoft-client-secret"
                type="password"
                bind:value={providerConfigs.microsoft.client_secret}
                placeholder="Enter Microsoft OAuth Client Secret"
                class="mt-1"
              />
            </div>
            <Button on:click={() => handleSave("microsoft")}>Save Microsoft Config</Button>
          </div>
        </div>
      </div>
    </section>

    <section>
      <h2 class="text-xl font-semibold mb-4">Add Email Account</h2>
      <p class="text-sm text-muted-foreground mb-4">
        Add a new email account to DEmail. Make sure you've configured the OAuth provider above first.
      </p>
      <div class="flex gap-2">
        <Input
          bind:value={emailToAdd}
          placeholder="email@example.com"
          type="email"
        />
        <Button on:click={handleAddAccount}>Add Account</Button>
      </div>
    </section>

    <!-- Phase 6: Email Signatures -->
    {#if accounts.length > 0}
      <section>
        <h2 class="text-xl font-semibold mb-4">Email Signatures</h2>
        <p class="text-sm text-muted-foreground mb-4">
          Manage email signatures for your accounts. Signatures will be automatically added to your emails.
        </p>

        {#if accounts.length > 1}
          <div class="mb-4">
            <Label for="account-select">Select Account</Label>
            <select
              id="account-select"
              bind:value={selectedAccountForSignatures}
              class="mt-1 w-full px-3 py-2 rounded-md border border-input bg-background"
            >
              {#each accounts as account}
                <option value={account}>{account.email}</option>
              {/each}
            </select>
          </div>
        {/if}

        {#if selectedAccountForSignatures}
          <SignatureManager accountId={selectedAccountForSignatures.id} />
        {/if}
      </section>
    {/if}
  </div>
</div>