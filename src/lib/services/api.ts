import { invoke } from '@tauri-apps/api/tauri';
import type {
  Account,
  Folder,
  Message,
  MessageHeader,
  OAuthProviderConfig,
  Draft,
  EmailSignature,
  Contact,
  Thread,
} from '../types';

export const addAccount = (emailAddress: string): Promise<string> => {
  return invoke('add_account', { emailAddress });
};

export const handleCallback = (code: string, state: string): Promise<Account> => {
  return invoke('handle_callback', { code, state });
};

export const getAccounts = (): Promise<Account[]> => {
  return invoke('get_accounts');
};

export const deleteAccount = (accountId: number): Promise<void> => {
  return invoke('delete_account', { accountId });
};

export const saveOauthProviderConfig = (
  provider: string,
  config: OAuthProviderConfig
): Promise<void> => {
  return invoke('save_oauth_provider_config', { provider, config });
};

export const getOauthProviderConfigs = (): Promise<Record<string, OAuthProviderConfig>> => {
  return invoke('get_oauth_provider_configs');
};

export const getFolders = (accountId: number): Promise<Folder[]> => {
  return invoke('get_folders', { accountId });
};

export const getMessages = (folderId: number): Promise<MessageHeader[]> => {
  return invoke('get_messages', { folderId });
};

export const getMessageDetails = (messageId: number): Promise<Message> => {
  return invoke('get_message_details', { messageId });
};

export const sendEmail = (
  accountId: number,
  to: string,
  subject: string,
  body: string
): Promise<void> => {
  return invoke('send_email', { accountId, to, subject, body });
};

export const startExport = (
  accountId: number,
  destinationPath: string
): Promise<void> => {
  return invoke('start_export', { accountId, destinationPath });
};

export const markMessageRead = (messageId: number): Promise<void> => {
  return invoke('mark_message_read', { messageId });
};

export const markMessageUnread = (messageId: number): Promise<void> => {
  return invoke('mark_message_unread', { messageId });
};

export const refreshAccount = (accountId: number): Promise<void> => {
  return invoke('refresh_account', { accountId });
};

export const searchMessages = (
  accountId: number,
  query: string
): Promise<MessageHeader[]> => {
  return invoke('search_messages', { accountId, query });
};

export const saveDraft = (draft: Draft): Promise<number> => {
  return invoke('save_draft', { draft });
};

export const getDrafts = (accountId: number): Promise<Draft[]> => {
  return invoke('get_drafts', { accountId });
};

export const deleteDraft = (draftId: number): Promise<void> => {
  return invoke('delete_draft', { draftId });
};

export const saveSignature = (signature: EmailSignature): Promise<number> => {
  return invoke('save_signature', { signature });
};

export const getSignatures = (accountId: number): Promise<EmailSignature[]> => {
  return invoke('get_signatures', { accountId });
};

export const deleteSignature = (signatureId: number): Promise<void> => {
  return invoke('delete_signature', { signatureId });
};

export const downloadAttachment = (
  attachmentId: number,
  destinationPath: string
): Promise<void> => {
  return invoke('download_attachment', { attachmentId, destinationPath });
};

export const getMessagesPaginated = (
  folderId: number,
  limit: number,
  offset: number
): Promise<MessageHeader[]> => {
  return invoke('get_messages_paginated', { folderId, limit, offset });
};

export const countMessagesInFolder = (folderId: number): Promise<number> => {
  return invoke('count_messages_in_folder', { folderId });
};

export const deleteMessage = (messageId: number): Promise<void> => {
  return invoke('delete_message', { messageId });
};

export const moveMessage = (messageId: number, targetFolderId: number): Promise<void> => {
  return invoke('move_message', { messageId, targetFolderId });
};

export const saveSetting = (key: string, value: string): Promise<void> => {
  return invoke('save_setting', { key, value });
};

export const getSetting = (key: string): Promise<string | null> => {
  return invoke('get_setting', { key });
};

export const getAllSettings = (): Promise<{ key: string; value: string }[]> => {
  return invoke('get_all_settings');
};

export const prepareReply = (messageId: number, replyAll: boolean): Promise<{
  to: string;
  cc: string | null;
  subject: string;
  quoted_body: string;
}> => {
  return invoke('prepare_reply', { messageId, replyAll });
};

export const prepareForward = (messageId: number): Promise<{
  subject: string;
  body_with_header: string;
}> => {
  return invoke('prepare_forward', { messageId });
};

// ==================== Phase 3: Starring Messages ====================

export const starMessage = (messageId: number): Promise<void> => {
  return invoke('star_message', { messageId });
};

export const unstarMessage = (messageId: number): Promise<void> => {
  return invoke('unstar_message', { messageId });
};

export const getStarredMessages = (accountId: number): Promise<MessageHeader[]> => {
  return invoke('get_starred_messages', { accountId });
};

// ==================== Phase 3: Bulk Operations ====================

export const bulkMarkRead = (messageIds: number[]): Promise<void> => {
  return invoke('bulk_mark_read', { messageIds });
};

export const bulkMarkUnread = (messageIds: number[]): Promise<void> => {
  return invoke('bulk_mark_unread', { messageIds });
};

export const bulkDeleteMessages = (messageIds: number[]): Promise<void> => {
  return invoke('bulk_delete_messages', { messageIds });
};

export const bulkStarMessages = (messageIds: number[]): Promise<void> => {
  return invoke('bulk_star_messages', { messageIds });
};

export const bulkUnstarMessages = (messageIds: number[]): Promise<void> => {
  return invoke('bulk_unstar_messages', { messageIds });
};

// ==================== Phase 3: Contacts ====================

export const searchContacts = (query: string): Promise<Contact[]> => {
  return invoke('search_contacts', { query });
};

export const getRecentContacts = (limit: number): Promise<Contact[]> => {
  return invoke('get_recent_contacts', { limit });
};

export const getFrequentContacts = (limit: number): Promise<Contact[]> => {
  return invoke('get_frequent_contacts', { limit });
};

// ==================== Phase 3: Threading ====================

export const getThreadMessages = (threadId: number): Promise<Message[]> => {
  return invoke('get_thread_messages', { threadId });
};

export const getThread = (threadId: number): Promise<Thread> => {
  return invoke('get_thread', { threadId });
};