import { expect, test, vi } from 'vitest';
import type { Mock } from 'vitest';
import {
  addAccount,
  getAccounts,
  deleteAccount,
  getFolders,
  getMessages,
  sendEmail,
  markMessageRead,
  markMessageUnread,
  saveDraft,
  getDrafts,
  deleteDraft,
  saveSignature,
  getSignatures,
  deleteSignature,
  downloadAttachment,
  getMessagesPaginated,
  countMessagesInFolder,
} from '../../src/lib/services/api';
import type { Account, Draft, Message } from '../../src/lib/types';

vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/tauri';

const mockInvoke = invoke as Mock;

test('addAccount calls invoke with correct parameters', async () => {
  const mockEmail = 'test@example.com';
  mockInvoke.mockResolvedValue('auth-url');

  const result = await addAccount(mockEmail);

  expect(invoke).toHaveBeenCalledWith('add_account', { emailAddress: mockEmail });
  expect(result).toBe('auth-url');
});

test('getAccounts calls invoke', async () => {
  const mockAccounts: Account[] = [
    { id: 1, email_address: 'test@example.com', display_name: 'Test', provider_type: 'google' },
  ];
  mockInvoke.mockResolvedValue(mockAccounts);

  const result = await getAccounts();

  expect(invoke).toHaveBeenCalledWith('get_accounts');
  expect(result).toEqual(mockAccounts);
});

test('deleteAccount calls invoke with account ID', async () => {
  mockInvoke.mockResolvedValue(undefined);

  await deleteAccount(1);

  expect(invoke).toHaveBeenCalledWith('delete_account', { accountId: 1 });
});

test('saveDraft calls invoke with draft object', async () => {
  const mockDraft: Draft = {
    id: 0,
    account_id: 1,
    to_addresses: 'test@example.com',
    cc_addresses: null,
    bcc_addresses: null,
    subject: 'Test',
    body_plain: 'Body',
    body_html: null,
    created_at: 1234567890,
    updated_at: 1234567890,
  };
  mockInvoke.mockResolvedValue(123);

  const result = await saveDraft(mockDraft);

  expect(invoke).toHaveBeenCalledWith('save_draft', { draft: mockDraft });
  expect(result).toBe(123);
});

test('getDrafts calls invoke with account ID', async () => {
  const mockDrafts: Draft[] = [];
  mockInvoke.mockResolvedValue(mockDrafts);

  const result = await getDrafts(1);

  expect(invoke).toHaveBeenCalledWith('get_drafts', { accountId: 1 });
  expect(result).toEqual(mockDrafts);
});

test('getMessagesPaginated calls invoke with correct parameters', async () => {
  const mockMessages: Message[] = [];
  mockInvoke.mockResolvedValue(mockMessages);

  const result = await getMessagesPaginated(1, 50, 0);

  expect(invoke).toHaveBeenCalledWith('get_messages_paginated', {
    folderId: 1,
    limit: 50,
    offset: 0,
  });
  expect(result).toEqual(mockMessages);
});

test('countMessagesInFolder calls invoke with folder ID', async () => {
  mockInvoke.mockResolvedValue(42);

  const result = await countMessagesInFolder(1);

  expect(invoke).toHaveBeenCalledWith('count_messages_in_folder', { folderId: 1 });
  expect(result).toBe(42);
});

test('downloadAttachment calls invoke with correct parameters', async () => {
  mockInvoke.mockResolvedValue(undefined);

  await downloadAttachment(1, '/path/to/save');

  expect(invoke).toHaveBeenCalledWith('download_attachment', {
    attachmentId: 1,
    destinationPath: '/path/to/save',
  });
});
