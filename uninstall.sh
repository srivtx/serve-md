#!/bin/sh
# serve-md uninstaller
# Removes serve-md from your system
#
# Usage:
#   curl -sSfL https://raw.githubusercontent.com/srivtx/serve-md/main/uninstall.sh | sh

set -e

NAME="serve-md"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

say() {
    printf "${BLUE}==>${NC} %s\n" "$1"
}

say_ok() {
    printf "${GREEN}==>${NC} %s\n" "$1"
}

say_err() {
    printf "${RED}Error:${NC} %s\n" "$1" >&2
}

# Find where the binary is installed
find_binary() {
    if command -v "$NAME" >/dev/null 2>&1; then
        command -v "$NAME"
    else
        echo ""
    fi
}

# Remove PATH entries from shell configs
remove_path_entries() {
    local install_dir="$1"
    local removed=0

    # Remove lines that contain "Added by serve-md installer" or the PATH export
    for config in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.config/fish/config.fish"; do
        if [ -f "$config" ]; then
            # Create a backup
            cp "$config" "${config}.backup.$(date +%s)"
            
            # Remove lines added by our installer
            # We match either the comment or the export line
            if grep -q "Added by serve-md installer" "$config" 2>/dev/null; then
                sed -i.bak '/# Added by serve-md installer/d' "$config"
                sed -i.bak "/export PATH=\"${install_dir}:/d" "$config"
                sed -i.bak "/fish_add_path ${install_dir}/d" "$config"
                
                # Remove .bak files created by sed
                rm -f "${config}.bak"
                
                say "  Cleaned up ${config}"
                removed=1
            fi
            
            # Remove the timestamped backup if nothing changed
            if diff -q "$config" "${config}.backup.$(date +%s)" >/dev/null 2>&1; then
                rm -f "${config}.backup.$(date +%s)"
            fi
        fi
    done
    
    if [ "$removed" -eq 1 ]; then
        say_ok "PATH entries removed from shell configs"
        say "Open a new terminal for changes to take effect"
    fi
}

# Main uninstall flow
main() {
    say "serve-md uninstaller"
    say ""
    
    # Find the binary
    local binary_path
    binary_path=$(find_binary)
    
    if [ -z "$binary_path" ]; then
        say_err "serve-md not found on PATH"
        say "It may have been installed to a custom location or already removed"
        exit 1
    fi
    
    say "Found: ${binary_path}"
    
    # Get the directory containing the binary
    local install_dir
    install_dir=$(dirname "$binary_path")
    
    # Remove the binary
    say "Removing binary..."
    rm -f "$binary_path"
    say_ok "Binary removed"
    
    # Remove shell config entries
    say "Removing PATH entries..."
    remove_path_entries "$install_dir"
    
    # Check if the install directory is now empty and remove it
    if [ -d "$install_dir" ]; then
        if [ "$(ls -A "$install_dir" 2>/dev/null)" = "" ]; then
            say "Removing empty directory: ${install_dir}"
            rmdir "$install_dir"
            
            # If it was ~/.serve-md/bin, also remove ~/.serve-md
            local parent_dir
            parent_dir=$(dirname "$install_dir")
            if [ "$(basename "$parent_dir")" = ".serve-md" ] && [ -d "$parent_dir" ]; then
                if [ "$(ls -A "$parent_dir" 2>/dev/null)" = "" ]; then
                    say "Removing empty directory: ${parent_dir}"
                    rmdir "$parent_dir"
                fi
            fi
        fi
    fi
    
    say ""
    say_ok "serve-md has been uninstalled"
    say ""
    say "If you installed via cargo, also run:"
    say "  cargo uninstall serve-md"
}

main "$@"
