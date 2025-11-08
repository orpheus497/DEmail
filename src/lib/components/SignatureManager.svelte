<script lang="ts">
  import { onMount } from 'svelte';
  import { getSignatures, saveSignature, deleteSignature } from '$lib/services/api';
  import type { EmailSignature } from '$lib/types';
  import Button from '$lib/components/ui/button/index.svelte';
  import Input from '$lib/components/ui/input/index.svelte';
  import Label from '$lib/components/ui/label/index.svelte';
  import { Plus, Trash2, Save, Star } from 'lucide-svelte';

  export let accountId: number;

  let signatures: EmailSignature[] = [];
  let loading = false;
  let error: string | null = null;
  let editingSignature: EmailSignature | null = null;
  let showEditor = false;

  let signatureName = '';
  let signatureContentPlain = '';
  let signatureContentHtml = '';
  let isDefault = false;

  onMount(async () => {
    await loadSignatures();
  });

  async function loadSignatures() {
    loading = true;
    error = null;
    try {
      signatures = await getSignatures(accountId);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleSaveSignature() {
    error = null;
    try {
      const signature: EmailSignature = {
        id: editingSignature?.id || 0,
        account_id: accountId,
        name: signatureName,
        content_plain: signatureContentPlain,
        content_html: signatureContentHtml || signatureContentPlain,
        is_default: isDefault,
      };

      await saveSignature(signature);
      await loadSignatures();
      handleCloseEditor();
    } catch (e) {
      error = String(e);
    }
  }

  async function handleDeleteSignature(signatureId: number) {
    try {
      await deleteSignature(signatureId);
      await loadSignatures();
    } catch (e) {
      error = String(e);
    }
  }

  function handleEditSignature(signature: EmailSignature) {
    editingSignature = signature;
    signatureName = signature.name;
    signatureContentPlain = signature.content_plain;
    signatureContentHtml = signature.content_html;
    isDefault = signature.is_default;
    showEditor = true;
  }

  function handleNewSignature() {
    editingSignature = null;
    signatureName = '';
    signatureContentPlain = '';
    signatureContentHtml = '';
    isDefault = false;
    showEditor = true;
  }

  function handleCloseEditor() {
    showEditor = false;
    editingSignature = null;
    signatureName = '';
    signatureContentPlain = '';
    signatureContentHtml = '';
    isDefault = false;
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <h3 class="text-lg font-semibold">Email Signatures</h3>
    <Button size="sm" on:click={handleNewSignature}>
      <Plus class="h-4 w-4 mr-2" />
      Add Signature
    </Button>
  </div>

  {#if error}
    <div class="p-3 bg-destructive/10 text-destructive rounded-md text-sm">
      {error}
    </div>
  {/if}

  {#if loading}
    <div class="flex items-center justify-center p-8">
      <p class="text-muted-foreground">Loading signatures...</p>
    </div>
  {:else if signatures.length === 0 && !showEditor}
    <div class="flex items-center justify-center p-8 border border-dashed rounded-lg">
      <p class="text-muted-foreground">No signatures created yet</p>
    </div>
  {:else if !showEditor}
    <div class="space-y-2">
      {#each signatures as signature}
        <div class="border rounded-lg p-4">
          <div class="flex items-start justify-between gap-4">
            <div class="flex-1">
              <div class="flex items-center gap-2">
                <h4 class="font-semibold">{signature.name}</h4>
                {#if signature.is_default}
                  <Star class="h-4 w-4 fill-yellow-400 text-yellow-400" />
                {/if}
              </div>
              <p class="text-sm text-muted-foreground mt-2 whitespace-pre-wrap">
                {signature.content_plain}
              </p>
            </div>
            <div class="flex gap-2">
              <Button
                variant="ghost"
                size="sm"
                on:click={() => handleEditSignature(signature)}
              >
                Edit
              </Button>
              <Button
                variant="ghost"
                size="sm"
                on:click={() => handleDeleteSignature(signature.id)}
              >
                <Trash2 class="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if showEditor}
    <div class="border rounded-lg p-6 space-y-4">
      <h4 class="text-lg font-semibold">
        {editingSignature ? 'Edit Signature' : 'New Signature'}
      </h4>

      <div>
        <Label for="sig-name">Signature Name *</Label>
        <Input
          id="sig-name"
          bind:value={signatureName}
          placeholder="e.g., Professional, Personal"
          required
          class="mt-1"
        />
      </div>

      <div>
        <Label for="sig-content">Signature Content *</Label>
        <textarea
          id="sig-content"
          bind:value={signatureContentPlain}
          rows="6"
          placeholder="Your signature text..."
          required
          class="mt-1 w-full p-3 border border-input rounded-md bg-background resize-none focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>

      <div class="flex items-center gap-2">
        <input
          type="checkbox"
          id="sig-default"
          bind:checked={isDefault}
          class="h-4 w-4 rounded border-gray-300"
        />
        <Label for="sig-default">Set as default signature</Label>
      </div>

      <div class="flex justify-end gap-2">
        <Button variant="outline" on:click={handleCloseEditor}>
          Cancel
        </Button>
        <Button on:click={handleSaveSignature}>
          <Save class="h-4 w-4 mr-2" />
          Save Signature
        </Button>
      </div>
    </div>
  {/if}
</div>
