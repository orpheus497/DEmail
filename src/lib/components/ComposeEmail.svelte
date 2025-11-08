<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import Button from '$lib/components/ui/button/index.svelte';
  import Input from '$lib/components/ui/input/index.svelte';
  import Label from '$lib/components/ui/label/index.svelte';
  import ContactAutocomplete from '$lib/components/ContactAutocomplete.svelte';
  import { sendEmail, prepareReply, prepareForward, getSignatures, saveDraft, deleteDraft, getDrafts } from '$lib/services/api';
  import type { EmailSignature, Draft } from '$lib/types';
  import { X, Save } from 'lucide-svelte';

  export let accountId: number;
  export let open = false;
  export let mode: 'compose' | 'reply' | 'replyAll' | 'forward' = 'compose';
  export let messageId: number | null = null;
  export let draftId: number | null = null;

  let to = '';
  let cc = '';
  let bcc = '';
  let subject = '';
  let body = '';
  let sending = false;
  let loading = false;
  let error: string | null = null;
  let defaultSignature: EmailSignature | null = null;

  // Phase 6: Draft auto-save
  let savingDraft = false;
  let lastDraftSaveTime: number | null = null;
  let autoSaveTimeout: number | null = null;
  let currentDraftId: number | null = null;

  const dispatch = createEventDispatcher<{ sent: void; close: void }>();

  // Phase 6: Load default signature when opening compose dialog
  $: if (open && mode === 'compose' && !draftId) {
    loadDefaultSignature();
  }

  // Phase 6: Load draft when draftId is provided
  $: if (open && draftId) {
    loadDraftData();
  }

  // Load reply/forward data when mode changes
  $: if (open && messageId && mode !== 'compose') {
    loadComposeData();
  }

  // Phase 6: Auto-save draft (debounced)
  $: if (open && mode === 'compose' && (to || subject || body)) {
    if (autoSaveTimeout) {
      clearTimeout(autoSaveTimeout);
    }
    autoSaveTimeout = window.setTimeout(() => {
      handleAutoSaveDraft();
    }, 3000); // Auto-save after 3 seconds of inactivity
  }

  onDestroy(() => {
    if (autoSaveTimeout) {
      clearTimeout(autoSaveTimeout);
    }
  });

  async function loadDefaultSignature() {
    try {
      const signatures = await getSignatures(accountId);
      defaultSignature = signatures.find(sig => sig.is_default) || null;

      // Add signature to body if composing new email
      if (defaultSignature && !body) {
        body = '\n\n--\n' + defaultSignature.content_plain;
      }
    } catch (e) {
      console.error('Failed to load signature:', e);
      defaultSignature = null;
    }
  }

  // Phase 6: Load draft data
  async function loadDraftData() {
    if (!draftId) return;

    loading = true;
    error = null;

    try {
      const drafts = await getDrafts(accountId);
      const draft = drafts.find(d => d.id === draftId);

      if (draft) {
        to = draft.to_header || '';
        cc = draft.cc_header || '';
        bcc = draft.bcc_header || '';
        subject = draft.subject || '';
        body = draft.body_plain || '';
        currentDraftId = draft.id;
      } else {
        error = 'Draft not found';
      }
    } catch (e) {
      error = `Failed to load draft: ${String(e)}`;
    } finally {
      loading = false;
    }
  }

  async function loadComposeData() {
    if (!messageId) return;

    loading = true;
    error = null;

    try {
      // Load signature if not already loaded
      if (!defaultSignature) {
        const signatures = await getSignatures(accountId);
        defaultSignature = signatures.find(sig => sig.is_default) || null;
      }

      if (mode === 'reply' || mode === 'replyAll') {
        const replyData = await prepareReply(messageId, mode === 'replyAll');
        to = replyData.to;
        cc = replyData.cc || '';
        subject = replyData.subject;
        body = '\n\n' + replyData.quoted_body;

        // Add signature to replies
        if (defaultSignature) {
          body = '\n\n--\n' + defaultSignature.content_plain + body;
        }
      } else if (mode === 'forward') {
        const forwardData = await prepareForward(messageId);
        to = '';
        cc = '';
        subject = forwardData.subject;
        body = '\n\n' + forwardData.body_with_header;

        // Add signature to forwards
        if (defaultSignature) {
          body = '\n\n--\n' + defaultSignature.content_plain + body;
        }
      }
    } catch (e) {
      error = `Failed to load ${mode} data: ${String(e)}`;
    } finally {
      loading = false;
    }
  }

  function getTitle(): string {
    switch (mode) {
      case 'reply': return 'Reply';
      case 'replyAll': return 'Reply All';
      case 'forward': return 'Forward';
      default: return 'Compose Email';
    }
  }

  // Phase 6: Auto-save draft
  async function handleAutoSaveDraft() {
    if (sending || !to && !subject && !body) return;

    savingDraft = true;
    try {
      const draft: Draft = {
        id: currentDraftId || draftId || 0,
        account_id: accountId,
        to_header: to,
        cc_header: cc || null,
        bcc_header: bcc || null,
        subject,
        body_plain: body,
        body_html: null,
        created_at: 0,
        updated_at: 0,
      };

      const savedId = await saveDraft(draft);
      if (!currentDraftId) {
        currentDraftId = savedId;
      }
      lastDraftSaveTime = Date.now();
    } catch (e) {
      console.error('Failed to auto-save draft:', e);
    } finally {
      savingDraft = false;
    }
  }

  async function handleManualSaveDraft() {
    if (!to && !subject && !body) {
      error = 'Cannot save empty draft';
      return;
    }

    await handleAutoSaveDraft();
    if (!error) {
      // Show confirmation message briefly
      const originalError = error;
      error = null;
      setTimeout(() => {
        if (error === null) {
          error = originalError;
        }
      }, 2000);
    }
  }

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

      // Delete draft after successful send
      if (currentDraftId || draftId) {
        try {
          await deleteDraft(currentDraftId || draftId || 0);
        } catch (e) {
          console.error('Failed to delete draft:', e);
        }
      }

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
    bcc = '';
    subject = '';
    body = '';
    error = null;
    mode = 'compose';
    messageId = null;
    draftId = null;
    currentDraftId = null;
    lastDraftSaveTime = null;
    if (autoSaveTimeout) {
      clearTimeout(autoSaveTimeout);
      autoSaveTimeout = null;
    }
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
        <h2 class="text-2xl font-bold">{getTitle()}</h2>
        <Button variant="ghost" size="sm" on:click={handleClose}>
          <X class="h-4 w-4" />
        </Button>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        {#if loading}
          <div class="flex items-center justify-center p-8">
            <div class="text-muted-foreground">Loading...</div>
          </div>
        {:else}
          {#if error}
            <div class="mb-4 p-3 bg-destructive/10 text-destructive rounded-md text-sm">
              {error}
            </div>
          {/if}

        <form on:submit|preventDefault={handleSend} class="space-y-4">
          <ContactAutocomplete
            id="to"
            label="To"
            bind:value={to}
            placeholder="recipient@example.com"
            required={true}
          />

          <ContactAutocomplete
            id="cc"
            label="CC"
            bind:value={cc}
            placeholder="cc@example.com (optional)"
          />

          <ContactAutocomplete
            id="bcc"
            label="BCC"
            bind:value={bcc}
            placeholder="bcc@example.com (optional)"
          />

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

          <div class="flex items-center justify-between pt-4">
            <!-- Phase 6: Auto-save indicator -->
            <div class="text-xs text-muted-foreground">
              {#if savingDraft}
                <span class="flex items-center gap-1">
                  <Save class="h-3 w-3 animate-pulse" />
                  Saving draft...
                </span>
              {:else if lastDraftSaveTime && mode === 'compose'}
                <span>Draft saved {new Date(lastDraftSaveTime).toLocaleTimeString()}</span>
              {/if}
            </div>

            <div class="flex gap-2">
              <Button type="button" variant="outline" on:click={handleClose}>
                Cancel
              </Button>
              {#if mode === 'compose'}
                <Button type="button" variant="secondary" on:click={handleManualSaveDraft} disabled={savingDraft}>
                  <Save class="h-4 w-4 mr-2" />
                  Save Draft
                </Button>
              {/if}
              <Button type="submit" disabled={sending}>
                {sending ? 'Sending...' : 'Send Email'}
              </Button>
            </div>
          </div>
        </form>
        {/if}
      </div>
    </div>
  </div>
{/if}
