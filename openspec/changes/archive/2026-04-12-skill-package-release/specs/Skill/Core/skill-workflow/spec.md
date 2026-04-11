## MODIFIED Requirements

### Requirement: Preflight 环境检查

Skill 工作流在执行任何 AIGC 命令之前，MUST 先执行 preflight 检查。检查项包括：

1. `tapsvc-aigc` 二进制是否可执行（Windows 上为 `tapsvc-aigc.exe`）。查找顺序：(a) 若 Agent 平台提供技能目录变量（如 Claude Code 的 `${CLAUDE_SKILL_DIR}`），先检查该目录下的 `scripts/tapsvc-aigc[.exe]`；(b) 若 (a) 不存在、不可执行、或平台不提供技能目录变量，再检查 PATH。两步均失败才判定为不可用
2. 若运行目录下不存在 `.env` 文件：检查环境变量 `TAPSVC_BASE_URL` 和 `TAPSVC_API_KEY` 是否已设置
3. 若运行目录下存在 `.env` 文件：跳过环境变量检查，依赖 CLI 自身在运行时加载

任一检查失败时，Agent MUST NOT 继续执行后续步骤，SHALL 给出具体的配置指引。

#### Scenario: 二进制不可用

- **WHEN** `tapsvc-aigc`（或 Windows 上的 `tapsvc-aigc.exe`）在技能目录的 `scripts/` 和 PATH 中均不存在
- **THEN** Agent MUST NOT 执行后续命令，SHALL 提示用户安装 tapsvc-aigc 或检查 PATH 配置

#### Scenario: 无 .env 且环境变量缺失

- **WHEN** 运行目录下不存在 `.env` 文件，且 `TAPSVC_BASE_URL` 或 `TAPSVC_API_KEY` 未设置
- **THEN** Agent MUST NOT 执行后续命令，SHALL 提示用户通过 `.env` 文件或环境变量配置缺失项

#### Scenario: 有 .env 文件

- **WHEN** 运行目录下存在 `.env` 文件
- **THEN** Agent SHALL 跳过环境变量检查，继续执行工作流

#### Scenario: 所有检查通过

- **WHEN** 二进制可执行且环境配置满足要求
- **THEN** Agent SHALL 继续执行工作流的提示词优化步骤

### Requirement: CLI 工作目录约束

Agent 执行 `tapsvc-aigc` CLI 命令时 MUST 在用户的当前工作目录下运行，MUST NOT cd 到技能安装目录。CLI 从当前工作目录加载 `.env` 文件并将输出文件写入当前工作目录。

#### Scenario: 在用户工作目录执行 CLI

- **WHEN** Agent 执行任何 `tapsvc-aigc` 命令
- **THEN** 命令的工作目录 MUST 为用户的当前工作目录，Agent MUST 通过绝对路径或 PATH 调用 CLI binary

#### Scenario: .env 从工作目录加载

- **WHEN** 用户工作目录下存在 `.env` 文件且 Agent 执行 CLI 命令
- **THEN** CLI MUST 从用户工作目录加载 `.env`，而非从技能安装目录加载

### Requirement: Skill 主工作流

Skill 的完整工作流 SHALL 按以下顺序执行：

1. **Step 0 — Preflight**: 检查二进制和环境变量
2. **Step 1 — 优化提示词**: 根据对应模态的 reference 文件优化用户输入
3. **Step 2 — 执行命令**: 按 reference 文件中的 CLI 用法构建并执行命令
4. **Step 3 — 交付结果**: 将生成的结果交付给用户。Agent SHALL 根据当前运行环境自主判断最佳交付方式：若存在可用的发送通道（如消息推送、文件传输等），SHALL 通过该通道发送结果；若无可用通道，SHALL 直接向用户报告结果（文件路径、预览等）

Agent MUST NOT 跳过 Step 0 直接执行命令。

#### Scenario: 完整工作流执行

- **WHEN** 用户请求任何 AIGC 任务（图片/音频/视频）
- **THEN** Agent SHALL 按 Step 0 → Step 1 → Step 2 → Step 3 顺序执行，不得跳过任何步骤

#### Scenario: 交付结果 — 存在发送通道

- **WHEN** 命令执行成功且当前环境存在可用的结果发送通道
- **THEN** Agent SHALL 通过该通道将生成的文件发送给用户

#### Scenario: 交付结果 — 无发送通道

- **WHEN** 命令执行成功但当前环境无可用的发送通道
- **THEN** Agent SHALL 向用户报告生成结果的文件路径及相关信息
