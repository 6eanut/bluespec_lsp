# Bluespec LSP Workspace

Note: LLM-led implementation.

This workspace contains two related projects that together provide a Bluespec SystemVerilog (BSV) language experience:

- `tree-sitter-bsv` — a Tree-sitter grammar and parser for BSV.
- `bsv-language-server` — a Rust-based Language Server implementation with a VS Code client extension.

Getting started (development)

1. Open this workspace folder (`bluespec_lsp`) in VS Code.
2. See each subproject for build instructions:
   - `tree-sitter-bsv`: run `tree-sitter generate` and `tree-sitter test` to validate the grammar.
   - `bsv-language-server`: build the Rust server (`cargo build --release`) and compile the client (`npm install && npm run compile`).
3. To iterate on the extension: open `bsv-language-server` in VS Code and use Run and Debug → `Launch Extension` to start an Extension Development Host. Open a `.bsv` file there to test features (hover, completion, document symbols, go-to-definition).

Where to look next
- `tree-sitter-bsv/src/` — grammar and parser sources.
- `bsv-language-server/src/` — server implementation (Rust).
- `bsv-language-server/client/` — VS Code extension client (TypeScript).

## Release Process

The project uses GitHub Actions to automatically build and release VSIX packages for multiple platforms when a version tag is pushed:

### Supported Platforms
- **Windows x86_64** (`bsv-language-server-<version>-windows-x64.vsix`)
- **macOS ARM64 (Apple Silicon)** (`bsv-language-server-<version>-darwin-arm64.vsix`)

### Creating a Release
1. Create and push a version tag (e.g., `v1.0.0`):
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. GitHub Actions will automatically:
   - Build the Rust server for both Windows and macOS ARM64
   - Compile the TypeScript client
   - Package platform-specific VSIX files
   - Create a GitHub Release with both VSIX files

3. Download the appropriate VSIX for your platform from the [Releases](https://github.com/open-rdma/bluespec-lsp/releases) page.

### Development Builds
For development, build locally:
```bash
cd bsv-language-server
# Build Rust server
cargo build --release
# Compile TypeScript client
npm install
npm run compile
# Package VSIX (requires vsce)
npm install -g @vscode/vsce
vsce package
```

Contributing
- Please open issues or PRs for grammar fixes, LSP features, or client improvements.
