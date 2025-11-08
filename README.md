# DEmail

A sovereign desktop email client.

## About

DEmail is a desktop email client built on the principles of data sovereignty, privacy, and user control. It allows you to connect to your existing email accounts and also provides a powerful utility to export your entire mailbox to an open, local format, freeing you from platform lock-in.

This project was designed and originated by **orpheus497**.

## Features

### Email Management

- Connect to Google, Microsoft, and other email providers via OAuth 2.0
- Full email client functionality: read, write, and manage your emails
- Draft management with auto-save functionality
- Email signatures with HTML and plain text support
- Full-text search across all emails using SQLite FTS5
- Read/unread message status tracking
- Message pagination for performance with large mailboxes
- Attachment viewing with file size and MIME type display

### Security & Privacy

- Securely stores your credentials in your operating system's native keyring
- HTML email sanitization to prevent XSS attacks
- No telemetry or tracking - your data stays on your machine

### Data Sovereignty

- **Data Export:** Archive your entire mailbox to your local machine in a human-readable format (PDF, HTML, TXT)
- All emails stored locally in SQLite database
- Complete control over your email data

### User Experience

- Modern, responsive UI built with SvelteKit and `shadcn-svelte`
- Resizable 3-pane layout for comfortable email browsing
- Background sync service for keeping emails up-to-date
- Manual refresh for immediate synchronization
- Keyboard shortcuts and context menus for power users

## Configuration

Before adding an account, you must configure the OAuth credentials for your email provider (e.g., Google, Microsoft). This is done in the **Settings** page of the application.

You will need to create an OAuth 2.0 application in your provider's developer console (e.g., Google Cloud Platform, Azure Active Directory) and obtain a **Client ID** and **Client Secret**.

- The **Client ID** is stored in a `config.json` file in your application's configuration directory.
- The **Client Secret** is stored securely in your operating system's native keyring.

## Technology Stack

- **Core:** Tauri (Rust + WebView)
- **Backend:** Rust, Tokio, `ammonia`, `headless_chrome`
- **Frontend:** SvelteKit, `shadcn-svelte`, Tailwind CSS
- **Database:** SQLite

## FOSS Attribution

This project is built with and relies on the following open-source software:

| Component/Library       | Author/Maintainer                | License             |
| :---------------------- | :------------------------------- | :------------------ |
| Tauri                   | Tauri-Apps Team                  | MIT / Apache-2.0    |
| Rust                    | Rust Foundation                  | MIT / Apache-2.0    |
| SvelteKit               | Vercel Inc.                      | MIT                 |
| imap crate              | Jon Gjengset et al.              | MIT / Apache-2.0    |
| lettre crate            | lettre-rs developers             | MIT                 |
| oauth2 crate            | Corentin Henry et al.            | MIT / Apache-2.0    |
| SQLite / rusqlite       | D. Richard Hipp / rusqlite team  | Public Domain / MIT |
| tokio                   | Tokio project authors            | MIT                 |
| mail-parser crate       | mail-parser developers           | MIT                 |
| ammonia                 | The `ammonia` developers         | MIT / Apache-2.0    |
| headless_chrome         | The `headless_chrome` developers | MIT                 |
| shadcn-svelte           | Huntabyte                        | MIT                 |
| bits-ui                 | Huntabyte                        | MIT                 |
| tailwindcss             | Tailwind Labs                    | MIT                 |
| lucide-svelte           | Lucide contributors              | ISC                 |
| keyring                 | keyring-rs developers            | MIT / Apache-2.0    |
| mime_guess              | Contributors                     | MIT                 |
| regex                   | The Rust Project Developers      | MIT / Apache-2.0    |
| uuid                    | uuid-rs developers               | MIT / Apache-2.0    |
| vitest                  | Anthony Fu et al.                | MIT                 |
| @testing-library/svelte | Testing Library                  | MIT                 |
| happy-dom               | Capricorn86                      | MIT                 |
| paneforge               | Huntabyte                        | MIT                 |

## Getting Started

### Prerequisites

Before building DEmail, you need to install system dependencies for Tauri on Linux:

**Debian/Ubuntu:**

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora:**

```bash
sudo dnf install webkit2gtk4.0-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel
sudo dnf group install "C Development Tools and Libraries"
```

**Arch Linux:**

```bash
sudo pacman -Syu
sudo pacman -S webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips
```

### Building and Running

1.  Install dependencies: `npm install`
2.  Run the development server: `npm run tauri dev`
3.  Build for production: `npm run tauri build`

### Testing

Run frontend tests:

```bash
npm test              # Run tests once
npm run test:ui       # Run tests with UI
npm run test:coverage # Generate coverage report
```

## License

This project is licensed under the MIT License.
