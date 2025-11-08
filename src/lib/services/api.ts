import { invoke } from '@tauri-apps/api/tauri';
import type {
  Account,
  Folder,
  Message,
  MessageHeader,
  OAuthProviderConfig,
  Draft,
  EmailSignature,
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