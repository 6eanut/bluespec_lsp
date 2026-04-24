# 构建说明

## 前提条件

1. **Node.js** (v20或更高版本)
2. **Rust** (稳定版)
3. **tree-sitter-cli** (用于生成解析器)

## 安装tree-sitter-cli

```bash
npm install -g tree-sitter-cli
```

或者作为开发依赖安装：
```bash
cd tree-sitter-bsv
npm install
```

## 生成解析器文件

在首次构建或更新语法后，需要生成解析器文件：

```bash
cd tree-sitter-bsv
tree-sitter generate
# 或者
npm run generate
```

这将生成：
- `src/parser.c` (C解析器)
- `src/grammar.json` (语法定义)
- `src/node-types.json` (节点类型)

## 构建语言服务器

```bash
cd bsv-language-server

# 安装Node.js依赖
npm install

# 编译TypeScript客户端
npm run compile

# 构建Rust服务器
cargo build --release
```

## 自动构建

构建脚本(`bsv-language-server/build.rs`)会自动处理：
1. 检查`src/tree_sitter_bsv.c`是否存在
2. 如果不存在，从`../tree-sitter-bsv/src/parser.c`复制
3. 如果复制失败，提示用户运行`tree-sitter generate`

## GitHub Actions

CI/CD流程会自动：
1. 安装tree-sitter-cli
2. 生成解析器文件
3. 复制到语言服务器目录
4. 构建Windows版本

## 故障排除

### 错误: "tree_sitter_bsv.c not found"
运行：`cd tree-sitter-bsv && tree-sitter generate`

### 错误: "tree-sitter command not found"
安装：`npm install -g tree-sitter-cli`

### 错误: 构建失败
确保所有依赖已安装，然后重试完整构建流程。

## 生成文件和Git

以下生成文件**不**应提交到Git：
- `tree-sitter-bsv/src/parser.c`
- `tree-sitter-bsv/src/grammar.json`
- `tree-sitter-bsv/src/node-types.json`
- `bsv-language-server/src/tree_sitter_bsv.c`
- `bsv-language-server/src/grammar.json`
- `bsv-language-server/src/node-types.json`

这些文件已添加到`.gitignore`中，会在构建时自动生成。