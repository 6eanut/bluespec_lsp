#!/bin/bash

# BSV Language Server 发布构建脚本
# 用于在GitHub Actions中构建服务器并准备VSIX打包

set -e  # 遇到错误时退出

echo "=== BSV Language Server 发布构建脚本 ==="
echo "工作目录: $(pwd)"

# 检查是否在正确的目录
if [ ! -f "Cargo.toml" ]; then
    echo "错误: 需要在bsv-language-server目录中运行此脚本"
    exit 1
fi

# 目标平台（默认为当前平台）
TARGET="${1:-aarch64-apple-darwin}"
echo "目标平台: $TARGET"

# 安装Rust目标（如果尚未安装）
echo "安装Rust目标: $TARGET"
rustup target add "$TARGET"

# 清理之前的构建
echo "清理之前的构建..."
cargo clean

# 构建发布版本
echo "构建Rust服务器 (release模式)..."
cargo build --release --target "$TARGET"

# 验证构建结果
SERVER_BINARY="target/$TARGET/release/bsv-language-server"
if [ ! -f "$SERVER_BINARY" ]; then
    echo "错误: 服务器可执行文件未找到: $SERVER_BINARY"
    exit 1
fi

echo "服务器构建成功: $SERVER_BINARY"
file "$SERVER_BINARY"

# 创建server目录（如果不存在）
echo "准备server目录..."
mkdir -p server

# 复制服务器可执行文件到server目录
echo "复制服务器可执行文件到server/目录..."
cp "$SERVER_BINARY" server/
chmod +x server/bsv-language-server

# 验证复制结果
echo "验证server目录内容:"
ls -la server/

# 检查文件大小
echo "服务器可执行文件大小:"
ls -lh server/bsv-language-server

echo "=== 构建完成 ==="
echo "服务器可执行文件已复制到: server/bsv-language-server"
echo "现在可以运行 'vsce package' 来打包VSIX文件"