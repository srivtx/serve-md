# serve-md

Serve Markdown files as a clean, navigable local website. Zero configuration.

![demo](https://user-images.githubusercontent.com/placeholder/demo.gif)

## Quick Start

```bash
cd my-docs/
serve-md
```

Then open `http://localhost:3000` in your browser.

## Features

- **Zero config** — Just run it in any directory
- **Clean URLs** — `guide.md` becomes `/guide`
- **Table of contents** — Auto-generated from headings
- **Dark mode** — Respects your OS preference
- **Static assets** — Images and files work automatically
- **`.gitignore` aware** — Won't leak your `.env` files
- **Secure** — Path traversal is impossible

## Install

### macOS + Linux (Recommended)

```bash
curl -sSfL https://raw.githubusercontent.com/srivtx/serve-md/main/install.sh | sh
```

### With cargo

```bash
cargo install serve-md
```

### Homebrew (macOS)

```bash
brew tap srivtx/serve-md
brew install serve-md
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/srivtx/serve-md/releases).

## Usage

```bash
serve-md                           # Serve current directory
serve-md ./docs                    # Serve specific directory
serve-md --port 8080               # Custom port
serve-md --bind 0.0.0.0            # Share on LAN
serve-md -o                        # Auto-open browser
```

## Why?

I was tired of:
- Jekyll forcing me to restructure my files
- MkDocs needing a `mkdocs.yml`
- Node.js servers being heavy and slow

I just wanted to `cd` into a folder and type one command.

## Tech Stack

- Rust + Tokio + Axum
- pulldown-cmark for Markdown parsing
- ignore crate for `.gitignore` support

## License

MIT
