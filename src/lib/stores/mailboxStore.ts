import { writable } from 'svelte/store';
import type { Account, Folder, Message, MessageHeader } from '../types';
import {
  getAccounts,
  getFolders,
  getMessages,
  getMessageDetails,
  getMessagesPaginated,
  countMessagesInFolder,
  starMessage as apiStarMessage,
  unstarMessage as apiUnstarMessage,
  getStarredMessages,
  bulkMarkRead as apiBulkMarkRead,
  bulkMarkUnread as apiBulkMarkUnread,
  bulkDeleteMessages as apiBulkDeleteMessages,
  bulkStarMessages as apiBulkStarMessages,
  bulkUnstarMessages as apiBulkUnstarMessages,
  getThreadMessages,
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
  // Pagination state
  totalMessages: number;
  currentPage: number;
  pageSize: number;
  hasMore: boolean;
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
    totalMessages: 0,
    currentPage: 0,
    pageSize: 50,
    hasMore: false,
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
      await import('../services/api').then((api) => api.markMessageRead(messageId));
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
      await import('../services/api').then((api) => api.markMessageUnread(messageId));
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

      await import('../services/api').then((api) => api.refreshAccount(selectedAccount.id));

      const folders = await import('../services/api').then((api) =>
        api.getFolders(selectedAccount.id)
      );
      update((state) => ({ ...state, folders, loading: false }));

      const { selectedFolder } = await new Promise<MailboxStore>((resolve) => {
        const unsub = subscribe((state) => {
          resolve(state);
          unsub();
        });
      });

      if (selectedFolder) {
        const messages = await import('../services/api').then((api) =>
          api.getMessages(selectedFolder.id)
        );
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

      const messages = await import('../services/api').then((api) =>
        api.searchMessages(selectedAccount.id, query)
      );
      update((state) => ({ ...state, messages, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const deleteMessage = async (messageId: number) => {
    try {
      await import('../services/api').then((api) => api.deleteMessage(messageId));
      update((state) => {
        const updatedMessages = state.messages.filter((msg) => msg.id !== messageId);
        const updatedSelectedMessage =
          state.selectedMessage?.id === messageId ? null : state.selectedMessage;
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

  const moveMessage = async (messageId: number, targetFolderId: number) => {
    try {
      await import('../services/api').then((api) => api.moveMessage(messageId, targetFolderId));
      update((state) => {
        const updatedMessages = state.messages.filter((msg) => msg.id !== messageId);
        const updatedSelectedMessage =
          state.selectedMessage?.id === messageId ? null : state.selectedMessage;
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

  // ==================== Phase 3: Starring Messages ====================

  const starMessage = async (messageId: number) => {
    try {
      await apiStarMessage(messageId);
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          msg.id === messageId ? { ...msg, is_starred: true } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage?.id === messageId
            ? { ...state.selectedMessage, is_starred: true }
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

  const unstarMessage = async (messageId: number) => {
    try {
      await apiUnstarMessage(messageId);
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          msg.id === messageId ? { ...msg, is_starred: false } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage?.id === messageId
            ? { ...state.selectedMessage, is_starred: false }
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

  const loadStarredMessages = async () => {
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

      const messages = await getStarredMessages(selectedAccount.id);
      update((state) => ({ ...state, messages, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  // ==================== Phase 3: Bulk Operations ====================

  const bulkMarkRead = async (messageIds: number[]) => {
    try {
      await apiBulkMarkRead(messageIds);
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          messageIds.includes(msg.id) ? { ...msg, is_read: true } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage && messageIds.includes(state.selectedMessage.id)
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

  const bulkMarkUnread = async (messageIds: number[]) => {
    try {
      await apiBulkMarkUnread(messageIds);
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          messageIds.includes(msg.id) ? { ...msg, is_read: false } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage && messageIds.includes(state.selectedMessage.id)
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

  const bulkDelete = async (messageIds: number[]) => {
    try {
      await apiBulkDeleteMessages(messageIds);
      update((state) => {
        const updatedMessages = state.messages.filter((msg) => !messageIds.includes(msg.id));
        const updatedSelectedMessage =
          state.selectedMessage && messageIds.includes(state.selectedMessage.id)
            ? null
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

  const bulkStar = async (messageIds: number[]) => {
    try {
      await apiBulkStarMessages(messageIds);
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          messageIds.includes(msg.id) ? { ...msg, is_starred: true } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage && messageIds.includes(state.selectedMessage.id)
            ? { ...state.selectedMessage, is_starred: true }
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

  const bulkUnstar = async (messageIds: number[]) => {
    try {
      await apiBulkUnstarMessages(messageIds);
      update((state) => {
        const updatedMessages = state.messages.map((msg) =>
          messageIds.includes(msg.id) ? { ...msg, is_starred: false } : msg
        );
        const updatedSelectedMessage =
          state.selectedMessage && messageIds.includes(state.selectedMessage.id)
            ? { ...state.selectedMessage, is_starred: false }
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

  // ==================== Phase 3: Threading ====================

  const loadThreadMessages = async (threadId: number) => {
    update((state) => ({ ...state, loading: true, error: null }));
    try {
      const threadMessages = await getThreadMessages(threadId);
      // Convert full messages to message headers for display in list
      const messageHeaders: MessageHeader[] = threadMessages.map((msg) => ({
        id: msg.id,
        subject: msg.subject,
        from: msg.from_header,
        date: msg.date,
        is_read: msg.is_read,
        has_attachments: msg.has_attachments,
        is_starred: msg.is_starred,
      }));
      update((state) => ({ ...state, messages: messageHeaders, loading: false }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  // ==================== Phase 5: Pagination ====================

  const loadMoreMessages = async () => {
    let currentState: MailboxStore | null = null;
    const unsub = subscribe((state) => {
      currentState = state;
    });
    unsub();

    if (
      !currentState ||
      !currentState.selectedFolder ||
      currentState.loading ||
      !currentState.hasMore
    ) {
      return;
    }

    update((state) => ({ ...state, loading: true, error: null }));

    try {
      const offset = currentState.currentPage * currentState.pageSize;
      const newMessages = await getMessagesPaginated(
        currentState.selectedFolder.id,
        currentState.pageSize,
        offset
      );

      update((state) => ({
        ...state,
        messages: [...state.messages, ...newMessages],
        currentPage: state.currentPage + 1,
        hasMore: newMessages.length === state.pageSize,
        loading: false,
      }));
    } catch (error) {
      update((state) => ({ ...state, error: String(error), loading: false }));
    }
  };

  const selectFolderWithPagination = async (folder: Folder) => {
    update((state) => ({
      ...state,
      selectedFolder: folder,
      messages: [],
      selectedMessage: null,
      loading: true,
      error: null,
      currentPage: 0,
      totalMessages: 0,
      hasMore: false,
    }));

    try {
      // Get total count
      const total = await countMessagesInFolder(folder.id);

      // Load first page
      const pageSize = 50;
      const messages = await getMessagesPaginated(folder.id, pageSize, 0);

      update((state) => ({
        ...state,
        messages,
        totalMessages: total,
        currentPage: 1,
        pageSize,
        hasMore: messages.length === pageSize && total > pageSize,
        loading: false,
      }));
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
    deleteMessage,
    moveMessage,
    starMessage,
    unstarMessage,
    loadStarredMessages,
    bulkMarkRead,
    bulkMarkUnread,
    bulkDelete,
    bulkStar,
    bulkUnstar,
    loadThreadMessages,
    loadMoreMessages,
    selectFolderWithPagination,
  };
};

export const mailbox = createMailboxStore();
