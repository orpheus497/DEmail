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

  return {
    subscribe,
    fetchAccounts,
    selectAccount,
    selectFolder,
    selectMessage,
  };
};

export const mailbox = createMailboxStore();