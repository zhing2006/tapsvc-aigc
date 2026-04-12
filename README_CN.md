[English](README.md)

# tapsvc-aigc

AIGC 内容生成 Agent 技能。通过 `tapsvc-aigc` CLI 生成图片、语音和视频。

## 支持的能力

- **图片生成与编辑** — gpt-image-1.5、gemini-3-pro-image-preview、gemini-3.1-flash-image-preview
- **语音合成** — elevenlabs/eleven_v3、elevenlabs/eleven_multilingual_v2
- **视频生成** — doubao-seedance-2-0-fast-260128、doubao-seedance-2-0-260128

## 安装 / 更新（面向 Agent）

按照以下步骤安装或更新 tapsvc-aigc 技能包。

### 1. 检测平台

确定当前系统的 OS 和架构，映射到对应的包名：

| OS | Arch | 包名 |
|----|------|------|
| Windows | x64 | `tapsvc-aigc-skill-win32-x64.zip` |
| Linux | x64 | `tapsvc-aigc-skill-linux-x64.tar.gz` |
| Linux | arm64/aarch64 | `tapsvc-aigc-skill-linux-arm64.tar.gz` |
| macOS | x64 | `tapsvc-aigc-skill-darwin-x64.tar.gz` |
| macOS | arm64/aarch64 | `tapsvc-aigc-skill-darwin-arm64.tar.gz` |

如果 OS/架构组合不在列表中，告知用户当前平台不受支持。

### 2. 下载最新 Release

通过 GitHub API 获取最新 release 并下载对应平台的资产：

**Linux / macOS (bash):**
```bash
# 获取最新 release 资产地址并下载
curl -s https://api.github.com/repos/zhing2006/tapsvc-aigc/releases/latest \
  | grep -o '"browser_download_url": *"[^"]*<PACKAGE_NAME>[^"]*"' \
  | head -1 | cut -d'"' -f4 \
  | xargs curl -L -o <PACKAGE_NAME>
```

**Windows (PowerShell):**
```powershell
# 获取最新 release 资产地址并下载
$release = Invoke-RestMethod "https://api.github.com/repos/zhing2006/tapsvc-aigc/releases/latest"
$asset = $release.assets | Where-Object { $_.name -eq "<PACKAGE_NAME>" }
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile <PACKAGE_NAME>
```

### 3. 解压到技能目录

将包解压到 Agent 平台的技能目录。压缩包内包含 `tapsvc-aigc/` 顶层目录，可直接放入技能目录。

**Claude Code:**
```bash
# tar.gz (Linux/macOS)
tar xzf <PACKAGE_NAME> -C ~/.claude/skills/

# zip (Windows, PowerShell)
Expand-Archive -Path <PACKAGE_NAME> -DestinationPath $env:USERPROFILE\.claude\skills\ -Force
```

**其他 Agent 平台:** 解压到你的平台的技能目录。

更新已有安装时，直接解压覆盖现有的 `tapsvc-aigc/` 目录即可。

### 4. 验证

运行 CLI 确认二进制可执行：

```bash
# Linux/macOS
<skill-dir>/tapsvc-aigc/scripts/tapsvc-aigc --help

# Windows
<skill-dir>\tapsvc-aigc\scripts\tapsvc-aigc.exe --help
```

命令成功执行则表示技能已安装就绪。

## 配置

CLI 需要两个环境变量用于 API 访问：

- `TAPSVC_BASE_URL` — API 代理基础 URL
- `TAPSVC_API_KEY` — API 密钥

通过工作目录下的 `.env` 文件或环境变量设置。CLI 在启动时自动从当前工作目录加载 `.env`。

## 许可证

Apache-2.0
