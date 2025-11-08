# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Resizable pane components using paneforge library for adjustable 3-pane email layout
- OAuth callback page at `/callback` route for handling authentication redirects
- Inbox page at `/inbox` route with full 3-pane email client interface
- Index exports for Select UI component for proper module resolution
- Improved Settings page with account addition functionality and status feedback

### Fixed
- **CRITICAL:** Removed triple-quote syntax artifacts from multiple Svelte and TypeScript component files that prevented compilation (api.ts, utils.ts, AccountSwitcher.svelte, FolderList.svelte, MessageList.svelte, MessageView.svelte, +page.svelte, Settings.svelte, +layout.svelte)
- **CRITICAL:** Fixed undefined variable `folders` in imap_sync.rs fetch_folders function by adding proper Vec initialization
- **CRITICAL:** Fixed app.html to use SvelteKit placeholders (%sveltekit.head% and %sveltekit.body%) instead of vanilla Svelte structure
- **BUG:** Fixed deprecated `chrono::NaiveDateTime::from_timestamp` API call in export.rs - now uses `chrono::DateTime::from_timestamp` with proper error handling
- **BUG:** Removed conflicting main.ts and App.svelte files that were incompatible with SvelteKit routing
- **BUG:** Fixed AccountSwitcher component to properly handle Select onValueChange callback with type-safe value handling
- **BUG:** Fixed FolderList and MessageList components to use reactive mailbox store references directly
- **BUG:** Fixed MessageView component to properly display HTML and plain text email bodies with fallback handling for empty content
- **BUG:** Fixed routes structure - root route now redirects to /inbox, Settings moved to /settings route

### Changed
- **Dependencies:** Added paneforge (^0.2.0) to package.json for resizable panel functionality
- **Architecture:** Refactored routing to use proper SvelteKit page structure with dedicated inbox and settings routes
- **UI:** Simplified +layout.svelte to minimal layout wrapper, moved 3-pane UI to dedicated inbox page
- **UI:** Improved MessageList with empty state handling and read/unread visual distinction
- **UI:** Improved MessageView with better header layout, content display, and overflow handling
- **UI:** Settings page now includes account addition workflow with OAuth URL opening via Tauri shell API

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