# serve-md

[![CI](https://github.com/srivtx/serve-md/actions/workflows/ci.yml/badge.svg)](https://github.com/srivtx/serve-md/actions/workflows/ci.yml)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/srivtx/serve-md)](https://github.com/srivtx/serve-md/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platforms](https://img.shields.io/badge/platform-macOS%20%7C%20Linux-blue)](https://github.com/srivtx/serve-md/releases)

> Serve Markdown files as a clean, navigable local website. **Zero configuration.**

```bash
cd my-docs/
serve-md
# → Server running at http://127.0.0.1:3000
```

No `mkdocs.yml`. No `_config.yml`. No `package.json`. Just run it.

## Why serve-md?

I was tired of documentation tools that demand you restructure your files or write config files. I just wanted to `cd` into a folder and type one command.

| Tool | Config Required | File Restructure | Binary Size |
|------|----------------|------------------|-------------|
| **serve-md** | ❌ None | ❌ None | ~2MB |
| Jekyll | ✅ `_config.yml` | ✅ Yes | ~50MB (Ruby) |
| MkDocs | ✅ `mkdocs.yml` | ✅ Yes | ~20MB (Python) |
| Docusaurus | ✅ `docusaurus.config.js` | ✅ Yes | ~100MB (Node) |

**Who should use this:**
- Developers who want to preview docs locally without setup
- Writers who organize notes in folders with Markdown
- Teams who want to share internal docs on the LAN
- Anyone who hates configuration files

## Features

- **Zero config** — Just run it in any directory
- **Clean URLs** — `guide.md` becomes `/guide`
- **Full-text search** — Search across all your docs instantly
- **Table of contents** — Auto-generated from headings
- **Dark mode** — Respects your OS preference
- **Static assets** — Images and files work automatically
- **`.gitignore` aware** — Won't leak your `.env` files
- **Path traversal impossible** — Security by design
- **Auto port discovery** — Finds an open port automatically
- **Graceful shutdown** — Press Ctrl+C, it cleans up properly

## Install

### One-line install (macOS / Linux)

```bash
curl -sSfL https://raw.githubusercontent.com/srivtx/serve-md/main/install.sh | sh
```

This downloads the correct pre-built binary for your platform and adds it to your PATH.

### With cargo

```bash
cargo install --git https://github.com/srivtx/serve-md
```

### Homebrew (macOS)

```bash
brew tap srivtx/serve-md
brew install serve-md
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/srivtx/serve-md/releases).

## Uninstall

```bash
curl -sSfL https://raw.githubusercontent.com/srivtx/serve-md/main/uninstall.sh | sh
```

Or if installed via cargo:

```bash
cargo uninstall serve-md
```

## Usage

```bash
serve-md                           # Serve current directory
serve-md ./docs                    # Serve specific directory
serve-md --port 8080               # Custom port
serve-md --bind 0.0.0.0            # Share on LAN
serve-md -o                        # Auto-open browser
```

## Security

- Hidden files (starting with `.`) are never served
- Path traversal is impossible — we only serve files we scanned
- File size limits prevent DoS (10MB markdown, 100MB static)
- Raw HTML in markdown is escaped to prevent XSS

## Tech Stack

- Rust + Tokio + Axum
- pulldown-cmark for Markdown parsing
- ignore crate for `.gitignore` support

## License

MIT
