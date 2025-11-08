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
- Core module: `src-tauri/src/core/reply_forward.rs` for email reply and forward message preparation with quoted body formatting
- Tauri commands: `mark_message_read`, `mark_message_unread`, `refresh_account`, `search_messages`
- Tauri commands: `save_draft`, `get_drafts`, `delete_draft` for draft management
- Tauri commands: `save_signature`, `get_signatures`, `delete_signature` for signature management
- Tauri commands: `download_attachment` for attachment file download to user-specified location
- Tauri commands: `get_messages_paginated`, `count_messages_in_folder` for pagination support
- Tauri commands: `delete_message`, `move_message` for message management operations
- Tauri commands: `save_setting`, `get_setting`, `get_all_settings` for application settings persistence
- Tauri commands: `prepare_reply`, `prepare_forward` for generating reply and forward message templates
- Tauri feature flags: `dialog-all`, `fs-all`, `path-all` for file system and dialog operations
- Tauri allowlist configuration: dialog permissions (open, save) for file picker operations
- Tauri allowlist configuration: fs permissions (readFile, writeFile) with scoped access to APPDATA, DOWNLOAD, DOCUMENT directories
- Database function: `get_attachments_for_message` for loading attachment metadata when viewing messages

**Frontend Components:**
- SearchBar component with debounced input (500ms delay) and clear button functionality
- ComposeEmail component as modal dialog for email composition with form validation and error handling
- DraftsList component for displaying saved drafts with edit and delete functionality
- DraftEditor component as modal dialog for creating and editing email drafts with full form support
- SignatureManager component for creating, editing, and managing email signatures with default signature designation
- MessagePagination component with page size selector (25, 50, 100, 200) and navigation controls for handling large message lists
- Context menu options in message list for manually marking messages as read or unread via right-click
- Visual distinction between read and unread messages in message list with mail icon indicators and bold font for unread messages
- Attachments display section in message view showing filename, MIME type, and file size for each attachment
- Attachment download button in MessageView component with file save dialog integration using Tauri dialog API
- Compose button in inbox header with Pencil icon for creating emails
- Refresh button in inbox header with animated spinning icon during synchronization
- Search bar integrated into message list pane header for quick email search
- Resizable pane components using paneforge library for adjustable 3-pane email layout

**Frontend API & Store:**
- Frontend API methods: `markMessageRead`, `markMessageUnread`, `refreshAccount`, `searchMessages` in services/api.ts
- Frontend API methods: `saveDraft`, `getDrafts`, `deleteDraft` for draft management operations
- Frontend API methods: `saveSignature`, `getSignatures`, `deleteSignature` for signature management operations
- Frontend API methods: `downloadAttachment` for attachment download with file path parameter
- Frontend API methods: `getMessagesPaginated`, `countMessagesInFolder` for paginated message retrieval
- Frontend API methods: `deleteMessage`, `moveMessage` for message management operations
- Frontend API methods: `saveSetting`, `getSetting`, `getAllSettings` for application settings management
- Frontend API methods: `prepareReply`, `prepareForward` for generating reply and forward message data
- Store methods in mailboxStore for read status management (`markRead`, `markUnread`), account refresh (`refreshAccount`), and message search (`searchInMessages`)
- Store methods in mailboxStore for message operations (`deleteMessage`, `moveMessage`)
- Type definitions: `Draft`, `EmailSignature`, `AppSetting` interfaces in types/index.ts

**Routing & Pages:**
- OAuth callback page at `/callback` route for handling authentication redirects
- Inbox page at `/inbox` route with full 3-pane email client interface
- Index exports for Select UI component for proper module resolution
- Improved Settings page with account addition functionality and status feedback

**Build & Development:**
- Application icon files in all required formats (32x32.png, 128x128.png, 128x128@2x.png, icon.ico, icon.icns) for complete Tauri build support across all platforms (Windows, macOS, Linux)
- Vitest configuration file (vitest.config.ts) for frontend unit testing setup with happy-dom environment
- Test setup file (tests/frontend/setup.ts) with beforeAll, afterEach, and afterAll hooks for test environment initialization
- Comprehensive frontend tests in tests/frontend/api.test.ts covering all API methods including drafts, signatures, attachments, and pagination
- Comprehensive backend tests in tests/backend/database_tests.rs covering database operations, FTS5, pagination, drafts, and signatures
- NPM scripts: `test`, `test:ui`, `test:coverage` for running frontend tests
- Testing dependencies: vitest (^0.34.0), @testing-library/svelte (^4.0.0), @vitest/ui (^0.34.0), happy-dom (^12.0.0)

**Dependencies:**
- Rust dependencies: `mime_guess` (2.0) for MIME type detection in attachment handling
- Rust dependencies: `regex` (1.10) for email address validation and pattern matching
- Rust dependencies: `uuid` (1.6) with v4 and serde features for unique identifier generation

**Modernization & Security (2025-11-08):**
- Input validation system for all user inputs with email address validation using validator crate, path sanitization using sanitize-filename crate, and comprehensive length limits
- Core validation module (src-tauri/src/core/validation.rs) with functions for validating emails, subjects, bodies, file paths, and preventing SQL injection patterns
- Database migration system with version tracking in migrations table for schema evolution management without data loss
- Database schema extensions: `migrations` table for tracking applied schema versions, `threads` table for email threading with subject hashing, `contacts` table for email address autocomplete
- Database schema additions: `is_starred` column on messages table for marking important emails, `thread_id` column on messages table for conversation grouping
- Configuration files: postcss.config.js for Tailwind CSS processing, tailwind.config.js for theme configuration with dark mode support
- Code quality tooling: .prettierrc for code formatting, .eslintrc.json for linting rules, rustfmt.toml for Rust code style
- NPM scripts: `format`, `lint`, `lint:fix` for automated code quality checks
- Build configuration: .gitignore updated to exclude .dev-docs directory per project hygiene requirements
- **Phase 2 Implementation:** Database connection pooling using r2d2 (0.8.10) and r2d2_sqlite (0.24.0) for non-blocking multi-threaded database access with max 15 connections
- **Phase 2 Implementation:** Structured logging system using tracing (0.1.40) and tracing-subscriber (0.3.18) replacing env_logger for thread-aware diagnostic logging
- **Phase 2 Implementation:** Comprehensive input validation on all API endpoints preventing SQL injection, XSS, and path traversal attacks
- **Phase 2 Implementation:** Message starring functionality with backend support (star_message, unstar_message, get_starred_messages functions in db.rs)
- **Phase 2 Implementation:** Bulk operations support (bulk_mark_read, bulk_delete_messages functions in db.rs) for multi-message management
- **Phase 2 Implementation:** Threading and contacts integration stubs in IMAP sync for Phase 3 full implementation
- **Phase 2 Implementation:** Validation test suite (validation_tests.rs) with 10+ comprehensive integration tests covering edge cases, Unicode support, and security scenarios
- **Phase 2 Implementation:** Updated Message and MessageHeader models to include is_starred and thread_id fields for starring and conversation threading support

### Fixed
- **CRITICAL BUILD:** Fixed SvelteKit adapter configuration - added fallback: 'index.html' to adapter-static for proper SPA mode enabling production builds for Tauri desktop application
- **CRITICAL BUILD:** Created src/routes/+layout.ts with prerender: false and ssr: false to disable server-side rendering for Tauri desktop application
- Accessibility improvement: Added keyboard event handler (Escape key) and ARIA role to context menu in MessageList component for compliance with a11y guidelines
- Corrected paneforge dependency version in package.json from non-existent ^0.2.0 to ^0.0.6 for Svelte 4 compatibility
- **CRITICAL:** Fixed type mismatch in Attachment model - changed `local_path` field from `String` to `Option<String>` in models.rs and database schema to allow NULL values for attachments without local storage
- **CRITICAL:** Removed triple-quote syntax artifacts from multiple Svelte and TypeScript component files that prevented compilation (api.ts, utils.ts, AccountSwitcher.svelte, FolderList.svelte, MessageList.svelte, MessageView.svelte, +page.svelte, Settings.svelte, +layout.svelte)
- **CRITICAL:** Fixed undefined variable `folders` in imap_sync.rs fetch_folders function by adding proper Vec initialization
- **CRITICAL:** Fixed app.html to use SvelteKit placeholders (%sveltekit.head% and %sveltekit.body%) instead of vanilla Svelte structure
- **CRITICAL:** Fixed missing `TlsStream` import in imap_sync.rs that caused compilation failure - added `use native_tls::TlsStream`
- **CRITICAL:** Removed duplicate `TlsStream` import declaration in imap_sync.rs that appeared after ImapSync implementation
- **CRITICAL:** Fixed AppState structure to use Arc-wrapped fields (Arc<Mutex<Connection>>, Arc<Mutex<Config>>) for proper thread-safe sharing across background sync and API handlers
- **CRITICAL SECURITY:** HTML sanitization now actively implemented using ammonia crate in save_message function to prevent XSS attacks from malicious email HTML content
- **CRITICAL:** IMAP authentication fixed - now passes actual user email address instead of empty string to login_with_oauth2 function
- **CRITICAL:** SMTP authentication fixed - send_email function now receives and uses actual user email address for OAuth2 XOAUTH2 authentication
- **CRITICAL:** Attachment data now properly saved during IMAP sync - parses mail-parser attachment objects, saves metadata to attachments table and binary data to attachment_data table
- **CRITICAL:** Attachment loading implemented in get_message_details - now queries and returns actual attachment metadata instead of empty Vec
- **CRITICAL:** CC header now properly extracted and saved from parsed email messages during IMAP sync
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
- **Dependencies:** Removed unused Rust crates from Cargo.toml: sha2, base64, rand, url, anyhow, regex for reduced binary size
- **Dependencies:** Added Rust crates: mime_guess, regex, uuid for enhanced functionality
- **Dependencies:** Updated Tauri feature flags to include dialog-all, fs-all, path-all for file system operations
- **Dependencies (Modernization):** Updated rusqlite from 0.29.0 to 0.32.1 for improved error handling, performance, and modern API support
- **Dependencies (Modernization):** Updated ammonia from 3.3.0 to 4.1.2 for enhanced HTML sanitization security and XSS protection
- **Dependencies (Modernization):** Updated lettre from 0.10.4 to 0.11.10 for better async SMTP operations and modern email sending API
- **Dependencies (Modernization):** Updated mail-parser from 0.8.1 to 0.9.4 for improved email parsing accuracy and header extraction
- **Dependencies (Modernization):** Added r2d2 (0.8.10) and r2d2_sqlite (0.24.0) for database connection pooling (planned implementation)
- **Dependencies (Modernization):** Added tracing (0.1.40) and tracing-subscriber (0.3.18) for structured logging with JSON output support (planned implementation)
- **Dependencies (Modernization):** Added validator (0.18.1) for RFC-compliant email and URL validation
- **Dependencies (Modernization):** Added sanitize-filename (0.5.0) for secure file path handling and directory traversal prevention
- **Dependencies (Modernization):** Added once_cell (1.20.2) for thread-safe lazy static initialization
- **Dependencies (Frontend):** Added @tailwindcss/typography (^0.5.10) for better email content rendering
- **Dependencies (Frontend):** Added date-fns (^2.30.0) for human-friendly date formatting
- **Dependencies (Frontend):** Added dompurify (^3.0.6) and @types/dompurify (^3.0.5) for client-side HTML sanitization
- **Dependencies (Dev):** Added prettier (^3.1.0) and prettier-plugin-svelte (^3.1.2) for code formatting
- **Dependencies (Dev):** Added eslint (^8.54.0), @typescript-eslint/eslint-plugin (^6.13.1), @typescript-eslint/parser (^6.13.1), eslint-plugin-svelte (^2.35.1) for linting
- **Dependencies (Dev):** Added @vitest/coverage-v8 (^0.34.0) for test coverage reporting
- **Architecture:** Refactored routing to use proper SvelteKit page structure with dedicated inbox and settings routes
- **Architecture:** Extended database schema with drafts, signatures, settings, and attachment_data tables
- **Architecture:** Added core modules for drafts and attachments management in backend
- **Architecture:** Added validation, migrations, threading, and contacts modules to core backend architecture
- **Architecture:** Implemented comprehensive database indexing strategy for performance optimization with indexes on is_starred, thread_id, subject_hash, and contact lookup fields
- **Architecture (Phase 2):** Migrated from single Connection to r2d2 Pool throughout entire backend (db.rs, api.rs, auth.rs, accounts.rs, drafts.rs, imap_sync.rs, export.rs)
- **Architecture (Phase 2):** Refactored AppState to use Arc<Pool> instead of Arc<Mutex<Connection>> for thread-safe non-blocking database access
- **Architecture (Phase 2):** All database operations now use connection pool pattern with proper error handling and automatic connection management
- **Architecture (Phase 2):** All API command handlers now include comprehensive input validation before processing user data
- **UI:** Simplified +layout.svelte to minimal layout wrapper, moved 3-pane UI to dedicated inbox page
- **UI:** Improved MessageList with empty state handling and read/unread visual distinction
- **UI:** Improved MessageView with better header layout, content display, and overflow handling
- **UI:** Settings page now includes account addition workflow with OAuth URL opening via Tauri shell API
- **Security:** Tauri allowlist now properly scoped for file system access (APPDATA, DOWNLOAD, DOCUMENT)
- **Security:** All API inputs now validated before processing to prevent injection attacks and malformed data
- **Security:** File paths sanitized and validated against allowed directories to prevent path traversal attacks
- **Build:** Package.json now includes test scripts for vitest integration and new format/lint scripts
- **Build:** Tailwind CSS properly configured with PostCSS for frontend styling
- **Error Handling:** Added Validation error variant to DEmailError enum for input validation failures

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