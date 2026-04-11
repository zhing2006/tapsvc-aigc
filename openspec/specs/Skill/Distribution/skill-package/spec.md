## Purpose

Define the structure, naming conventions, and release workflow for packaging the tapsvc-aigc skill as distributable archives across all supported platforms.

## Requirements

### Requirement: 技能包结构

Release 产出的技能包 SHALL 包含以下目录结构：

```
tapsvc-aigc/
├── SKILL.md
├── references/
│   ├── image-generation.md
│   ├── audio-generation.md
│   └── video-generation.md
└── scripts/
    └── tapsvc-aigc[.exe]
```

解压后 MUST 得到 `tapsvc-aigc/` 顶层目录，可以直接移动到 Agent 的技能目录。README 文件不包含在技能包内，仅存在于 GitHub 仓库供安装前阅读。

#### Scenario: 解压后目录结构正确

- **WHEN** 用户解压任一平台的技能包
- **THEN** MUST 得到 `tapsvc-aigc/` 目录，其中包含 SKILL.md、references/ 子目录（含 3 个 md 文件）和 scripts/ 子目录（含可执行的 CLI binary）

### Requirement: 压缩格式按平台区分

Windows 平台的技能包 SHALL 使用 `.zip` 格式，Linux 和 macOS 平台 SHALL 使用 `.tar.gz` 格式。

#### Scenario: Windows 包为 zip

- **WHEN** 构建 Windows x64 平台的技能包
- **THEN** 产出文件 MUST 为 `.zip` 格式

#### Scenario: Unix 包为 tar.gz

- **WHEN** 构建 Linux 或 macOS 平台的技能包
- **THEN** 产出文件 MUST 为 `.tar.gz` 格式

### Requirement: 资产命名规范

技能包文件名 SHALL 遵循 `tapsvc-aigc-skill-{platform}-{arch}.{ext}` 格式。

平台映射表：

| Target | 包名 |
|--------|------|
| x86_64-pc-windows-msvc | `tapsvc-aigc-skill-win32-x64.zip` |
| x86_64-unknown-linux-musl | `tapsvc-aigc-skill-linux-x64.tar.gz` |
| aarch64-unknown-linux-musl | `tapsvc-aigc-skill-linux-arm64.tar.gz` |
| x86_64-apple-darwin | `tapsvc-aigc-skill-darwin-x64.tar.gz` |
| aarch64-apple-darwin | `tapsvc-aigc-skill-darwin-arm64.tar.gz` |

#### Scenario: 包名与平台匹配

- **WHEN** 为 `aarch64-apple-darwin` 构建技能包
- **THEN** 产出文件名 MUST 为 `tapsvc-aigc-skill-darwin-arm64.tar.gz`

### Requirement: Markdown 文件跨平台一致

所有 5 个技能包中的 SKILL.md 和 references/ 下的文件 SHALL 来自同一源文件（仓库中 `skills/tapsvc-aigc/` 目录）。唯一的差异 SHALL 只在 `scripts/` 下的 CLI binary。

#### Scenario: 不同平台包的 md 文件来源一致

- **WHEN** 5 个平台的 build job 组装技能包
- **THEN** 每个 job MUST 从同一 commit 的 `skills/tapsvc-aigc/` 复制 markdown 文件，确保内容一致

### Requirement: Release workflow 产出技能包

Release workflow 在 tag push 时 SHALL 构建 5 个平台的技能包并上传到 GitHub Release。不再上传裸二进制文件。

#### Scenario: Tag push 触发构建

- **WHEN** push 一个 `v*` 格式的 tag
- **THEN** Release workflow MUST 构建 5 个技能包并上传到 GitHub Release

#### Scenario: Release 中无裸二进制

- **WHEN** Release 构建完成
- **THEN** Release assets 中 MUST NOT 包含裸二进制文件，MUST 只包含 5 个技能压缩包
