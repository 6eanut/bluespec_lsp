# Bluespec LSP Workspace

Note: LLM-led implementation.

This workspace contains two related projects that together provide a Bluespec SystemVerilog (BSV) language experience:

- `tree-sitter-bsv` — a Tree-sitter grammar and parser for BSV.
- `bsv-language-server` — a Rust-based Language Server implementation with a VS Code client extension.

## Platform Support

The BSV Language Server extension is available for the following platforms:

| Platform | Architecture | Download |
|----------|--------------|----------|
| Windows | x86_64 | `bsv-language-server-windows-x64.vsix` |
| macOS | ARM64 (Apple Silicon) | `bsv-language-server-macos-arm64.vsix` |

### Installation

1. Download the appropriate VSIX file for your platform from the [Releases](https://github.com/6eanut/bluespec-lsp/releases) page.
2. Open VS Code.
3. Press `Ctrl+Shift+X` (Windows/Linux) or `Cmd+Shift+X` (macOS) to open the Extensions view.
4. Click the "..." menu in the top-right corner and select "Install from VSIX...".
5. Select the downloaded `.vsix` file.
6. Reload VS Code when prompted.

## Getting started (development)

1. Open this workspace folder (`bluespec_lsp`) in VS Code.
2. See each subproject for build instructions:
   - `tree-sitter-bsv`: run `tree-sitter generate` and `tree-sitter test` to validate the grammar.
   - `bsv-language-server`: build the Rust server (`cargo build --release`) and compile the client (`npm install && npm run compile`).
3. To iterate on the extension: open `bsv-language-server` in VS Code and use Run and Debug → `Launch Extension` to start an Extension Development Host. Open a `.bsv` file there to test features (hover, completion, document symbols, go-to-definition).

### Building from Source

To build for a specific platform:

```bash
# Windows x86_64
cargo build --release --target x86_64-pc-windows-msvc

# macOS ARM64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin
```

## Where to look next
- `tree-sitter-bsv/src/` — grammar and parser sources.
- `bsv-language-server/src/` — server implementation (Rust).
- `bsv-language-server/client/` — VS Code extension client (TypeScript).

## Contributing
- Please open issues or PRs for grammar fixes, LSP features, or client improvements.
- See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.
