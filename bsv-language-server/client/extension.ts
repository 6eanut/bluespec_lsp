import * as vscode from 'vscode';
import * as path from 'path';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';

let client: LanguageClient;

// Platform detection utilities
interface PlatformInfo {
    os: 'win32' | 'darwin' | 'linux' | 'other';
    arch: 'arm64' | 'x64' | 'x86' | 'other';
}

function getPlatform(): PlatformInfo {
    const nodePlatform = process.platform;
    const nodeArch = process.arch;

    let os: PlatformInfo['os'];
    switch (nodePlatform) {
        case 'win32':
            os = 'win32';
            break;
        case 'darwin':
            os = 'darwin';
            break;
        case 'linux':
            os = 'linux';
            break;
        default:
            os = 'other';
    }

    let arch: PlatformInfo['arch'];
    switch (nodeArch) {
        case 'arm64':
            arch = 'arm64';
            break;
        case 'x64':
            arch = 'x64';
            break;
        case 'ia32':
            arch = 'x86';
            break;
        default:
            arch = 'other';
    }

    return { os, arch };
}

function getPlatformServerDirectory(platform: PlatformInfo): string {
    // Map platform to VS Code extension platform directory names
    // https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions
    if (platform.os === 'win32' && platform.arch === 'x64') {
        return 'win32-x64';
    } else if (platform.os === 'darwin' && platform.arch === 'arm64') {
        return 'darwin-arm64';
    } else if (platform.os === 'linux' && platform.arch === 'x64') {
        return 'linux-x64';
    } else if (platform.os === 'darwin' && platform.arch === 'x64') {
        // macOS x64 is also 'darwin-x64' in VS Code
        return 'darwin-x64';
    } else if (platform.os === 'win32' && platform.arch === 'x86') {
        return 'win32-ia32';
    } else if (platform.os === 'linux' && platform.arch === 'arm64') {
        return 'linux-arm64';
    } else {
        // Fallback to generic directory
        return ''; // Will fall back to legacy path
    }
}

export function activate(context: vscode.ExtensionContext) {
    console.log('BSV Language Server extension is now active!');

    // Log platform information for debugging
    const platform = getPlatform();
    console.log(`Detected platform: OS=${platform.os}, Arch=${platform.arch}`);
    console.log(`Platform server directory: ${getPlatformServerDirectory(platform)}`);
    
    // 获取配置
    const config = vscode.workspace.getConfiguration('bsv');
    const serverPath = config.get<string>('languageServer.path');
    const traceServer = config.get<string>('languageServer.trace.server') || 'off';
    const enable = config.get<boolean>('languageServer.enable', true);
    
    if (!enable) {
        console.log('BSV language server is disabled by configuration.');
        return;
    }
    
    // 确定服务器路径
    let serverModule: string;
    const fs = require('fs');

    // Determine platform-specific paths
    const platform = getPlatform();
    const serverExecutableName = platform.os === 'win32' ? 'bsv-language-server.exe' : 'bsv-language-server';
    const platformServerDir = getPlatformServerDirectory(platform);

    const defaultPaths = [
        // Platform-specific binary in server/{platform} directory (if platform is supported)
        ...(platformServerDir ? [context.asAbsolutePath(path.join('server', platformServerDir, serverExecutableName))] : []),
        // Legacy path for backward compatibility (direct in server directory)
        context.asAbsolutePath(path.join('server', serverExecutableName)),
        // Development paths
        context.asAbsolutePath(path.join('..', 'bsv-language-server', 'target', 'release', serverExecutableName)),
        context.asAbsolutePath(path.join('..', 'target', 'release', serverExecutableName)),
    ];

    if (serverPath && serverPath.trim() !== '') {
        // 使用用户指定的路径
        serverModule = serverPath;
    } else {
        // 使用默认路径列表，优先使用打包的server目录中的可执行文件
        const foundPath = defaultPaths.find((p: string) => fs.existsSync(p));
        serverModule = foundPath || defaultPaths[0];
    }
    
    console.log(`Using server module: ${serverModule}`);
    
    // 如果服务器模块不存在，尝试从系统PATH查找
    if (!fs.existsSync(serverModule)) {
        console.warn(`BSV language server executable not found at ${serverModule}, falling back to PATH lookup.`);
        serverModule = 'bsv-language-server';
    }
    
    // 服务器选项
    const serverOptions: ServerOptions = {
        run: {
            command: serverModule,
            args: [],
            transport: TransportKind.stdio
        },
        debug: {
            command: serverModule,
            args: ['--debug'],
            transport: TransportKind.stdio
        }
    };
    
    // 客户端选项
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'bsv' },
            { scheme: 'untitled', language: 'bsv' }
        ],
        synchronize: {
            // 同步配置更改
            configurationSection: 'bsv',
            // 通知服务器文件更改
            fileEvents: [
                vscode.workspace.createFileSystemWatcher('**/*.bsv'),
                vscode.workspace.createFileSystemWatcher('**/*.bs')
            ]
        },
        outputChannel: vscode.window.createOutputChannel('BSV Language Server'),
        traceOutputChannel: vscode.window.createOutputChannel('BSV Language Server Trace'),
        initializationOptions: {
            // 传递给服务器的初始化选项
            workspaceFolders: vscode.workspace.workspaceFolders ? 
                vscode.workspace.workspaceFolders.map(folder => folder.uri.toString()) : []
        }
    };
    
    // 创建语言客户端
    client = new LanguageClient(
        'bsvLanguageServer',
        'BSV Language Server',
        serverOptions,
        clientOptions
    );
    
    // 设置跟踪级别
    client.setTrace(traceServer === 'verbose' ? 2 : traceServer === 'messages' ? 1 : 0);
    
    // 启动客户端
    client.start().then(() => {
        console.log('BSV Language Server client started successfully.');
        
        // 注册命令
        context.subscriptions.push(
            vscode.commands.registerCommand('bsv.restartServer', async () => {
                await client.stop();
                await client.start();
                vscode.window.showInformationMessage('BSV Language Server restarted.');
            }),
            
            vscode.commands.registerCommand('bsv.showOutput', () => {
                client.outputChannel.show();
            })
        );
    }).catch((err: any) => {
        vscode.window.showErrorMessage(`Failed to start BSV Language Server: ${err.message}`);
        console.error('Failed to start BSV Language Server:', err);
    });
    
    // 添加到订阅列表
    context.subscriptions.push(client);
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}