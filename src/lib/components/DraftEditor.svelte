<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/ui/button/index.svelte';
  import Input from '$lib/components/ui/input/index.svelte';
  import Label from '$lib/components/ui/label/index.svelte';
  import { saveDraft } from '$lib/services/api';
  import type { Draft } from '$lib/types';
  import { X, Save } from 'lucide-svelte';

  export let draft: Draft | null = null;
  export let accountId: number;
  export let open = false;

  let to = '';
  let cc = '';
  let bcc = '';
  let subject = '';
  let body = '';
  let saving = false;
  let error: string | null = null;

  const dispatch = createEventDispatcher<{ saved: void; close: void }>();

  $: if (draft) {
    to = draft.to_addresses;
    cc = draft.cc_addresses || '';
    bcc = draft.bcc_addresses || '';
    subject = draft.subject;
    body = draft.body_plain || '';
  }

  async function handleSave() {
    saving = true;
    error = null;

    try {
      const draftData: Draft = {
        id: draft?.id || 0,
        account_id: accountId,
        to_addresses: to,
        cc_addresses: cc || null,
        bcc_addresses: bcc || null,
        subject: subject,
        body_plain: body,
        body_html: null,
        created_at: draft?.created_at || Math.floor(Date.now() / 1000),
        updated_at: Math.floor(Date.now() / 1000),
      };

      await saveDraft(draftData);
      dispatch('saved');
      handleClose();
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }

  function handleClose() {
    resetForm();
    open = false;
    dispatch('close');
  }

  function resetForm() {
    to = '';
    cc = '';
    bcc = '';
    subject = '';
    body = '';
    error = null;
  }
</script>

{#if open}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
    <div class="bg-background rounded-lg shadow-lg max-w-2xl w-full max-h-[90vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b">
        <h2 class="text-2xl font-bold">Edit Draft</h2>
        <Button variant="ghost" size="sm" on:click={handleClose}>
          <X class="h-4 w-4" />
        </Button>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        {#if error}
          <div class="mb-4 p-3 bg-destructive/10 text-destructive rounded-md text-sm">
            {error}
          </div>
        {/if}

        <form on:submit|preventDefault={handleSave} class="space-y-4">
          <div>
            <Label for="to">To *</Label>
            <Input
              id="to"
              type="email"
              bind:value={to}
              placeholder="recipient@example.com"
              required
              class="mt-1"
            />
          </div>

          <div>
            <Label for="cc">CC</Label>
            <Input
              id="cc"
              type="email"
              bind:value={cc}
              placeholder="cc@example.com (optional)"
              class="mt-1"
            />
          </div>

          <div>
            <Label for="bcc">BCC</Label>
            <Input
              id="bcc"
              type="email"
              bind:value={bcc}
              placeholder="bcc@example.com (optional)"
              class="mt-1"
            />
          </div>

          <div>
            <Label for="subject">Subject *</Label>
            <Input
              id="subject"
              bind:value={subject}
              placeholder="Email subject"
              required
              class="mt-1"
            />
          </div>

          <div>
            <Label for="body">Message</Label>
            <textarea
              id="body"
              bind:value={body}
              rows="12"
              placeholder="Write your message here..."
              class="mt-1 w-full p-3 border border-input rounded-md bg-background resize-none focus:outline-none focus:ring-2 focus:ring-ring"
            />
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <Button type="button" variant="outline" on:click={handleClose}>Cancel</Button>
            <Button type="submit" disabled={saving}>
              <Save class="h-4 w-4 mr-2" />
              {saving ? 'Saving...' : 'Save Draft'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
