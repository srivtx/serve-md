#!/bin/sh
# serve-md installer
# This script detects your OS and architecture, downloads the latest
# serve-md binary from GitHub Releases, and installs it to a directory
# on your PATH.
#
# Usage:
#   curl -sSfL https://raw.githubusercontent.com/YOURNAME/serve-md/main/install.sh | sh

set -e

REPO="srivtx/serve-md"
NAME="serve-md"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

say() {
    printf "${BLUE}==>${NC} %s\n" "$1"
}

say_err() {
    printf "${RED}Error:${NC} %s\n" "$1" >&2
}

say_warn() {
    printf "${YELLOW}Warning:${NC} %s\n" "$1"
}

say_ok() {
    printf "${GREEN}==>${NC} %s\n" "$1"
}

# Detect the operating system
detect_os() {
    case "$(uname -s)" in
        Darwin)
            echo "apple-darwin"
            ;;
        Linux)
            echo "unknown-linux-gnu"
            ;;
        MINGW* | MSYS* | CYGWIN*)
            say_err "Windows is not supported by this installer."
            say "Please download the binary manually from:"
            say "  https://github.com/${REPO}/releases"
            exit 1
            ;;
        *)
            say_err "Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac
}

# Detect the CPU architecture
detect_arch() {
    case "$(uname -m)" in
        x86_64 | amd64)
            echo "x86_64"
            ;;
        arm64 | aarch64)
            echo "aarch64"
            ;;
        *)
            say_err "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac
}

# Get the latest release tag from GitHub API
get_latest_tag() {
    # Use curl to fetch the latest release from GitHub API
    tag=$(curl -sSfL --retry 3 --retry-delay 2 \
        -H "Accept: application/vnd.github.v3+json" \
        "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name":' \
        | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    
    if [ -z "$tag" ]; then
        say_err "Could not determine latest release."
        say "Please install manually from: https://github.com/${REPO}/releases"
        exit 1
    fi
    
    echo "$tag"
}

# Download the binary tarball
download_release() {
    local tag="$1"
    local os="$2"
    local arch="$3"
    
    local asset="${NAME}-${tag}-${arch}-${os}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${tag}/${asset}"
    local tmpdir
    tmpdir=$(mktemp -d)
    
    if ! curl -sSfL --retry 3 --retry-delay 2 "$url" -o "${tmpdir}/${asset}"; then
        say_err "Failed to download release asset."
        say "URL: ${url}"
        say "Your platform may not have a pre-built binary."
        say "You can build from source with: cargo install ${NAME}"
        rm -rf "$tmpdir"
        exit 1
    fi
    
    echo "$tmpdir/$asset"
}

# Install the binary to the target directory
install_binary() {
    local archive="$1"
    local install_dir="$2"
    
    say "Installing to ${install_dir}..."
    
    # Create the install directory
    mkdir -p "$install_dir"
    
    # Extract the binary from the tarball
    tar -xzf "$archive" -C "$install_dir"
    
    # Make it executable
    chmod +x "${install_dir}/${NAME}"
    
    say_ok "Binary installed to ${install_dir}/${NAME}"
}

# Ensure the install directory is on PATH
ensure_path() {
    local install_dir="$1"
    
    # Check if install_dir is already on PATH
    case ":$PATH:" in
        *":${install_dir}:"*)
            # Already on PATH
            return 0
            ;;
    esac
    
    say_warn "${install_dir} is not on your PATH."
    say "Adding to shell configuration files..."
    
    # Detect which shell config files exist
    local added=0
    
    if [ -f "$HOME/.bashrc" ]; then
        if ! grep -q "$install_dir" "$HOME/.bashrc" 2>/dev/null; then
            echo "" >> "$HOME/.bashrc"
            echo "# Added by serve-md installer" >> "$HOME/.bashrc"
            echo "export PATH=\"${install_dir}:\$PATH\"" >> "$HOME/.bashrc"
            say "  Added to ~/.bashrc"
            added=1
        fi
    fi
    
    if [ -f "$HOME/.zshrc" ]; then
        if ! grep -q "$install_dir" "$HOME/.zshrc" 2>/dev/null; then
            echo "" >> "$HOME/.zshrc"
            echo "# Added by serve-md installer" >> "$HOME/.zshrc"
            echo "export PATH=\"${install_dir}:\$PATH\"" >> "$HOME/.zshrc"
            say "  Added to ~/.zshrc"
            added=1
        fi
    fi
    
    if [ -f "$HOME/.config/fish/config.fish" ]; then
        if ! grep -q "$install_dir" "$HOME/.config/fish/config.fish" 2>/dev/null; then
            echo "" >> "$HOME/.config/fish/config.fish"
            echo "# Added by serve-md installer" >> "$HOME/.config/fish/config.fish"
            echo "fish_add_path ${install_dir}" >> "$HOME/.config/fish/config.fish"
            say "  Added to ~/.config/fish/config.fish"
            added=1
        fi
    fi
    
    if [ "$added" -eq 0 ]; then
        say_warn "Could not automatically add to PATH."
        say "Please add this line to your shell configuration:"
        say ""
        say "  export PATH=\"${install_dir}:\$PATH\""
        say ""
    fi
}

# Main installation flow
main() {
    say "serve-md installer"
    say ""
    
    # Detect platform
    local os
    os=$(detect_os)
    
    local arch
    arch=$(detect_arch)
    
    say "Detected platform: ${arch}-${os}"
    
    # Get latest release
    say "Checking for latest release..."
    local tag
    tag=$(get_latest_tag)
    say "Latest release: ${tag}"
    
    # Download
    say "Downloading ${NAME} ${tag} for ${arch}-${os}..."
    local archive
    archive=$(download_release "$tag" "$os" "$arch")
    
    # Determine install directory
    local install_dir
    if [ "$os" = "apple-darwin" ]; then
        # macOS: use ~/.local/bin or ~/.serve-md/bin
        if [ -d "$HOME/.local/bin" ]; then
            install_dir="$HOME/.local/bin"
        else
            install_dir="$HOME/.serve-md/bin"
        fi
    else
        # Linux: use ~/.local/bin (XDG standard)
        install_dir="$HOME/.local/bin"
    fi
    
    # Install
    install_binary "$archive" "$install_dir"
    
    # Ensure PATH
    ensure_path "$install_dir"
    
    # Cleanup
    local tmpdir
    tmpdir=$(dirname "$archive")
    rm -rf "$tmpdir"
    
    # Verify
    say ""
    say_ok "Installation complete!"
    say ""
    
    if command -v "$NAME" >/dev/null 2>&1; then
        local version
        version=$("${install_dir}/${NAME}" --version 2>/dev/null || echo "unknown")
        say "Installed version: ${version}"
        say ""
        say "Get started:"
        say "  cd my-docs/"
        say "  serve-md"
    else
        say_warn "Please open a new terminal window or run:"
        say "  source ~/.bashrc   # or ~/.zshrc"
        say ""
        say "Then you can run:"
        say "  serve-md --version"
    fi
}

main "$@"
