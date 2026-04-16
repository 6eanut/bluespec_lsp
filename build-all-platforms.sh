#!/bin/bash
set -e

# Build script for all platforms
# This script helps test the build process locally

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"
BUILD_DIR="$PROJECT_DIR/dist"

echo "=== Building BSV Language Server for all platforms ==="

# Clean previous builds
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Function to build for a specific platform
build_platform() {
    local platform=$1
    local os=$2
    local arch=$3
    local rust_target=$4
    local artifact_suffix=$5

    echo ""
    echo "=== Building for $platform ($os/$arch) ==="
    echo "Rust target: $rust_target"
    echo "Artifact suffix: $artifact_suffix"

    # Create platform directory
    local platform_dir="$BUILD_DIR/$artifact_suffix"
    mkdir -p "$platform_dir"

    # Build tree-sitter-bsv
    echo "Building tree-sitter-bsv..."
    cd "$PROJECT_DIR/tree-sitter-bsv"

    # Clean previous builds
    make clean 2>/dev/null || true
    rm -f libtree-sitter-bsv.* 2>/dev/null || true

    # Generate parser and build
    tree-sitter generate
    make

    # Copy appropriate library file
    if [[ "$os" == "windows" ]]; then
        cp libtree-sitter-bsv.dll "$platform_dir/" 2>/dev/null || echo "Warning: No DLL file found"
    elif [[ "$os" == "darwin" ]]; then
        cp libtree-sitter-bsv.dylib "$platform_dir/" 2>/dev/null || echo "Warning: No dylib file found"
    else
        cp libtree-sitter-bsv.so "$platform_dir/" 2>/dev/null || echo "Warning: No SO file found"
    fi

    # Build Rust language server
    echo "Building Rust language server..."
    cd "$PROJECT_DIR/bsv-language-server"

    # Clean previous builds
    cargo clean

    # Build for target
    echo "Building for target: $rust_target"
    rustup target add "$rust_target" 2>/dev/null || true
    cargo build --release --target "$rust_target"

    # Copy binary with appropriate extension
    if [[ "$os" == "windows" ]]; then
        cp "target/$rust_target/release/bsv-language-server.exe" "$platform_dir/"
    else
        cp "target/$rust_target/release/bsv-language-server" "$platform_dir/"
    fi

    # Build TypeScript client
    echo "Building TypeScript client..."
    npm ci
    npm run compile

    # Package VSIX
    echo "Packaging VSIX..."

    # Backup original package.json
    cp package.json package.json.backup

    # Update package.json with platform-specific binary path
    node -e "
      const fs = require('fs');
      const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8'));

      // Update binary path for this platform
      pkg.contributes.configuration.properties['bsv.languageServer.path'].default = './dist/$artifact_suffix/bsv-language-server${\"$os\" === 'windows' ? '.exe' : ''}';

      fs.writeFileSync('package.json', JSON.stringify(pkg, null, 2));
    "

    # Install vsce if not present
    if ! command -v vsce &> /dev/null; then
        echo "Installing vsce..."
        npm install -g @vscode/vsce
    fi

    # Package VSIX
    vsce package --out "$platform_dir/bsv-language-server-$artifact_suffix.vsix"

    # Restore original package.json
    mv package.json.backup package.json

    echo "✓ Built $platform: $platform_dir/bsv-language-server-$artifact_suffix.vsix"
}

# Check for required tools
echo "Checking required tools..."

if ! command -v tree-sitter &> /dev/null; then
    echo "Error: tree-sitter CLI not found. Install with: npm install -g tree-sitter-cli"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: Rust cargo not found. Install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "Error: npm not found. Install Node.js from https://nodejs.org/"
    exit 1
fi

# Note: Local builds are limited to the host platform
# For cross-compilation, you'll need appropriate toolchains installed

echo ""
echo "Building for host platform only (cross-compilation requires additional setup)"
echo "Host platform detection..."

# Detect host platform
case "$(uname -s)" in
    Linux*)
        HOST_OS="linux"
        if [[ "$(uname -m)" == "aarch64" ]]; then
            HOST_ARCH="arm64"
            HOST_RUST_TARGET="aarch64-unknown-linux-gnu"
            HOST_SUFFIX="linux-arm64"
        else
            HOST_ARCH="x64"
            HOST_RUST_TARGET="x86_64-unknown-linux-gnu"
            HOST_SUFFIX="linux-x64"
        fi
        ;;
    Darwin*)
        HOST_OS="darwin"
        if [[ "$(uname -m)" == "arm64" ]]; then
            HOST_ARCH="arm64"
            HOST_RUST_TARGET="aarch64-apple-darwin"
            HOST_SUFFIX="darwin-arm64"
        else
            HOST_ARCH="x64"
            HOST_RUST_TARGET="x86_64-apple-darwin"
            HOST_SUFFIX="darwin-x64"
        fi
        ;;
    CYGWIN*|MINGW32*|MSYS*|MINGW*)
        HOST_OS="windows"
        # Note: This is simplified - Windows architecture detection is more complex
        HOST_ARCH="x64"
        HOST_RUST_TARGET="x86_64-pc-windows-msvc"
        HOST_SUFFIX="win32-x64"
        ;;
    *)
        echo "Unknown OS: $(uname -s)"
        exit 1
        ;;
esac

echo "Detected: $HOST_OS $HOST_ARCH"

# Build for host platform
build_platform "Host" "$HOST_OS" "$HOST_ARCH" "$HOST_RUST_TARGET" "$HOST_SUFFIX"

echo ""
echo "=== Build completed ==="
echo "VSIX files are in: $BUILD_DIR/"
ls -la "$BUILD_DIR"/*/*.vsix 2>/dev/null || echo "No VSIX files found (check build logs for errors)"

echo ""
echo "For full cross-platform builds, use GitHub Actions workflow."