<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/ui/button/index.svelte';
  import Input from '$lib/components/ui/input/index.svelte';
  import Label from '$lib/components/ui/label/index.svelte';
  import { sendEmail } from '$lib/services/api';
  import { X } from 'lucide-svelte';

  export let accountId: number;
  export let open = false;

  let to = '';
  let cc = '';
  let subject = '';
  let body = '';
  let sending = false;
  let error: string | null = null;

  const dispatch = createEventDispatcher<{ sent: void; close: void }>();

  async function handleSend() {
    if (!to.trim() || !subject.trim()) {
      error = 'To and Subject fields are required';
      return;
    }

    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(to.trim())) {
      error = 'Please enter a valid email address in the To field';
      return;
    }

    if (cc.trim() && !emailRegex.test(cc.trim())) {
      error = 'Please enter a valid email address in the CC field';
      return;
    }

    sending = true;
    error = null;

    try {
      await sendEmail(accountId, to.trim(), subject.trim(), body);
      dispatch('sent');
      resetForm();
      open = false;
    } catch (e) {
      error = String(e);
    } finally {
      sending = false;
    }
  }

  function resetForm() {
    to = '';
    cc = '';
    subject = '';
    body = '';
    error = null;
  }

  function handleClose() {
    resetForm();
    open = false;
    dispatch('close');
  }
</script>

{#if open}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
    <div class="bg-background rounded-lg shadow-lg max-w-2xl w-full max-h-[90vh] flex flex-col">
      <div class="flex items-center justify-between p-6 border-b">
        <h2 class="text-2xl font-bold">Compose Email</h2>
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

        <form on:submit|preventDefault={handleSend} class="space-y-4">
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
            <Button type="button" variant="outline" on:click={handleClose}>
              Cancel
            </Button>
            <Button type="submit" disabled={sending}>
              {sending ? 'Sending...' : 'Send Email'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}
