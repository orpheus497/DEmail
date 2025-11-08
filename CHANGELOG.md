# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

**Core Email Features:**
- Full-text search functionality using SQLite FTS5 virtual table for searching across message subjects, senders, recipients, and body text
- Email composition modal component with To, CC, Subject, and Body fields, including email validation and send functionality
- Draft management system with auto-save functionality, database persistence, and UI for managing draft emails
- Email signature system with HTML and plain text support, per-account signatures, and default signature selection
- Manual account refresh button in inbox header for triggering immediate IMAP synchronization
- Read/unread message status management with automatic marking as read on message view
- Message pagination with configurable page sizes for performance optimization with large mailboxes
- Attachment download functionality with file dialogs and local storage integration
- Attachment validation with file size limits (25MB) and dangerous file extension warnings
- Settings management system for storing application preferences in SQLite database

**Database Enhancements:**
- Database schema: `drafts` table for storing email drafts with full composition data
- Database schema: `signatures` table for per-account email signatures with HTML/plain text content
- Database schema: `settings` table for application-wide configuration key-value pairs
- Database schema: `attachment_data` table for storing attachment binary data locally
- Database indexes on messages table (date, folder_id, account_id) for query performance
- Database functions: `save_draft`, `get_drafts`, `delete_draft` for draft management
- Database functions: `save_signature`, `get_signatures`, `delete_signature` for signature management
- Database functions: `save_setting`, `get_setting`, `get_all_settings` for settings persistence
- Database functions: `save_attachment_data`, `get_attachment_data` for attachment storage
- Database functions: `delete_message`, `move_message` for message management
- Database functions: `get_messages_paginated`, `count_messages_in_folder` for pagination
- Database functions: `update_message_read_status` for updating message read status
- Database functions: `search_messages_fts` for FTS5-based full-text search
- FTS5 virtual table `messages_fts` with automatic sync triggers (insert, update, delete) for real-time search indexing

**Backend Modules:**
- Core module: `src-tauri/src/core/drafts.rs` for draft creation, update, deletion, and auto-save logic
- Core module: `src-tauri/src/core/attachments.rs` for attachment file operations, validation, and safety checks
- Tauri commands: `mark_message_read`, `mark_message_unread`, `refresh_account`, `search_messages`
- Tauri feature flags: `dialog-all`, `fs-all`, `path-all` for file system and dialog operations
- Tauri allowlist configuration: dialog permissions (open, save) for file picker operations
- Tauri allowlist configuration: fs permissions (readFile, writeFile) with scoped access to APPDATA, DOWNLOAD, DOCUMENT directories

**Frontend Components:**
- SearchBar component with debounced input (500ms delay) and clear button functionality
- ComposeEmail component as modal dialog for email composition with form validation and error handling
- Context menu options in message list for manually marking messages as read or unread via right-click
- Visual distinction between read and unread messages in message list with mail icon indicators and bold font for unread messages
- Attachments display section in message view showing filename, MIME type, and file size for each attachment
- Compose button in inbox header with Pencil icon for creating emails
- Refresh button in inbox header with animated spinning icon during synchronization
- Search bar integrated into message list pane header for quick email search
- Resizable pane components using paneforge library for adjustable 3-pane email layout

**Frontend API & Store:**
- Frontend API methods: `markMessageRead`, `markMessageUnread`, `refreshAccount`, `searchMessages` in services/api.ts
- Store methods in mailboxStore for read status management (`markRead`, `markUnread`), account refresh (`refreshAccount`), and message search (`searchInMessages`)
- Type definitions: `Draft`, `EmailSignature`, `AppSetting` interfaces in types/index.ts

**Routing & Pages:**
- OAuth callback page at `/callback` route for handling authentication redirects
- Inbox page at `/inbox` route with full 3-pane email client interface
- Index exports for Select UI component for proper module resolution
- Improved Settings page with account addition functionality and status feedback

**Build & Development:**
- Application icon files in all required formats (32x32.png, 128x128.png, 128x128@2x.png, icon.ico, icon.icns) for complete Tauri build support across all platforms (Windows, macOS, Linux)
- Vitest configuration file (vitest.config.ts) for frontend unit testing setup with happy-dom environment
- NPM scripts: `test`, `test:ui`, `test:coverage` for running frontend tests
- Testing dependencies: vitest (^0.34.0), @testing-library/svelte (^4.0.0), @vitest/ui (^0.34.0), happy-dom (^12.0.0)

**Dependencies:**
- Rust dependencies: `mime_guess` (2.0) for MIME type detection in attachment handling
- Rust dependencies: `regex` (1.10) for email address validation and pattern matching
- Rust dependencies: `uuid` (1.6) with v4 and serde features for unique identifier generation

### Fixed
- Corrected paneforge dependency version in package.json from non-existent ^0.2.0 to ^0.0.6 for Svelte 4 compatibility
- **CRITICAL:** Removed triple-quote syntax artifacts from multiple Svelte and TypeScript component files that prevented compilation (api.ts, utils.ts, AccountSwitcher.svelte, FolderList.svelte, MessageList.svelte, MessageView.svelte, +page.svelte, Settings.svelte, +layout.svelte)
- **CRITICAL:** Fixed undefined variable `folders` in imap_sync.rs fetch_folders function by adding proper Vec initialization
- **CRITICAL:** Fixed app.html to use SvelteKit placeholders (%sveltekit.head% and %sveltekit.body%) instead of vanilla Svelte structure
- **CRITICAL:** Fixed missing `TlsStream` import in imap_sync.rs that caused compilation failure - added `use native_tls::TlsStream`
- **CRITICAL:** Removed duplicate `TlsStream` import declaration in imap_sync.rs that appeared after ImapSync implementation
- **BUG:** Fixed deprecated `chrono::NaiveDateTime::from_timestamp` API call in export.rs - now uses `chrono::DateTime::from_timestamp` with proper error handling
- **BUG:** Removed conflicting main.ts and App.svelte files that were incompatible with SvelteKit routing
- **BUG:** Fixed AccountSwitcher component to properly handle Select onValueChange callback with type-safe value handling
- **BUG:** Fixed FolderList and MessageList components to use reactive mailbox store references directly
- **BUG:** Fixed MessageView component to properly display HTML and plain text email bodies with fallback handling for empty content
- **BUG:** Fixed routes structure - root route now redirects to /inbox
- **BUG:** Corrected Settings page route structure - moved from /routes/Settings.svelte to /routes/settings/+page.svelte for proper SvelteKit routing

### Changed
- **Dependencies:** Added paneforge (^0.0.6) to package.json for resizable panel functionality (corrected from ^0.2.0)
- **Dependencies:** Added testing framework dependencies: vitest, @testing-library/svelte, @vitest/ui, happy-dom for comprehensive frontend testing
- **Dependencies:** Added Rust crates: mime_guess, regex, uuid for enhanced functionality
- **Dependencies:** Updated Tauri feature flags to include dialog-all, fs-all, path-all for file system operations
- **Architecture:** Refactored routing to use proper SvelteKit page structure with dedicated inbox and settings routes
- **Architecture:** Extended database schema with drafts, signatures, settings, and attachment_data tables
- **Architecture:** Added core modules for drafts and attachments management in backend
- **Architecture:** Implemented comprehensive database indexing strategy for performance optimization
- **UI:** Simplified +layout.svelte to minimal layout wrapper, moved 3-pane UI to dedicated inbox page
- **UI:** Improved MessageList with empty state handling and read/unread visual distinction
- **UI:** Improved MessageView with better header layout, content display, and overflow handling
- **UI:** Settings page now includes account addition workflow with OAuth URL opening via Tauri shell API
- **Security:** Tauri allowlist now properly scoped for file system access (APPDATA, DOWNLOAD, DOCUMENT)
- **Build:** Package.json now includes test scripts for vitest integration

## [1.0.0] - 2025-11-01

### Added
- Initial implementation of DEmail application.
- Core architecture with Tauri, Rust, and SvelteKit.
- Account management with OAuth 2.0 for Google, Microsoft.
- Secure credential storage using OS keyring.
- IMAP synchronization engine with stateful sync and IDLE support.
- SMTP module for sending emails.
- Local caching of emails in an SQLite database.
- Data export utility to save emails as PDF, HTML, and TXT.
- Three-pane UI for email browsing.
- **Background Sync Service:** Added a background service to periodically sync all accounts.
- **Configuration UI:** Added a new Settings page to manage OAuth provider credentials.
- **Secure Client Secret Storage:** OAuth client secrets are now stored securely in the OS keyring.
- **Robust Error Handling:** Implemented proper error handling across the backend, removing all `.unwrap()` calls.
- **HTML Sanitization:** All incoming HTML email bodies are now sanitized using `ammonia` to prevent XSS attacks.
- **Modern UI Components:** Re-implemented the entire frontend using `shadcn-svelte` and `tailwindcss` for a modern and responsive UI.
- **Frontend Project Structure:** Established a proper SvelteKit project with Vite, TypeScript, and all necessary configurations.
- **Standard Logging:** Implemented standard Rust logging with `env_logger` for comprehensive application diagnostics.

### Fixed
- **CRITICAL:** Removed hardcoded OAuth credentials from the source code.
- **CRITICAL:** Fixed XSS vulnerability by sanitizing HTML emails before rendering.
- **CRITICAL:** Fixed TypeScript path configuration - corrected tsconfig.json to use `$lib` and `$lib/*` path mappings for proper SvelteKit module resolution.
- **CRITICAL:** Fixed Tauri version alignment - updated Tauri CLI from 2.9.2 to 1.5.14 to match Tauri runtime 1.6.1.
- **CRITICAL:** Removed tauri-plugin-log dependency and replaced with standard env_logger for Tauri 1.x compatibility.
- **CRITICAL:** Fixed lettre crate feature flags - changed from `tokio1-rustls-impl` to `tokio1-rustls-tls` for correct async TLS support.
- **BUG:** Fixed broken OAuth callback logic; accounts are now correctly created and persisted after authentication.
- **BUG:** Implemented access token refresh logic, making IMAP and SMTP operations functional.
- **BUG:** Corrected the unsafe database path to use the OS-specific application data directory.
- **BUG:** Replaced blocking network and database calls with asynchronous operations to prevent UI freezing.
- **BUG:** Fully implemented all previously stubbed API endpoints for fetching folders and messages.
- **BUG:** Re-implemented the PDF export feature using `headless_chrome` for reliable, non-interactive PDF generation.
- **BUG:** Removed triple-quote syntax artifacts from Rust source files (main.rs, api.rs, error.rs) that were preventing compilation.
- **BUG:** Removed triple-quote syntax artifacts from TypeScript source files (types/index.ts, stores/mailboxStore.ts) that were causing module resolution errors.
- **BUG:** Removed unused `paneforge` dependency that was causing peer dependency conflicts with Svelte 4.

### Changed
- **Architecture:** Refactored the backend to use a shared `AppState` for managing the database pool and application configuration.
- **Dependencies:** Added `ammonia` and `headless_chrome` to the backend. Added `vite`, `tailwindcss`, `shadcn-svelte`, and `bits-ui` to the frontend.
- **Dependencies:** Removed `tauri-plugin-log` (Tauri 2.x only) in favor of standard `env_logger` for Tauri 1.x compatibility.
- **Configuration:** Updated .gitignore to allow committing .dev-docs directory per project requirements.
- **Build System:** Updated npm scripts to remove deprecated svelte-kit sync command.