## ADDED Requirements

### Requirement: README 双语版本

仓库根目录 SHALL 包含 `README.md`（英文）和 `README_CN.md`（中文），两个文件的文件头 MUST 包含切换到另一语言版本的链接。

#### Scenario: 英文 README 有中文链接

- **WHEN** Agent 或用户打开 README.md
- **THEN** 文件头 MUST 包含指向 README_CN.md 的语言切换链接

#### Scenario: 中文 README 有英文链接

- **WHEN** Agent 或用户打开 README_CN.md
- **THEN** 文件头 MUST 包含指向 README.md 的语言切换链接

### Requirement: Agent 驱动的平台检测

README 的安装指引 SHALL 包含 Agent 可执行的平台检测步骤，使 Agent 能确定当前系统的 OS 和 architecture，并映射到对应的技能包名。

支持的平台映射：

| OS | Arch | 包名 |
|----|------|------|
| Windows | x64 | `tapsvc-aigc-skill-win32-x64.zip` |
| Linux | x64 | `tapsvc-aigc-skill-linux-x64.tar.gz` |
| Linux | arm64/aarch64 | `tapsvc-aigc-skill-linux-arm64.tar.gz` |
| macOS | x64 | `tapsvc-aigc-skill-darwin-x64.tar.gz` |
| macOS | arm64/aarch64 | `tapsvc-aigc-skill-darwin-arm64.tar.gz` |

#### Scenario: macOS ARM64 平台检测

- **WHEN** Agent 在 macOS ARM64 系统上执行平台检测
- **THEN** Agent MUST 映射到 `tapsvc-aigc-skill-darwin-arm64.tar.gz`

#### Scenario: 不支持的平台

- **WHEN** Agent 检测到的 OS/Arch 组合不在支持列表中
- **THEN** Agent MUST 告知用户当前平台不受支持

### Requirement: GitHub Release 下载流程

README 的安装指引 SHALL 指导 Agent 通过 GitHub API 获取最新 release 并下载对应平台的技能包。

#### Scenario: 获取最新 release

- **WHEN** Agent 执行安装流程
- **THEN** Agent MUST 通过 GitHub API 获取最新 release 的 asset 列表，找到对应平台的包并下载

### Requirement: 解压并部署到技能目录

README 的安装指引 SHALL 指导 Agent 将下载的技能包解压到当前 Agent 平台的技能目录。若 Agent 平台不提供技能目录变量或不会自动解析技能内的 `scripts/` 路径，README SHALL 指导 Agent 将 `scripts/` 下的 binary 加入 PATH，以确保后续 Preflight 能通过 PATH 找到 binary。

#### Scenario: 全新安装

- **WHEN** 技能目录中不存在 `tapsvc-aigc/`
- **THEN** Agent MUST 将下载的包解压到技能目录，使 `tapsvc-aigc/` 出现在技能目录下

#### Scenario: 更新已有安装

- **WHEN** 技能目录中已存在 `tapsvc-aigc/`
- **THEN** Agent MUST 覆盖整个 `tapsvc-aigc/` 目录完成更新

#### Scenario: 平台无技能目录变量

- **WHEN** Agent 平台不提供技能目录变量且不会自动解析 skill 相对路径
- **THEN** Agent MUST 将 `tapsvc-aigc/scripts/` 下的 binary 加入 PATH，确保后续 Preflight 可通过 PATH 找到 binary

### Requirement: 安装后验证

README 的安装指引 SHALL 包含验证步骤，确认安装成功。

#### Scenario: 验证安装成功

- **WHEN** Agent 完成解压部署
- **THEN** Agent MUST 执行技能目录下的 `tapsvc-aigc/scripts/tapsvc-aigc --help`（或 `.exe`），确认 binary 可执行

#### Scenario: 验证失败

- **WHEN** 验证步骤执行失败
- **THEN** Agent MUST 告知用户安装失败并提供错误信息
