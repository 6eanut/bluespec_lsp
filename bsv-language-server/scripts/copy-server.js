const fs = require('fs');
const path = require('path');

const platform = process.platform;
const arch = process.arch;
const valid = new Set(['linux', 'darwin', 'win32']);
const archMap = new Map([
  ['x64', 'x64'],
  ['arm64', 'arm64']
]);

if (!valid.has(platform) || !archMap.has(arch)) {
  console.error(`Unsupported platform/arch: ${platform}/${arch}`);
  process.exit(1);
}

const platformArch = `${platform}-${archMap.get(arch)}`;
const serverName = platform === 'win32' ? 'bsv-language-server.exe' : 'bsv-language-server';
const releaseDir = path.join(__dirname, '..', 'target', 'release');
const sourcePath = path.join(releaseDir, serverName);
const platformDir = path.join(__dirname, '..', 'server', platformArch);
const legacyDir = path.join(__dirname, '..', 'server');
const platformPath = path.join(platformDir, serverName);
const legacyPath = path.join(legacyDir, serverName);

if (!fs.existsSync(sourcePath)) {
  console.error(`Server binary not found at ${sourcePath}. Please build with cargo build --release first.`);
  process.exit(1);
}

fs.mkdirSync(platformDir, { recursive: true });
fs.mkdirSync(legacyDir, { recursive: true });
fs.copyFileSync(sourcePath, platformPath);
fs.copyFileSync(sourcePath, legacyPath);
fs.chmodSync(platformPath, 0o755);
if (platform !== 'win32') {
  fs.chmodSync(legacyPath, 0o755);
}

console.log(`Copied server binary to:
  ${platformPath}
  ${legacyPath}`);
const fs = require('fs');
const path = require('path');

const platform = process.platform;
const arch = process.arch;
const valid = new Set(['linux', 'darwin', 'win32']);
const archMap = new Map([
  ['x64', 'x64'],
  ['arm64', 'arm64']
]);

if (!valid.has(platform) || !archMap.has(arch)) {
  console.error(`Unsupported platform/arch: ${platform}/${arch}`);
  process.exit(1);
}

const platformArch = `${platform}-${archMap.get(arch)}`;
const serverName = platform === 'win32' ? 'bsv-language-server.exe' : 'bsv-language-server';
const releaseDir = path.join(__dirname, '..', 'target', 'release');
const sourcePath = path.join(releaseDir, serverName);
const platformDir = path.join(__dirname, '..', 'server', platformArch);
const legacyDir = path.join(__dirname, '..', 'server');
const platformPath = path.join(platformDir, serverName);
const legacyPath = path.join(legacyDir, serverName);

if (!fs.existsSync(sourcePath)) {
  console.error(`Server binary not found at ${sourcePath}. Please build with cargo build --release first.`);
  process.exit(1);
}

fs.mkdirSync(platformDir, { recursive: true });
fs.mkdirSync(legacyDir, { recursive: true });
fs.copyFileSync(sourcePath, platformPath);
fs.copyFileSync(sourcePath, legacyPath);
fs.chmodSync(platformPath, 0o755);
if (platform !== 'win32') {
  fs.chmodSync(legacyPath, 0o755);
}

console.log(`Copied server binary to:
  ${platformPath}
  ${legacyPath}`);
