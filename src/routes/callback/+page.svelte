<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { handleCallback } from '$lib/services/api';

  let status = 'Processing authentication...';
  let error: string | null = null;

  onMount(async () => {
    const code = $page.url.searchParams.get('code');
    const state = $page.url.searchParams.get('state');

    if (!code || !state) {
      error = 'Missing authorization code or state parameter';
      return;
    }

    try {
      const account = await handleCallback(code, state);
      status = `Successfully added account: ${account.email_address}`;

      setTimeout(() => {
        goto('/');
      }, 2000);
    } catch (e) {
      error = `Failed to complete authentication: ${e}`;
    }
  });
</script>

<div class="flex flex-col items-center justify-center h-screen gap-4 p-8">
  <div class="text-center space-y-4 max-w-md">
    <h1 class="text-2xl font-bold">OAuth Callback</h1>

    {#if error}
      <div class="p-4 border border-destructive rounded-md text-destructive">
        {error}
      </div>
      <p class="text-sm text-muted-foreground">Please close this window and try again.</p>
    {:else}
      <div class="p-4 border rounded-md">
        {status}
      </div>
      <p class="text-sm text-muted-foreground">You will be redirected shortly...</p>
    {/if}
  </div>
</div>
