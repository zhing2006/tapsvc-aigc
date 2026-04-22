[中文版 / Chinese](README_CN.md)

# tapsvc-aigc

AIGC content generation Agent Skill. Generates images, audio speech, and videos via the `tapsvc-aigc` CLI.

## Supported Capabilities

- **Image generation & editing** — gpt-image-2 (default), gpt-image-1.5, gemini-3-pro-image-preview, gemini-3.1-flash-image-preview
- **Audio speech synthesis** — elevenlabs/eleven_v3, elevenlabs/eleven_multilingual_v2
- **Video generation** — doubao-seedance-2-0-fast-260128, doubao-seedance-2-0-260128

## Install / Update (for Agents)

Follow the steps below to install or update the tapsvc-aigc skill package.

### 1. Detect Platform

Determine the current OS and architecture, then map to the correct package name:

| OS | Arch | Package |
|----|------|---------|
| Windows | x64 | `tapsvc-aigc-skill-win32-x64.zip` |
| Linux | x64 | `tapsvc-aigc-skill-linux-x64.tar.gz` |
| Linux | arm64/aarch64 | `tapsvc-aigc-skill-linux-arm64.tar.gz` |
| macOS | x64 | `tapsvc-aigc-skill-darwin-x64.tar.gz` |
| macOS | arm64/aarch64 | `tapsvc-aigc-skill-darwin-arm64.tar.gz` |

If the OS/arch combination is not listed, inform the user that their platform is not supported.

### 2. Download Latest Release

Use the GitHub API to find the latest release and download the matching asset:

**Linux / macOS (bash):**
```bash
# Get latest release asset URL and download
curl -s https://api.github.com/repos/zhing2006/tapsvc-aigc/releases/latest \
  | grep -o '"browser_download_url": *"[^"]*<PACKAGE_NAME>[^"]*"' \
  | head -1 | cut -d'"' -f4 \
  | xargs curl -L -o <PACKAGE_NAME>
```

**Windows (PowerShell):**
```powershell
# Get latest release asset URL and download
$release = Invoke-RestMethod "https://api.github.com/repos/zhing2006/tapsvc-aigc/releases/latest"
$asset = $release.assets | Where-Object { $_.name -eq "<PACKAGE_NAME>" }
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile <PACKAGE_NAME>
```

### 3. Extract to Skill Directory

Extract the package to the skill directory of your current environment. `<skills-dir>` is the directory where the Agent platform stores all installed skills. The archive contains a `tapsvc-aigc/` top-level directory that can be placed directly into the skill directory.

#### `<skills-dir>` by Agent Platform

| Agent Platform | Scope | `<skills-dir>` Path |
|---|---|---|
| **Claude Code** | Personal (global) | `~/.claude/skills` |
| **Claude Code** | Project | `<project-root>/.claude/skills` |
| **Codex** | Personal (global) | `~/.agents/skills` |
| **Codex** | Project | `<repo-root>/.agents/skills` |
| **Codex** | System | `/etc/codex/skills` |
| **OpenClaw** | Workspace | `<workspace>/skills` |
| **OpenClaw** | Personal (global) | `~/.openclaw/skills` |
| **OpenClaw** | Docker | `/home/node/.openclaw/skills` (container) |
| **Hermes Agent** | Personal (global) | `~/.hermes/skills` |
| **Hermes Agent** | Docker | `/opt/data/skills` (container) |

> **Docker deployment notes:**
> - **OpenClaw** — Host `~/.openclaw` is bind-mounted to `/home/node/.openclaw` inside the container. Skill directory structure and precedence are identical to a direct install; the only difference is the mount boundary. If skills depend on external binaries, they must be installed via `agents.defaults.sandbox.docker.setupCommand`.
> - **Hermes Agent** — Host `~/.hermes` is bind-mounted to `/opt/data` inside the container. Skills are auto-mounted as read-only volumes. Credential files declared by skills are also mounted automatically. Skills persist across container restarts via this mount.

```bash
# tar.gz (Linux/macOS)
tar xzf <PACKAGE_NAME> -C <skills-dir>/

# zip (Windows, PowerShell)
Expand-Archive -Path <PACKAGE_NAME> -DestinationPath <skills-dir>\ -Force
```

When updating an existing installation, extract over the existing `tapsvc-aigc/` directory to overwrite all files.

### 4. Verify

Run the CLI to confirm the binary is executable:

```bash
# Linux/macOS
<skills-dir>/tapsvc-aigc/scripts/tapsvc-aigc --help

# Windows
<skills-dir>\tapsvc-aigc\scripts\tapsvc-aigc.exe --help
```

If the command succeeds, the skill is installed and ready.

## Configuration

The CLI requires two environment variables for API access:

- `TAPSVC_BASE_URL` — API proxy base URL
- `TAPSVC_API_KEY` — API key

Set them via a `.env` file in your working directory or as environment variables. The CLI loads `.env` from the current working directory at startup.

## License

Apache-2.0
