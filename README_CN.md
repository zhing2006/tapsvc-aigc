[English](README.md)

# tapsvc-aigc

AIGC 内容生成技能。通过 `tapsvc-aigc` CLI 生成图片、语音和视频。

## 支持的能力

- **图片生成与编辑** — gpt-image-2（默认）、gpt-image-1.5、gemini-3-pro-image、gemini-3.1-flash-image（兼容 `-preview` 别名）
- **语音合成** — elevenlabs/eleven_v3、elevenlabs/eleven_multilingual_v2
- **视频生成与编辑** — bytedance/seedance-2.5、doubao-seedance-2-0-fast-260128、doubao-seedance-2-0-260128、happyhorse-1.1-t2v、happyhorse-1.1-i2v、happyhorse-1.1-r2v、happyhorse-1.0-video-edit

网关提供多种协议：图片和语音使用 OpenAI 兼容端点，Seedance 使用
Volcengine 透传，HappyHorse 使用 DashScope 透传。`/v1/models` 返回的是跨协议
扁平目录，模型出现在列表中并不表示它能在所有端点上使用。

查询实时模型目录：

```bash
curl -s -H "Authorization: Bearer $TAPSVC_API_KEY" \
  "${TAPSVC_BASE_URL:-https://llm-proxy.tapsvc.com}/v1/models" \
  | jq -r '.data[].id'
```

视频能力重点：

- Seedance 2.5 支持最长 30 秒单次连贯输出、纯参考音频驱动、最多 30 张参考图，并可输出 `mov`；分辨率上限为 480p/720p。
- Seedance 2.0 完整版支持 480p、720p、1080p、4k；fast 版支持 480p/720p。
- HappyHorse 支持文生视频、首帧图生视频、多参考图生视频和已有视频编辑，分辨率为 720p/1080p。
- 使用 `tapsvc-aigc video get <task-id> --provider happyhorse` 查询 HappyHorse 任务。

## 安装 / 更新

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

将包解压到当前环境的技能目录。`<skills-dir>` 是 Agent 平台存放所有已安装技能的目录。压缩包内包含 `tapsvc-aigc/` 顶层目录，可直接放入技能目录。

#### 各 Agent 平台的 `<skills-dir>` 路径

| Agent 平台 | 作用域 | `<skills-dir>` 路径 |
|---|---|---|
| **Claude Code** | 个人（全局） | `~/.claude/skills` |
| **Claude Code** | 项目级 | `<project-root>/.claude/skills` |
| **Codex** | 个人（全局） | `~/.agents/skills` |
| **Codex** | 项目级 | `<repo-root>/.agents/skills` |
| **Codex** | 系统级 | `/etc/codex/skills` |
| **OpenClaw** | 工作区 | `<workspace>/skills` |
| **OpenClaw** | 个人（全局） | `~/.openclaw/skills` |
| **OpenClaw** | Docker 容器内 | `/home/node/.openclaw/skills` |
| **Hermes Agent** | 个人（全局） | `~/.hermes/skills` |
| **Hermes Agent** | Docker 容器内 | `/opt/data/skills` |

> **Docker 部署说明：**
> - **OpenClaw** — 宿主机 `~/.openclaw` 通过 bind-mount 映射到容器内 `/home/node/.openclaw`。技能目录结构和优先级与直接安装完全一致，唯一区别是挂载边界。如果技能依赖外部二进制文件，需通过 `agents.defaults.sandbox.docker.setupCommand` 安装。
> - **Hermes Agent** — 宿主机 `~/.hermes` 通过 bind-mount 映射到容器内 `/opt/data`。技能以只读卷方式自动挂载，技能声明的凭证文件也会自动挂载。技能数据通过此挂载在容器重启后持久保留。

```bash
# tar.gz (Linux/macOS)
tar xzf <PACKAGE_NAME> -C <skills-dir>/

# zip (Windows, PowerShell)
Expand-Archive -Path <PACKAGE_NAME> -DestinationPath <skills-dir>\ -Force
```

更新已有安装时，直接解压覆盖现有的 `tapsvc-aigc/` 目录即可。

### 4. 验证

运行 CLI 确认二进制可执行：

```bash
# Linux/macOS
<skills-dir>/tapsvc-aigc/scripts/tapsvc-aigc --help

# Windows
<skills-dir>\tapsvc-aigc\scripts\tapsvc-aigc.exe --help
```

命令成功执行则表示技能已安装就绪。

## 配置

CLI 需要两个环境变量用于 API 访问：

- `TAPSVC_BASE_URL` — API 代理基础 URL
- `TAPSVC_API_KEY` — API 密钥

通过工作目录下的 `.env` 文件或环境变量设置。CLI 在启动时自动从当前工作目录加载 `.env`。

## Release 流程

推送 `v*` 标签会触发 GitHub Release 工作流。工作流使用锁定依赖为五个平台构建可静态分发的二进制，
对可运行产物执行冒烟测试，在 Windows 上验证 VCRuntime 静态链接，打包 `SKILL.md`、参考文档和
对应平台二进制，最后创建带自动生成说明的 GitHub Release。

## 许可证

Apache-2.0
