#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

// 这个脚本在 vsce package 之前运行，确保扩展包含正确的文件

const platform = process.argv[2];
if (!platform) {
    console.error('Usage: node prepare-extension.js <platform-suffix>');
    console.error('Example: node prepare-extension.js win32-x64');
    process.exit(1);
}

console.log(`Preparing extension for platform: ${platform}`);

const projectRoot = path.join(__dirname, '..');
const bsvLanguageServerDir = path.join(projectRoot, 'bsv-language-server');
const packageJsonPath = path.join(bsvLanguageServerDir, 'package.json');

// 读取 package.json
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

// 确定二进制文件扩展名
let binaryExt = '';
let libExt = '';

if (platform.startsWith('win32')) {
    binaryExt = '.exe';
    libExt = '.dll';
} else if (platform.startsWith('linux')) {
    binaryExt = '';
    libExt = '.so';
} else if (platform.startsWith('darwin')) {
    binaryExt = '';
    libExt = '.dylib';
}

const serverBinaryName = `bsv-language-server${binaryExt}`;
const libName = `libtree-sitter-bsv${libExt}`;

// 更新 package.json 以包含正确的文件
packageJson.files = [
    'client/out/**/*.js',
    'client/out/**/*.js.map',
    'syntaxes/*.json',
    'language-configuration.json',
    `dist/${platform}/${serverBinaryName}`,
    `dist/${platform}/${libName}`,
    'README.md',
    'LICENSE'
];

// 更新配置默认值
packageJson.contributes.configuration.properties['bsv.languageServer.path'].default = `./dist/${platform}/${serverBinaryName}`;

// 写回 package.json
fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
console.log(`Updated package.json for platform ${platform}`);

// 确保 dist 目录存在
const distPlatformDir = path.join(bsvLanguageServerDir, 'dist', platform);
if (!fs.existsSync(distPlatformDir)) {
    fs.mkdirSync(distPlatformDir, { recursive: true });
    console.log(`Created directory: ${distPlatformDir}`);
}

// 检查服务器文件是否存在（必需）
const requiredServerFile = path.join(distPlatformDir, serverBinaryName);
if (!fs.existsSync(requiredServerFile)) {
    console.error(`Missing required file: ${requiredServerFile}`);
    console.error('\n⚠ Server binary is missing. Build the binaries first.');
    console.error('Run: node scripts/build-vsix.js');
    process.exit(1);
}

// 检查 tree-sitter 库文件（可选，对于 Windows 可能不存在）
const libFile = path.join(distPlatformDir, libName);
if (!fs.existsSync(libFile)) {
    console.warn(`Warning: tree-sitter library not found: ${libFile}`);
    console.warn('This may affect some features but the extension will still work.');
}

console.log(`✓ Extension prepared for ${platform}`);
console.log(`  - Server binary: dist/${platform}/${serverBinaryName}`);
console.log(`  - Tree-sitter library: dist/${platform}/${libName}`);