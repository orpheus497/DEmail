export interface Account {
  id: number;
  email_address: string;
  display_name: string;
  provider_type: string;
}

export interface Folder {
  id: number;
  account_id: number;
  name: string;
  path: string;
  parent_id: number | null;
}

export interface MessageHeader {
  id: number;
  subject: string;
  from: string;
  date: number;
  is_read: boolean;
  has_attachments: boolean;
  is_starred: boolean;
}

export interface Message {
  id: number;
  account_id: number;
  folder_id: number;
  imap_uid: number;
  message_id_header: string;
  from_header: string;
  to_header: string;
  cc_header: string | null;
  subject: string;
  date: number;
  body_plain: string | null;
  body_html: string | null;
  has_attachments: boolean;
  is_read: boolean;
  is_starred: boolean;
  thread_id: number | null;
  attachments: Attachment[];
}

export interface Attachment {
  id: number;
  message_id: number;
  filename: string;
  mime_type: string;
  size_bytes: number;
  local_path: string | null;
}

export interface OAuthProviderConfig {
  client_id: string;
  client_secret: string;
}

export interface Draft {
  id: number;
  account_id: number;
  to_addresses: string;
  cc_addresses: string | null;
  bcc_addresses: string | null;
  subject: string;
  body_plain: string | null;
  body_html: string | null;
  created_at: number;
  updated_at: number;
}

export interface EmailSignature {
  id: number;
  account_id: number;
  name: string;
  content_html: string;
  content_plain: string;
  is_default: boolean;
}

export interface AppSetting {
  key: string;
  value: string;
}

export interface Contact {
  id: number;
  email: string;
  name: string | null;
  last_used: number;
  use_count: number;
}

export interface Thread {
  id: number;
  subject_hash: string;
  first_message_id: number;
  last_message_id: number;
  message_count: number;
  account_id: number;
  created_at: number;
  updated_at: number;
}