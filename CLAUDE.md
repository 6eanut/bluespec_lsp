# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This repository contains two related projects for Bluespec SystemVerilog (BSV) language support:

1. **tree-sitter-bsv**: Tree-sitter grammar and parser for BSV
2. **bsv-language-server**: Rust-based Language Server Protocol (LSP) implementation with VS Code client extension

## Development Commands

### Building tree-sitter-bsv
```bash
cd tree-sitter-bsv
tree-sitter generate    # Generate C parser files from grammar.json
tree-sitter test        # Run grammar tests using files in test/
```

### Building bsv-language-server
```bash
cd bsv-language-server
cargo build --release   # Build Rust language server
npm install             # Install Node.js dependencies
npm run compile         # Compile TypeScript client to client/out/
```

### Running Tests
```bash
cd bsv-language-server
cargo test              # Run all Rust tests
cargo test -- --nocapture # Run with verbose output
./scripts/run_tests.sh  # Use test runner script
./scripts/run_tests.sh --verbose
./scripts/run_tests.sh --release
```

### Development Workflow
1. Open `bsv-language-server` folder in VS Code
2. Use Run and Debug → `Launch Extension` to start an Extension Development Host
3. Open `.bsv` files in the development host to test LSP features
4. Use `bsv.restartServer` command from Command Palette to restart language server without restarting VS Code

### Release Process
The GitHub Actions workflow (`/.github/workflows/release.yml`) builds VSIX packages for multiple platforms:
- Windows x86_64 (`win32-x64`)
- macOS arm64 (`darwin-arm64`)
- Linux x86_64 (`linux-x64`)

To trigger a release:
```bash
git tag v0.0.2
git push origin v0.0.2
```

## Architecture

### Language Server Structure
```
bsv-language-server/
├── src/
│   ├── server.rs          # Main LSP server implementation
│   ├── parser.rs          # Tree-sitter parser with error recovery
│   ├── symbols.rs         # Symbol table management
│   ├── constant_expansion/ # Constant evaluation system
│   └── lib.rs             # Main library exports
├── client/
│   └── extension.ts       # VS Code extension entry point (TypeScript)
├── test_fixtures/         # Test BSV code samples
└── scripts/               # Build and test scripts
```

### Key Components

1. **Parser**: Uses `tree-sitter-bsv` grammar with error recovery to extract symbols from malformed BSV code
2. **Symbol Table**: Concurrent symbol storage using `dashmap` for workspace-wide symbol lookup
3. **Constant Expansion**: Evaluates `#define` constants with nested type functions (TAdd, TSub, TMul, etc.)
4. **Client**: VS Code extension that launches the Rust server binary and provides LSP features

### Platform Support
The extension supports multiple platforms by:
1. Building Rust server binaries for each target (`x86_64-pc-windows-msvc`, `aarch64-apple-darwin`, `x86_64-unknown-linux-gnu`)
2. Packaging binaries in platform-specific directories (`win32-x64`, `darwin-arm64`, `linux-x64`) within the `server/` folder
3. VS Code extension automatically selects the correct binary based on the platform

### Error Recovery System
The parser implements error-tolerant symbol extraction by:
- Traversing ERROR nodes in parse trees
- Identifying recognizable patterns despite syntax errors
- Extracting module definitions, functions, and variables even with broken syntax

## Testing
Test fixtures are in `bsv-language-server/test_fixtures/`:
- `correct.bsv` - Syntactically correct BSV code
- `broken.bsv` - BSV code with intentional syntax errors
- `constants.bsv` - `#define` constant definitions for testing