set -euo pipefail

REPO="princepal9120/devrunner"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="devrunner"
SUDO_CMD=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Icons
INFO="🔍"
PACKAGE="📦"
SUCCESS="✓"
WARNING="⚠"
ERROR="❌"

print_info() {
    echo -e "${BLUE}${INFO} $1${NC}"
}

print_success() {
    echo -e "${GREEN}${SUCCESS} $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}${WARNING} $1${NC}"
}

print_error() {
    echo -e "${RED}${ERROR} $1${NC}"
}

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)

    case "$os" in
        linux)
            OS="linux"
            ;;
        darwin)
            OS="macos"
            ;;
        mingw*|msys*|cygwin*)
            OS="windows"
            ;;
        *)
            print_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac

    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            print_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac

    PLATFORM="${OS}-${ARCH}"
    print_info "Detected platform: ${PLATFORM}"
}

select_install_dir() {
    local user_dir="${HOME}/.local/bin"

    if mkdir -p "$user_dir" 2>/dev/null && [ -w "$user_dir" ]; then
        INSTALL_DIR="$user_dir"
        SUDO_CMD=""
        return
    fi

    if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
        INSTALL_DIR="/usr/local/bin"
        SUDO_CMD=""
        return
    fi

    if command -v sudo >/dev/null 2>&1; then
        INSTALL_DIR="/usr/local/bin"
        SUDO_CMD="sudo"
        return
    fi

    print_error "Could not find a writable install directory"
    print_error "Create ${user_dir} or install sudo to allow /usr/local/bin fallback"
    exit 1
}

# Get the latest release version
get_latest_version() {
    print_info "Fetching latest version..."

    local response=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest")
    LATEST_VERSION=$(echo "$response" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/' || true)

    if [ -z "$LATEST_VERSION" ]; then
        print_error "Failed to fetch latest version"
        exit 1
    fi

    print_info "Latest version: ${LATEST_VERSION}"
}

# Download and verify binary
download_binary() {
    # Try devrunner- first, then run-
    local asset_base="devrunner-${PLATFORM}"
    local asset_legacy="run-${PLATFORM}"
    
    if [ "$OS" = "windows" ]; then
        asset_base="${asset_base}.exe"
        asset_legacy="${asset_legacy}.exe"
    fi

    local download_url="https://github.com/${REPO}/releases/download/${LATEST_VERSION}/${asset_base}"
    local asset_name="${asset_base}"

    # Check if devrunner asset exists (using curl -I for HEAD request)
    # If it fails, fallback to legacy name
    if ! curl -fsSI "$download_url" > /dev/null; then
        asset_name="${asset_legacy}"
        download_url="https://github.com/${REPO}/releases/download/${LATEST_VERSION}/${asset_name}"
    fi

    local checksum_url="${download_url}.sha256"

    print_info "Downloading ${asset_name}..."

    local tmp_dir=$(mktemp -d)
    local tmp_binary="${tmp_dir}/${asset_name}"
    local tmp_checksum="${tmp_dir}/${asset_name}.sha256"

    # Download binary with original asset name for checksum verification
    if ! curl -fsSL "$download_url" -o "$tmp_binary"; then
        print_error "Failed to download binary"
        rm -rf "$tmp_dir"
        exit 1
    fi

    # Download and verify checksum (required)
    print_info "Verifying checksum..."
    if ! curl -fsSL "$checksum_url" -o "$tmp_checksum"; then
        print_error "Failed to download checksum file: ${checksum_url}"
        rm -rf "$tmp_dir"
        exit 1
    fi

    local expected_hash
    expected_hash=$(awk '{print tolower($1)}' "$tmp_checksum")
    if [ -z "$expected_hash" ]; then
        print_error "Invalid checksum file format for ${asset_name}"
        rm -rf "$tmp_dir"
        exit 1
    fi

    local actual_hash
    if command -v sha256sum >/dev/null 2>&1; then
        actual_hash=$(sha256sum "$tmp_binary" | awk '{print tolower($1)}')
    elif command -v shasum >/dev/null 2>&1; then
        actual_hash=$(shasum -a 256 "$tmp_binary" | awk '{print tolower($1)}')
    else
        print_error "No SHA256 tool found (need sha256sum or shasum)"
        rm -rf "$tmp_dir"
        exit 1
    fi

    if [ "$actual_hash" != "$expected_hash" ]; then
        print_error "Checksum mismatch for ${asset_name}"
        rm -rf "$tmp_dir"
        exit 1
    fi

    print_success "Checksum verified"

    # Rename binary to final name
    mv "$tmp_binary" "${tmp_dir}/${BINARY_NAME}"

    # Create install directory if needed
    if [ -n "$SUDO_CMD" ]; then
        $SUDO_CMD mkdir -p "$INSTALL_DIR"
    else
        mkdir -p "$INSTALL_DIR"
    fi

    # Install binary
    print_info "Installing to ${INSTALL_DIR}/${BINARY_NAME}..."
    if [ -n "$SUDO_CMD" ]; then
        $SUDO_CMD mv "${tmp_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
        $SUDO_CMD chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    else
        mv "${tmp_dir}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
        chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    fi

    # Cleanup
    rm -rf "$tmp_dir"
}

# Check if directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        print_warning "${INSTALL_DIR} is not in your PATH"
        echo ""
        echo "Add the following line to your shell configuration file:"
        echo ""

        local shell_name=$(basename "$SHELL")
        case "$shell_name" in
            bash)
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
                echo "  source ~/.bashrc"
                ;;
            zsh)
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
                echo "  source ~/.zshrc"
                ;;
            fish)
                echo "  fish_add_path ~/.local/bin"
                ;;
            *)
                echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
                ;;
        esac
        echo ""
    fi
}


main() {
    # ASCII Art Banner
    echo ""
    echo -e "${BLUE}"
    echo "  ██████╗ ███████╗██╗   ██╗██████╗ ██╗   ██╗███╗   ██╗███╗   ██╗███████╗██████╗ "
    echo "  ██╔══██╗██╔════╝██║   ██║██╔══██╗██║   ██║████╗  ██║████╗  ██║██╔════╝██╔══██╗"
    echo "  ██║  ██║█████╗  ██║   ██║██████╔╝██║   ██║██╔██╗ ██║██╔██╗ ██║█████╗  ██████╔╝"
    echo "  ██║  ██║██╔══╝  ╚██╗ ██╔╝██╔══██╗██║   ██║██║╚██╗██║██║╚██╗██║██╔══╝  ██╔══██╗"
    echo "  ██████╔╝███████╗ ╚████╔╝ ██║  ██║╚██████╔╝██║ ╚████║██║ ╚████║███████╗██║  ██║"
    echo "  ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝"
    echo -e "${NC}"
    echo -e "${GREEN}                   🚀 Universal Task Runner${NC}"
    echo ""

    detect_platform
    select_install_dir
    print_info "Install directory: ${INSTALL_DIR}"
    get_latest_version
    download_binary
    check_path

    echo ""
    echo -e "${GREEN}  ✅ Installation complete!${NC}"
    echo ""
    echo -e "  Run ${BLUE}devrunner --help${NC} to get started"
    echo -e "  Example: ${BLUE}devrunner test${NC} or ${BLUE}devrunner build${NC}"
    echo ""
}

main "$@"
