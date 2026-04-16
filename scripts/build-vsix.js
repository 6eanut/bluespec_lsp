#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// 平台配置
const platforms = [
    { os: 'windows', arch: 'x64', rustTarget: 'x86_64-pc-windows-msvc', suffix: 'win32-x64', binaryExt: '.exe', libExt: '.dll' },
    { os: 'windows', arch: 'arm64', rustTarget: 'aarch64-pc-windows-msvc', suffix: 'win32-arm64', binaryExt: '.exe', libExt: '.dll' },
    { os: 'linux', arch: 'x64', rustTarget: 'x86_64-unknown-linux-gnu', suffix: 'linux-x64', binaryExt: '', libExt: '.so' },
    { os: 'linux', arch: 'arm64', rustTarget: 'aarch64-unknown-linux-gnu', suffix: 'linux-arm64', binaryExt: '', libExt: '.so' },
    { os: 'darwin', arch: 'x64', rustTarget: 'x86_64-apple-darwin', suffix: 'darwin-x64', binaryExt: '', libExt: '.dylib' },
    { os: 'darwin', arch: 'arm64', rustTarget: 'aarch64-apple-darwin', suffix: 'darwin-arm64', binaryExt: '', libExt: '.dylib' }
];

// 项目根目录
const projectRoot = path.join(__dirname, '..');
const bsvLanguageServerDir = path.join(projectRoot, 'bsv-language-server');
const treeSitterBsvDir = path.join(projectRoot, 'tree-sitter-bsv');
const distDir = path.join(projectRoot, 'dist');

function runCommand(cmd, cwd = projectRoot) {
    console.log(`Running: ${cmd}`);
    try {
        execSync(cmd, { cwd, stdio: 'inherit' });
    } catch (error) {
        console.error(`Command failed: ${cmd}`);
        process.exit(1);
    }
}

function ensureDir(dir) {
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

function copyFile(src, dst) {
    if (fs.existsSync(src)) {
        fs.copyFileSync(src, dst);
        console.log(`Copied: ${src} -> ${dst}`);
    } else {
        console.warn(`Source file not found: ${src}`);
    }
}

function buildForPlatform(platform) {
    console.log(`\n=== Building for ${platform.suffix} ===`);

    // 1. 构建 tree-sitter-bsv
    console.log('Building tree-sitter-bsv...');
    runCommand('tree-sitter generate', treeSitterBsvDir);
    runCommand('make', treeSitterBsvDir);

    // 2. 构建 Rust 服务器
    console.log('Building Rust language server...');
    runCommand(`cargo build --release --target ${platform.rustTarget}`, bsvLanguageServerDir);

    // 3. 创建平台特定的扩展目录
    const platformExtDir = path.join(bsvLanguageServerDir, 'dist', platform.suffix);
    ensureDir(platformExtDir);

    // 4. 复制服务器二进制文件
    const serverBinaryName = `bsv-language-server${platform.binaryExt}`;
    const serverSrc = path.join(bsvLanguageServerDir, 'target', platform.rustTarget, 'release', serverBinaryName);
    const serverDst = path.join(platformExtDir, serverBinaryName);
    copyFile(serverSrc, serverDst);

    // 5. 复制 tree-sitter 库
    const libName = `libtree-sitter-bsv${platform.libExt}`;
    const libSrc = path.join(treeSitterBsvDir, libName);
    const libDst = path.join(platformExtDir, libName);
    copyFile(libSrc, libDst);

    // 6. 更新 package.json 以包含服务器路径
    const packageJsonPath = path.join(bsvLanguageServerDir, 'package.json');
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

    // 添加发布配置
    packageJson.scripts = packageJson.scripts || {};
    packageJson.scripts[`vscode:prepublish-${platform.suffix}`] = `node ../../scripts/prepare-extension.js ${platform.suffix}`;

    // 添加发布目标
    packageJson.vsce = packageJson.vsce || {};
    packageJson.vsce.targets = packageJson.vsce.targets || [];
    packageJson.vsce.targets.push(platform.suffix);

    fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));

    // 7. 构建 TypeScript 客户端
    console.log('Building TypeScript client...');
    runCommand('npm ci', bsvLanguageServerDir);
    runCommand('npm run compile', bsvLanguageServerDir);

    // 8. 打包 VSIX
    console.log('Packaging VSIX...');
    const vsixName = `bsv-language-server-${platform.suffix}.vsix`;
    const vsixPath = path.join(distDir, platform.suffix, vsixName);
    ensureDir(path.dirname(vsixPath));

    // 临时修改 package.json 以包含正确的二进制路径
    const tempPackageJson = { ...packageJson };
    tempPackageJson.contributes.configuration.properties['bsv.languageServer.path'].default = `./dist/${platform.suffix}/${serverBinaryName}`;

    const tempPackageJsonPath = path.join(bsvLanguageServerDir, 'package.temp.json');
    fs.writeFileSync(tempPackageJsonPath, JSON.stringify(tempPackageJson, null, 2));

    // 备份原始 package.json
    fs.copyFileSync(packageJsonPath, packageJsonPath + '.backup');
    fs.copyFileSync(tempPackageJsonPath, packageJsonPath);

    try {
        // 安装 vsce 如果不存在
        try {
            execSync('vsce --version', { stdio: 'ignore' });
        } catch {
            console.log('Installing vsce...');
            runCommand('npm install -g @vscode/vsce');
        }

        // 打包 VSIX
        runCommand(`vsce package --out ${vsixPath}`, bsvLanguageServerDir);
        console.log(`✓ VSIX created: ${vsixPath}`);
    } finally {
        // 恢复原始 package.json
        fs.copyFileSync(packageJsonPath + '.backup', packageJsonPath);
        fs.unlinkSync(packageJsonPath + '.backup');
        fs.unlinkSync(tempPackageJsonPath);
    }

    return vsixPath;
}

function main() {
    console.log('=== BSV Language Server VSIX Builder ===');

    // 检查依赖
    console.log('Checking dependencies...');
    try {
        execSync('tree-sitter --version', { stdio: 'ignore' });
        console.log('✓ tree-sitter found');
    } catch {
        console.error('✗ tree-sitter not found. Install with: npm install -g tree-sitter-cli');
        process.exit(1);
    }

    try {
        execSync('cargo --version', { stdio: 'ignore' });
        console.log('✓ cargo found');
    } catch {
        console.error('✗ cargo not found. Install Rust from https://rustup.rs/');
        process.exit(1);
    }

    try {
        execSync('npm --version', { stdio: 'ignore' });
        console.log('✓ npm found');
    } catch {
        console.error('✗ npm not found. Install Node.js from https://nodejs.org/');
        process.exit(1);
    }

    // 清理 dist 目录
    if (fs.existsSync(distDir)) {
        fs.rmSync(distDir, { recursive: true });
    }
    ensureDir(distDir);

    // 构建所有平台
    const builtVsix = [];

    for (const platform of platforms) {
        // 检查是否支持当前主机平台（简化版，实际需要交叉编译工具链）
        if (process.platform !== platform.os && platform.os !== 'windows') {
            console.log(`\nSkipping ${platform.suffix} (requires cross-compilation)`);
            continue;
        }

        const vsixPath = buildForPlatform(platform);
        builtVsix.push({ platform: platform.suffix, path: vsixPath });
    }

    // 生成构建报告
    console.log('\n=== Build Summary ===');
    console.log('Built VSIX files:');
    builtVsix.forEach(vsix => {
        console.log(`  ${vsix.platform}: ${vsix.path}`);
    });

    if (builtVsix.length === 0) {
        console.log('\n⚠ No VSIX files were built. Check platform compatibility.');
    } else {
        console.log(`\n✓ Successfully built ${builtVsix.length} VSIX file(s)`);
    }
}

if (require.main === module) {
    main();
}

module.exports = { buildForPlatform, platforms };