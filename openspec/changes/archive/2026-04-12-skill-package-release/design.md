## Context

当前 release workflow（`.github/workflows/release.yml`）在 tag push 时为 5 个平台构建裸二进制文件并上传到 GitHub Release。用户需要手动将 SKILL.md 和 references 文件组合才能使用技能。

仓库根 `README.md` 目前只有一行标题，无安装指引。

CLI 通过 `dotenvy::dotenv()` 从 **当前工作目录** 加载 `.env`，输出文件也写入 CWD。这意味着 Agent 必须在用户工作目录执行 CLI，不能 cd 到技能安装目录。

## Goals / Non-Goals

**Goals:**

- Release 产出从裸二进制改为可安装的技能包
- Agent 能通过阅读 README.md 自动完成安装/更新
- SKILL.md 明确 CWD 执行约束
- README.md 提供双语版本（EN + CN）

**Non-Goals:**

- 不做 skill registry 或包管理器集成
- 不做自动更新检测（Agent 按用户指示检查更新）
- 不改变 CI workflow（ci.yml 不动）
- 不做 scripts/ 目录在仓库中的占位（仅在包内存在）

## Decisions

### Decision 1: 压缩格式按平台区分

Windows 使用 `.zip`（原生支持，无需额外工具），Linux/macOS 使用 `.tar.gz`（标准选择）。

**备选**: 全部统一 `.zip` — 简化 README 说明，但 Linux/macOS 上 `.tar.gz` 更自然且保留文件权限。

### Decision 2: 包内目录结构

```
tapsvc-aigc-skill-{platform}-{arch}.{ext}
└── tapsvc-aigc/
    ├── SKILL.md
    ├── references/
    │   ├── image-generation.md
    │   ├── audio-generation.md
    │   └── video-generation.md
    └── scripts/
        └── tapsvc-aigc[.exe]
```

解压后直接得到 `tapsvc-aigc/` 目录，可以整个移动到技能目录。README 不打包 — 它留在仓库里供 Agent 在安装前阅读。

**备选**: 包内用 `skills/tapsvc-aigc/` 路径 — 多一层无意义嵌套，不同 Agent 平台的技能根目录不同。

### Decision 3: 资产命名规范

`tapsvc-aigc-skill-{platform}-{arch}.{ext}`

| 平台 | 包名 |
|------|------|
| Windows x64 | `tapsvc-aigc-skill-win32-x64.zip` |
| Linux x64 | `tapsvc-aigc-skill-linux-x64.tar.gz` |
| Linux ARM64 | `tapsvc-aigc-skill-linux-arm64.tar.gz` |
| macOS x64 | `tapsvc-aigc-skill-darwin-x64.tar.gz` |
| macOS ARM64 | `tapsvc-aigc-skill-darwin-arm64.tar.gz` |

Agent 通过 OS + arch 映射到包名，从 GitHub Release 下载。

### Decision 4: README.md 双语结构

- `README.md` — 英文主版本，文件头有中文版链接
- `README_CN.md` — 中文版，文件头有英文版链接

两个文件只存在于仓库根目录，不打包进技能包。Agent 在安装前从 GitHub 仓库阅读 README，安装后使用 SKILL.md。

### Decision 5: SKILL.md CWD 约束

在 Step 0 Preflight 部分追加说明：

> Run the CLI from the current working directory — do NOT cd into the skill directory.
> The CLI loads `.env` from CWD and writes output files to CWD.

确保 Agent 不会错误地在技能安装目录执行命令。

### Decision 6: README 安装流程设计

README 的安装指引面向 Agent，包含以下步骤：

1. 检测当前 OS + architecture
2. 映射到对应的包名
3. 通过 GitHub API 获取最新 release 的 asset download URL
4. 下载并解压到 Agent 的技能目录
5. 验证安装：执行 `scripts/tapsvc-aigc --help`（Windows 上为 `scripts/tapsvc-aigc.exe --help`）

更新流程与安装相同 — 覆盖整个技能目录。

### Decision 7: Release workflow 打包策略

在现有 build job 的基础上增加打包步骤：

1. 构建二进制（现有逻辑不变）
2. 新增 package step：将 `skills/tapsvc-aigc/` 目录和构建的二进制组装为技能包
3. Release job 上传技能包（替换裸二进制）

使用 matrix 变量控制压缩格式（zip vs tar.gz）和二进制后缀（.exe vs 无）。

## Risks / Trade-offs

- **[包体积]** 技能包比裸二进制大几 KB（多了 md 文件）→ 可忽略不计
- **[Agent 兼容性]** `${CLAUDE_SKILL_DIR}` 是 Claude Code 特有变量 → SKILL.md 已有 PATH 回退。非 Claude 平台需要其宿主环境提供等效的 skill-dir 变量，或由用户将 `scripts/` 加入 PATH。这是预期行为，不做额外适配
- **[GitHub API 限制]** 未认证请求 60 次/小时 → 安装是低频操作，不构成问题
