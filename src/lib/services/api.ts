'''import { invoke } from '@tauri-apps/api/tauri';
import type {
  Account,
  Folder,
  Message,
  MessageHeader,
  OAuthProviderConfig,
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
''