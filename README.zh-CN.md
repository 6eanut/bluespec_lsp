# Bluespec LSP 工作区

[![Version](https://img.shields.io/badge/version-0.1.0--pre--alpha-blue)](https://github.com/6eanut/bluespec_lsp)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](LICENSE)
[![CI](https://img.shields.io/badge/CI-passing-brightgreen)](https://github.com/6eanut/bluespec_lsp/actions)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange)](https://www.rust-lang.org/)

> 为 Bluespec SystemVerilog (BSV) 提供 IDE 级编辑支持的 LSP 语言服务器实现。

## 项目状态

**pre-alpha 原型阶段** — 核心 LSP 功能已实现并正在积极扩展中。

**当前已实现的功能：**

| 功能 | 状态 |
|------|------|
| 语法高亮 | ✅ 基础 |
| 文档符号 | ✅ 扁平列表 |
| 转到定义 | ✅ 同文件及跨文件 |
| 悬停信息 | ✅ 常量展开 + 符号信息 |
| 自动补全 | ✅ 基础关键字及本地符号 |
| 工作区符号 | ✅ 跨所有打开的文件 |
| 错误恢复 | ✅ 从格式错误的代码中提取符号 |
| `#define` 常量展开 | ✅ 支持类型函数（TAdd、TSub 等） |
| BVI / BDPI 导入语法 | ✅ 支持 |

## 工作区内容

该工作区包含两个相关子项目，共同为 Bluespec SystemVerilog (BSV) 提供语言支持：

- **`tree-sitter-bsv`** — BSV 的 Tree-sitter 语法与解析器。
- **`bsv-language-server`** — 基于 Rust 的语言服务器实现以及对应的 VS Code 客户端扩展。

## 快速上手（开发）

### 前置要求

- **Rust** 工具链（1.70 或更高）
- **Node.js**（20 或更高）
- **tree-sitter CLI**（语法开发需要）

### 安装步骤

```bash
# 安装 tree-sitter CLI（如未安装）
cargo install tree-sitter-cli

# 构建语言服务器
cd bsv-language-server
cargo build --release

# 安装并编译 VS Code 客户端
npm install
npm run compile
```

### 测试 LSP 功能

1. 在 VS Code 中打开 `bsv-language-server`
2. 进入 **Run and Debug** → **Launch Extension**
3. 在新的 Extension Development Host 窗口中打开 `.bsv` 文件
4. 尝试：悬停、自动补全（`Ctrl+Space`）、转到定义（`F12`）、文档符号（`Ctrl+Shift+O`）

## 功能路线图

### 阶段 1 — 基础设施 ✅

| 功能 | 状态 |
|------|------|
| Tree-sitter BSV 语法 | ✅ |
| Rust LSP 服务器框架 | ✅ |
| VS Code 扩展客户端 | ✅ |
| 社区相关文件 | ✅ |

### 阶段 2 — 代码质量（进行中）

| 功能 | 状态 |
|------|------|
| 死代码清理 | ⏳ |
| 未使用依赖清理 | ⏳ |
| 弃用修复 | ⏳ |
| 常量展开器加固 + 测试 | ⏳ |
| 十六进制常量支持 + 测试 | ⏳ |
| Clippy 清理 | ⏳ |

### 阶段 3 — CI/CD

| 功能 | 状态 |
|------|------|
| GitHub Actions CI | ❌ |
| 多平台发布工作流 | ❌ |
| Dependabot 依赖更新 | ❌ |

### 阶段 4 — 核心 LSP 功能

| 功能 | 状态 |
|------|------|
| 诊断（语法错误报告） | ❌ |
| 引用查找 | ❌ |
| 分层文档符号 | ❌ |
| 改进的自动补全 | ❌ |

### 阶段 5 — 进阶 LSP 功能

| 功能 | 状态 |
|------|------|
| 语义令牌 | ❌ |
| 代码折叠 | ❌ |
| 代码操作 | ❌ |
| 签名帮助 | ❌ |
| 改进 TextMate 语法 | ❌ |
| 增量同步 | ❌ |

## 文档

- [README.md](README.md) — English documentation
- [CONTRIBUTING.md](CONTRIBUTING.md) — 贡献指南
- [CHANGELOG.md](CHANGELOG.md) — 版本历史
- [SECURITY.md](SECURITY.md) — 安全策略
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — 行为准则

## 许可证

本项目基于 Apache 2.0 许可证 — 详见 [LICENSE](LICENSE) 文件。
