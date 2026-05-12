# BSV Language Server - Claude Code Guide

## Project Overview

This project provides a Language Server Protocol (LSP) implementation for
Bluespec SystemVerilog (BSV), consisting of:

- `tree-sitter-bsv/` - Tree-sitter grammar and parser for BSV
- `bsv-language-server/` - Rust-based LSP server with VS Code extension

## Key Files

| File | Purpose |
|------|---------|
| `bsv-language-server/src/server.rs` | LSP server (tower-lsp) |
| `bsv-language-server/src/parser.rs` | Tree-sitter parser with error recovery |
| `bsv-language-server/src/symbols.rs` | Concurrent symbol table |
| `bsv-language-server/src/constant_expansion/` | #define constant evaluation |
| `bsv-language-server/src/errors.rs` | Error types |
| `bsv-language-server/src/utils.rs` | Utilities |
| `bsv-language-server/client/extension.ts` | VS Code extension client |
| `tree-sitter-bsv/grammar.js` | Tree-sitter grammar definition |

## Development Commands

```bash
# Build the language server
cd bsv-language-server && cargo build --release

# Run all tests
cd bsv-language-server && cargo test -- --test-threads=1

# Run with verbose output
cd bsv-language-server && cargo test -- --nocapture

# Run clippy
cd bsv-language-server && cargo clippy -- -D warnings

# Run formatter
cd bsv-language-server && cargo fmt --check
```

## Guidelines

- All new features must include tests (unit tests + integration tests where applicable)
- Run `cargo test` and `cargo clippy` before committing
- Follow conventional commits format: `feat:`, `fix:`, `test:`, `docs:`, `refactor:`, `chore:`
- The project uses `#![allow(deprecated)]` - avoid adding to it; fix deprecations instead
- Maintain English comments in source code (Chinese comments exist in legacy code)