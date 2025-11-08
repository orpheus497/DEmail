import { writable } from 'svelte/store';
import type { Account, Folder, Message, MessageHeader } from '../types';
import {
  getAccounts,
  getFolders,
  getMessages,
  getMessageDetails,
} from '../services/api';

interface MailboxStore {
  accounts: Account[];
  selectedAccount: Account | null;
  folders: Folder[];
  selectedFolder: Folder | null;
  messages: MessageHeader[];
  selectedMessage: Message | null;
  loading: boolean;
  error: string | null;
}

const createMailboxStore = () => {
  const { subscribe, set, update } = writable<MailboxStore>({
    accounts: [],
    selectedAccount: null,
    folders: [],
    selectedFolder: null,
    messages: [],
    selectedMessage: null,
    loading: false,
    error: null,
  });

  const fetchAccounts = async () => {
    update((state) => ({ ...state, loading: true, error: null }));
    try {
      const accounts = await getAccounts();
      update((state) => ({ ...state, accounts, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const selectAccount = async (account: Account) => {
    update((state) => ({
      ...state,
      selectedAccount: account,
      folders: [],
      selectedFolder: null,
      messages: [],
      selectedMessage: null,
      loading: true,
      error: null,
    }));
    try {
      const folders = await getFolders(account.id);
      update((state) => ({ ...state, folders, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const selectFolder = async (folder: Folder) => {
    update((state) => ({
      ...state,
      selectedFolder: folder,
      messages: [],
      selectedMessage: null,
      loading: true,
      error: null,
    }));
    try {
      const messages = await getMessages(folder.id);
      update((state) => ({ ...state, messages, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const selectMessage = async (messageHeader: MessageHeader) => {
    update((state) => ({
      ...state,
      selectedMessage: null,
      loading: true,
      error: null,
    }));
    try {
      const message = await getMessageDetails(messageHeader.id);
      update((state) => ({ ...state, selectedMessage: message, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const markRead = async (messageId: number) => {
    try {
      await import('../services/api').then(api => api.markMessageRead(messageId));
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          msg.id === messageId ? { ...msg, is_read: true } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage?.id === messageId
            ? { ...state.selectedMessage, is_read: true }
            : state.selectedMessage;
        return {
          ...state,
          messages: updatedMessages,
          selectedMessage: updatedSelectedMessage,
        };
      });
    } catch (error) {
      update((state) => ({ ...state, error: String(error) }));
    }
  };

  const markUnread = async (messageId: number) => {
    try {
      await import('../services/api').then(api => api.markMessageUnread(messageId));
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          msg.id === messageId ? { ...msg, is_read: false } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage?.id === messageId
            ? { ...state.selectedMessage, is_read: false }
            : state.selectedMessage;
        return {
          ...state,
          messages: updatedMessages,
          selectedMessage: updatedSelectedMessage,
        };
      });
    } catch (error) {
      update((state) => ({ ...state, error: String(error) }));
    }
  };

  const refreshAccount = async () => {
    update((state) => ({ ...state, loading: true, error: null }));
    try {
      const { selectedAccount } = await new Promise<MailboxStore>((resolve) => {
        const unsub = subscribe((state) => {
          resolve(state);
          unsub();
        });
      });

      if (!selectedAccount) {
        update((state) => ({ ...state, loading: false }));
        return;
      }

      await import('../services/api').then(api => api.refreshAccount(selectedAccount.id));

      const folders = await import('../services/api').then(api => api.getFolders(selectedAccount.id));
      update((state) => ({ ...state, folders, loading: false }));

      const { selectedFolder } = await new Promise<MailboxStore>((resolve) => {
        const unsub = subscribe((state) => {
          resolve(state);
          unsub();
        });
      });

      if (selectedFolder) {
        const messages = await import('../services/api').then(api => api.getMessages(selectedFolder.id));
        update((state) => ({ ...state, messages }));
      }
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const searchInMessages = async (query: string) => {
    if (!query.trim()) {
      const { selectedFolder } = await new Promise<MailboxStore>((resolve) => {
        const unsub = subscribe((state) => {
          resolve(state);
          unsub();
        });
      });
      if (selectedFolder) {
        await selectFolder(selectedFolder);
      }
      return;
    }

    update((state) => ({ ...state, loading: true, error: null }));
    try {
      const { selectedAccount } = await new Promise<MailboxStore>((resolve) => {
        const unsub = subscribe((state) => {
          resolve(state);
          unsub();
        });
      });

      if (!selectedAccount) {
        update((state) => ({ ...state, loading: false }));
        return;
      }

      const messages = await import('../services/api').then(api =>
        api.searchMessages(selectedAccount.id, query)
      );
      update((state) => ({ ...state, messages, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  return {
    subscribe,
    fetchAccounts,
    selectAccount,
    selectFolder,
    selectMessage,
    markRead,
    markUnread,
    refreshAccount,
    searchInMessages,
  };
};

export const mailbox = createMailboxStore();