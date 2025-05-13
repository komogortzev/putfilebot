# putfilebot

**Bot to automatically save received media files (documents, photos, videos, etc.) to a local directory.**

This simple Telegram bot, written in Rust using the `teloxide` library, is designed to automatically download and save files sent by users to the bot's chat.

## Features

* Saves various media file types: documents, photos, videos, audio, and voice messages.
* Uses asynchronous operations (`tokio`) for efficient download handling.
* Sanitizes file names for safe saving on disk.
* Configurable via environment variables.

## Usage

These instructions will help you set up and run the file saver bot.

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.
* A Telegram bot token obtained from [@BotFather](https://t.me/botfather).

### Setup

1.  Clone the repository

2.  Create a `.env` file in the project's root directory (where `Cargo.toml` is located).

3.  Open the `.env` file and add the following lines, replacing the placeholders with your actual values:
    ```env
    TELOXIDE_TOKEN=YOUR_BOT_TOKEN
    DOWNLOAD_PATH=/path/to/your/download/directory
    ```
    * `YOUR_BOT_TOKEN`: The token obtained from BotFather.
    * `/path/to/your/download/directory`: The absolute or relative path to the directory where the bot will save files. Ensure the user running the bot has write permissions for this directory. The bot will attempt to create the directory if it doesn't exist.

### Building

Build the project using Cargo:

* For a debug build:
    ```bash
    cargo build
    ```
* For a release build (recommended for production, it's optimized and faster):
    ```bash
    cargo build --release
    ```

### Running

Run the bot. The logging level is controlled by the `RUST_LOG` environment variable.

* **Run using `cargo run` (for debugging):**
    ```bash
    RUST_LOG=info cargo run
    ```
    or for more detailed logs:
    ```bash
    RUST_LOG=debug cargo run
    ```

* **Run the compiled executable (recommended for production):**
    Navigate to the directory containing the executable (`target/debug/` or `target/release/`).
    * On Linux/macOS:
        ```bash
        RUST_LOG=info ./your_crate_name
        ```
    * On Windows (Command Prompt):
        ```cmd
        set RUST_LOG=info && .\your_crate_name.exe
        ```
    * On Windows (PowerShell):
        ```powershell
        $env:RUST_LOG="info"; .\your_crate_name.exe
        ```

Once started, the bot will begin receiving messages. Send it a document, photo, video, audio, or voice message, and it will attempt to save it to the specified directory.
