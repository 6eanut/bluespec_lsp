---
name: Cleanup Generated Files
description: Remove generated build artifacts from git tracking to reduce repository size and improve codebase clarity
type: project
---

# 清理生成文件设计方案

## 背景

当前项目包含大量生成文件，特别是：
- `tree-sitter-bsv/src/parser.c` (17MB) - 从`grammar.js`生成的C解析器
- `tree-sitter-bsv/src/grammar.json` (359KB) - 生成的语法JSON
- `tree-sitter-bsv/src/node-types.json` (95KB) - 生成的节点类型定义
- `bsv-language-server/src/tree_sitter_bsv.c` (17MB) - 可能是`parser.c`的副本
- `node_modules/` - Node.js依赖目录

这些文件占用了大量存储空间，使得Git仓库臃肿。根据cloc统计，C语言文件占项目总代码量的51.7%（494,900/956,112行），其中大部分是生成的parser.c文件。

## 设计目标

1. **减小Git仓库大小** - 从17MB+的生成文件中释放空间
2. **提高代码库清晰度** - 只跟踪源代码，不跟踪构建产物
3. **保持构建流程完整性** - 本地和GitHub Actions都能正常工作
4. **确保新贡献者能轻松构建** - 提供清晰的构建指南

## 当前状态分析

### 项目结构
```
项目结构：
├── bsv-language-server/
│   ├── src/
│   │   └── tree_sitter_bsv.c (17MB，生成文件，被build.rs使用)
│   ├── build.rs (编译C文件)
│   └── .gitignore (现有)
├── tree-sitter-bsv/
│   ├── src/
│   │   ├── parser.c (17MB，从grammar.js生成)
│   │   ├── grammar.json (359KB，生成)
│   │   ├── node-types.json (95KB，生成)
│   │   └── tree_sitter/ (头文件)
│   ├── grammar.js (31KB，源代码)
│   └── .gitignore (现有)
└── .github/workflows/release.yml (GitHub Actions)
```

### 构建流程
1. `grammar.js` → `parser.c` (通过tree-sitter generate)
2. `build.rs`编译`tree_sitter_bsv.c`为静态库
3. Rust代码链接该库进行解析

### 问题
- 重复的C文件：`bsv-language-server/src/tree_sitter_bsv.c`和`tree-sitter-bsv/src/parser.c`内容几乎相同
- 生成文件被跟踪在git中，增加仓库大小
- 现有`.gitignore`文件不完整，缺少关键生成文件

## 设计方案

### 阶段1：创建统一的.gitignore文件
在项目根目录创建统一的`.gitignore`文件，包含所有构建产物。

**文件内容**：
```gitignore
# 通用构建产物
target/
build/
dist/
*.o
*.obj
*.so
*.dylib
*.dll
*.a
*.lib

# Node.js
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*
.pnpm-store/

# Rust
Cargo.lock
**/target/

# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg
MANIFEST

# tree-sitter生成文件
tree-sitter-bsv/src/parser.c
tree-sitter-bsv/src/grammar.json
tree-sitter-bsv/src/node-types.json
bsv-language-server/src/tree_sitter_bsv.c

# IDE和编辑器文件
.vscode/
.idea/
*.swp
*.swo
*~
```

### 阶段2：从Git中移除生成文件
使用`git rm --cached`从Git跟踪中移除生成文件，但保留在磁盘上供构建使用。

**要移除的文件**：
1. `tree-sitter-bsv/src/parser.c`
2. `tree-sitter-bsv/src/grammar.json`
3. `tree-sitter-bsv/src/node-types.json`
4. `bsv-language-server/src/tree_sitter_bsv.c`

**命令**：
```bash
git rm --cached tree-sitter-bsv/src/parser.c
git rm --cached tree-sitter-bsv/src/grammar.json
git rm --cached tree-sitter-bsv/src/node-types.json
git rm --cached bsv-language-server/src/tree_sitter_bsv.c
```

### 阶段3：更新构建流程
确保构建系统能重新生成必要的文件。

#### 3.1 更新tree-sitter-bsv/package.json
添加`tree-sitter-cli`为开发依赖：
```json
{
  "devDependencies": {
    "tree-sitter-cli": "^0.20.0"
  },
  "scripts": {
    "generate": "tree-sitter generate"
  }
}
```

#### 3.2 验证tree-sitter-bsv/Makefile
检查Makefile中的生成规则：
```makefile
$(PARSER): $(SRC_DIR)/grammar.json
    $(TS) generate $^
```

需要确保`grammar.json`存在或能从`grammar.js`生成。

#### 3.3 更新bsv-language-server/build.rs
修改构建脚本，在构建前确保C文件存在：
```rust
fn main() {
    // 确保tree_sitter_bsv.c文件存在
    let source_c = "src/tree_sitter_bsv.c";
    if !std::path::Path::new(source_c).exists() {
        // 尝试从tree-sitter-bsv目录复制
        let ts_parser = "../tree-sitter-bsv/src/parser.c";
        if std::path::Path::new(ts_parser).exists() {
            std::fs::copy(ts_parser, source_c).expect("Failed to copy parser.c");
        } else {
            panic!("tree_sitter_bsv.c not found and cannot be generated");
        }
    }
    
    // 编译tree-sitter-bsv
    cc::Build::new()
        .include("src")
        .file(source_c)
        .compile("tree-sitter-bsv");
    
    // 重新编译的触发条件
    println!("cargo:rerun-if-changed={}", source_c);
    println!("cargo:rerun-if-changed=src/tree_sitter_bsv.h");
}
```

### 阶段4：更新GitHub Actions工作流
确保CI/CD流程能正确构建。

#### 4.1 安装tree-sitter-cli
在`.github/workflows/release.yml`的依赖安装步骤中添加：
```yaml
- name: Install tree-sitter-cli
  run: npm install -g tree-sitter-cli
```

#### 4.2 生成parser.c文件
在构建步骤前添加：
```yaml
- name: Generate parser.c
  run: |
    cd tree-sitter-bsv
    tree-sitter generate || npm run generate
```

#### 4.3 复制C文件到bsv-language-server
```yaml
- name: Copy parser.c for language server
  run: |
    cp tree-sitter-bsv/src/parser.c bsv-language-server/src/tree_sitter_bsv.c
```

### 阶段5：验证和测试
#### 5.1 本地构建测试
1. 清理后运行`cargo build`
2. 运行`npm run compile`（TypeScript客户端）
3. 测试语言服务器功能

#### 5.2 CI/CD测试
1. 创建测试PR验证GitHub Actions
2. 确保Windows环境构建成功

#### 5.3 新环境测试
1. 在新目录克隆并构建
2. 验证生成文件能正确创建

## 关键决策点

### 1. 如何处理bsv-language-server/src/tree_sitter_bsv.c？
**选择方案A：从tree-sitter-bsv复制**
- 修改`build.rs`在构建时复制文件
- 优点：保持现有构建流程，风险低
- 缺点：需要文件复制逻辑

### 2. 如何确保tree-sitter CLI可用？
**方案**：
- 在`package.json`中添加`tree-sitter-cli`为devDependency
- 在构建脚本中检查tree-sitter CLI是否安装
- 提供安装指南给贡献者

### 3. 如何处理现有提交中的大文件？
**建议**：先处理未来提交，历史清理作为可选后续步骤。可以使用`git filter-repo`进行历史清理（高级操作）。

## 风险评估和缓解措施

| 风险 | 影响 | 缓解措施 |
|------|------|----------|
| 构建失败 | 高 | 分阶段实施，每步验证构建 |
| GitHub Actions失败 | 高 | 本地测试后更新工作流 |
| 新贡献者构建困难 | 中 | 更新README.md，提供详细构建指南 |
| 文件生成不一致 | 中 | 使用固定版本的tree-sitter-cli |
| 历史提交中的大文件 | 低 | 可选后续清理，不影响功能 |

## 实施计划

### 第1步：准备阶段
1. 创建统一的`.gitignore`文件
2. 更新README.md添加构建指南
3. 备份当前状态

### 第2步：文件清理
1. 从Git中移除生成文件
2. 提交`.gitignore`和文件移除

### 第3步：构建系统更新
1. 更新package.json添加tree-sitter-cli依赖
2. 更新build.rs添加文件复制逻辑
3. 验证本地构建

### 第4步：CI/CD更新
1. 更新GitHub Actions工作流
2. 创建测试PR验证CI

### 第5步：文档和测试
1. 更新构建文档
2. 测试新环境构建
3. 验证所有功能正常

## 后续优化（可选）

1. **统一构建系统**：使用单个Makefile或构建脚本管理所有生成步骤
2. **依赖管理优化**：使用git submodule或crate依赖替代文件复制
3. **文档完善**：详细构建指南和贡献者指南
4. **自动化检查**：在CI中添加生成文件检查，确保不意外提交生成文件

## 成功标准

1. Git仓库大小显著减小（从17MB+生成文件中释放）
2. 本地构建和GitHub Actions构建都能正常工作
3. 新贡献者能按照README.md指南成功构建项目
4. 所有现有功能保持正常