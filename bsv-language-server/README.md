# BSV Language Server (bsv-language-server)

Note: LLM-led implementation.

Bluespec SystemVerilog (BSV) Language Server implemented in Rust with a VS Code client.

Repository layout (important parts):
- `src/` — Rust language-server implementation (uses `tower-lsp`).
- `client/` — VS Code extension client (TypeScript). Entry point: `client/extension.ts`.
- `target/` — build output for the Rust server (binary will appear in `target/release/`).
- `syntaxes/` — TextMate grammar files used by the extension.
- `package.json` — VS Code extension manifest and build scripts.

Prerequisites
- Rust + Cargo (for server)
- Node.js + npm (for building the VS Code client)
- VS Code (for extension development/debug)

Platform Support
- Windows x86_64
- macOS ARM64 (Apple Silicon)

Build and run (development)

1. Build the Rust language server (release recommended):

```bash
cd bsv-language-server

# For your current platform
cargo build --release

# For specific platforms
cargo build --release --target x86_64-pc-windows-msvc    # Windows x86_64
cargo build --release --target aarch64-apple-darwin      # macOS ARM64

# Binary will be at target/release/bsv-language-server (or target/<target>/release/bsv-language-server)
```

2. Install and compile the extension client:

```bash
# from repository root (bsv-language-server)
npm install
npm run compile   # compiles client/ TypeScript to client/out
```

3. Launch the extension in VS Code (Dev host):
- Open the `bsv-language-server` folder in VS Code.
- Open the Run and Debug view (left sidebar).
- Select `Launch Extension` (or press F5). VS Code will open a new Extension Development Host window.
- In the new window, open or create a `.bsv` file and try features: symbol outline, hover, completion, and `Go to Definition` (F12 or right-click → "Go to Definition").

Notes about where the extension finds the server
- The client tries to use a configured path `bsv.languageServer.path` (see `package.json` configuration). If empty, it falls back to platform-specific paths:
  - Windows: Looks for `bsv-language-server.exe` in `server/win32-x64/` directory
  - macOS: Looks for `bsv-language-server` in `server/darwin-arm64/` directory
  - Development: Falls back to `../target/release/bsv-language-server[.exe]` or `PATH`-installed binary

Useful npm scripts (from `package.json`):
- `npm run compile` — compile the TypeScript client (`client/out/extension.js`).
- `npm run watch` — continuous compile during client development.

Developing tips
- When iterating on the Rust server, rebuild `cargo build` and then restart the extension host (stop and relaunch from the debugger) or use the `bsv.restartServer` command from the Command Palette.
- Use the Output panel and select "BSV Language Server" to view server logs.

Configuration
- `bsv.languageServer.path` — explicit path to the server executable
- `bsv.languageServer.trace.server` — tracing level: `off`, `messages`, `verbose`
- `bsv.languageServer.enable` — enable/disable the language server

License and contributions
- See repository LICENSE files. Contributions welcome via PRs and issues.
