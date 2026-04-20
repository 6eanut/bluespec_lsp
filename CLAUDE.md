# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This workspace contains a Bluespec SystemVerilog (BSV) Language Server implementation with two main components:

1. **tree-sitter-bsv** - Tree-sitter grammar and parser for BSV
2. **bsv-language-server** - Rust-based LSP server with VS Code client extension

## Development Commands

### Building the Parser (tree-sitter-bsv)
```bash
cd tree-sitter-bsv
tree-sitter generate    # Generate C parser from grammar.json
tree-sitter test        # Run grammar tests
```

### Building the Language Server
```bash
cd bsv-language-server

# Build Rust server (development)
cargo build

# Build Rust server (release)
cargo build --release
# Output: target/release/bsv-language-server (or .exe on Windows)

# Build TypeScript client
npm install
npm run compile         # Compiles to client/out/
```

### Testing
```bash
cd bsv-language-server

# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_extract_module_with_broken_endmodule

# Use test script
./scripts/run_tests.sh
./scripts/run_tests.sh --verbose
./scripts/run_tests.sh --release
```

### Development Workflow
1. Open `bsv-language-server` in VS Code
2. Use **Run and Debug** → **Launch Extension** to start Extension Development Host
3. Open `.bsv` files in the development host to test LSP features
4. Use `bsv.restartServer` command to restart language server without restarting VS Code

### Release Builds (GitHub Actions)
- Triggered by `v*` tags (e.g., `v1.0.0`)
- Builds multi-platform VSIX extension with:
  - Windows x86_64 (`win32-x64/bsv-language-server.exe`)
  - macOS arm64 (`darwin-arm64/bsv-language-server`)
  - Linux x86_64 (`linux-x64/bsv-language-server`)
- Creates GitHub Release with VSIX file

## Architecture

### Key Components
- **src/parser.rs** - Tree-sitter parser with error recovery (extracts symbols from malformed BSV)
- **src/symbols.rs** - Symbol table management
- **src/constant_expansion/** - Constant evaluation system for `#define` macros
- **src/server.rs** - Main LSP server implementation (tower-lsp)
- **client/extension.ts** - VS Code client extension

### Key Features
1. **Error Recovery**: Extracts symbols from syntax errors by traversing ERROR nodes
2. **Constant Expansion**: Evaluates `#define` constants with nested type functions (TAdd, TSub, TMul, etc.)
3. **LSP Features**: Document symbols, go-to-definition, hover, completion, workspace symbols
4. **Multi-platform**: Single VSIX with platform-specific server binaries

### Build System
- Rust server includes C parser via `build.rs` (`src/tree_sitter_bsv.c`)
- VS Code extension uses `vscode-languageclient` for LSP communication
- Multi-platform builds via GitHub Actions matrix strategy

## File Structure

```
bsv-language-server/
├── src/
│   ├── server.rs          # LSP server implementation
│   ├── parser.rs          # Tree-sitter parser with error recovery
│   ├── symbols.rs         # Symbol table management
│   ├── constant_expansion/# Constant evaluation
│   ├── lib.rs             # Library exports
│   ├── main.rs            # Server entry point
│   └── tree_sitter_bsv.c  # Generated C parser
├── client/
│   ├── extension.ts       # VS Code client
│   └── out/               # Compiled JavaScript
├── test_fixtures/         # Test BSV samples
├── server/                # Platform-specific server binaries (release)
└── scripts/               # Build and test scripts

tree-sitter-bsv/
├── src/grammar.json       # Tree-sitter grammar definition
├── src/node-types.json    # Node type definitions
├── src/parser.c           # Generated parser
└── test/                  # Grammar test files
```

## Testing
- Test fixtures: `test_fixtures/correct.bsv`, `test_fixtures/broken.bsv`, `test_fixtures/constants.bsv`
- Tests cover: correct code parsing, error recovery, constant expansion, performance
- Run tests before submitting changes

## Important Notes
- The extension looks for server binaries in platform-specific directories (`server/win32-x64/`, `server/darwin-arm64/`, `server/linux-x64/`)
- Backward compatibility: also checks legacy `server/` directory
- Client automatically selects correct binary based on platform and architecture
- Release builds include all platform binaries in a single VSIX file