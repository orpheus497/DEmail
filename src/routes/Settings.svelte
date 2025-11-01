'''<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import {
    getOauthProviderConfigs,
    saveOauthProviderConfig,
  } from "$lib/services/api";
  import type { OAuthProviderConfig } from "$lib/types";

  let providerConfigs: Record<string, OAuthProviderConfig> = {};

  onMount(async () => {
    providerConfigs = await getOauthProviderConfigs();
  });

  async function handleSave(provider: string) {
    const config = providerConfigs[provider];
    if (config) {
      await saveOauthProviderConfig(provider, config);
    }
  }
</script>

<div class="p-4">
  <h1 class="text-2xl font-bold">Settings</h1>

  <div class="mt-8">
    <h2 class="text-xl font-semibold">OAuth Providers</h2>

    <div class="mt-4 flex flex-col gap-4">
      <div>
        <h3 class="font-medium">Google</h3>
        <div class="mt-2 flex items-center gap-4">
          <Label for="google-client-id">Client ID</Label>
          <Input
            id="google-client-id"
            bind:value={providerConfigs.google?.client_id}
          />
        </div>
        <div class="mt-2 flex items-center gap-4">
          <Label for="google-client-secret">Client Secret</Label>
          <Input
            id="google-client-secret"
            type="password"
            bind:value={providerConfigs.google?.client_secret}
          />
        </div>
        <Button class="mt-2" on:click={() => handleSave("google")}>Save</Button>
      </div>

      <div>
        <h3 class="font-medium">Microsoft</h3>
        <div class="mt-2 flex items-center gap-4">
          <Label for="microsoft-client-id">Client ID</Label>
          <Input
            id="microsoft-client-id"
            bind:value={providerConfigs.microsoft?.client_id}
          />
        </div>
        <div class="mt-2 flex items-center gap-4">
          <Label for="microsoft-client-secret">Client Secret</Label>
          <Input
            id="microsoft-client-secret"
            type="password"
            bind:value={providerConfigs.microsoft?.client_secret}
          />
        </div>
        <Button class="mt-2" on:click={() => handleSave("microsoft")}>Save</Button>
      </div>
    </div>
  </div>
</div>
''