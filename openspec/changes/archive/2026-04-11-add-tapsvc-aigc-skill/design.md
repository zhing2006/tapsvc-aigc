## Context

tapsvc-aigc 是一个 Rust CLI 工具，支持通过 OpenAI 兼容 API 和火山引擎 ARK API 进行图片生成/编辑、音频合成和视频生成。目前用户需要手动构造 CLI 命令，包括选择模型、编写提示词、指定参数等。

项目遵循 [Agent Skills 开放标准](https://agentskills.io/specification)，Skill 由 `SKILL.md` + 可选的 `references/`、`scripts/`、`assets/` 目录组成。Claude Code 在标准基础上扩展了 `allowed-tools`、`disable-model-invocation` 等字段。

## Goals / Non-Goals

**Goals:**

- 创建符合 Agent Skills 开放标准的 Skill，让 Claude 自动处理 AIGC 任务
- 实现四步工作流：preflight 环境检查 → 优化提示词 → 执行 CLI 命令 → 交付结果
- 为每种模态（图/音/视频）提供独立的 reference 文件，包含模型对比和提示词最佳实践
- 支持用户通过自然语言触发，也支持 `/tapsvc-aigc` 手动触发

**Non-Goals:**

- 不修改现有 Rust CLI 代码
- 不覆盖 `video list` 和 `video delete` 命令
- 不创建安装脚本（后续单独 change 处理，安装脚本负责将 `tapsvc-aigc` 二进制放到 `scripts/` 目录并注册到 `.claude/skills/`）

## Decisions

### 1. Skill 目录位置：`skills/tapsvc-aigc/`（项目根目录）

**选择**: 放在项目根目录 `skills/` 下，而非 `.claude/skills/`。

**理由**: 用户明确要求"开发而不安装"。作为项目产出物存放在版本控制中，方便独立分发。使用时通过 `--add-dir` 或手动拷贝到 `.claude/skills/` 安装。

**替代方案**: `.claude/skills/tapsvc-aigc/` 可被 Claude Code 自动发现，但混合了开发产物和本地配置。

### 2. SKILL.md 使用默认调用模式（两者兼有）

**选择**: 不设置 `disable-model-invocation`，不设置 `user-invocable: false`。

**理由**: 用户希望既能说"帮我生成一张图"自动触发，也能 `/tapsvc-aigc generate an image` 手动触发。

### 3. `allowed-tools: Bash Read Write`

**选择**: 预授权 Bash、Read、Write 三个工具。

**理由**:
- `Bash`: 执行 `tapsvc-aigc` CLI 命令和文件操作
- `Read`: 加载 references 文件和查看生成的图片（多模态预览）
- `Write`: 将优化后的长提示词写入临时文件，通过 `--prompt-file` 传入

### 4. references 按模态拆分为三个文件

**选择**: `image-generation.md`、`audio-generation.md`、`video-generation.md`。

**理由**: 遵循渐进式披露原则，Claude 只在需要时加载对应模态的 reference 文件，节省上下文窗口。每个文件包含模型对比表 + 提示词最佳实践。

### 5. 图片编辑需要 mask 时自动切换到 gpt-image-1.5

**选择**: SKILL.md 中明确指令：当用户需要 mask 编辑时，MUST 使用 gpt-image-1.5。

**理由**: Gemini 模型不支持 mask 参数（被 LiteLLM proxy 拒绝）。只有 gpt-image-1.5 完整支持 mask 编辑。

### 6. 图片生成数量统一限制 n=1

**选择**: 所有图片模型统一限制 `n=1`，不使用 `--n` 参数。

**理由**: litellm 代理层限制，所有模型实际仅支持单张生成。需要多张时通过多次命令执行实现。

### 7. 音频模型根据文本长度自动选择

**选择**: Claude 在构建命令前检查输入文本字符数，≤ 3000 用 eleven_v3，3001-10000 自动切换 eleven_multilingual_v2，> 10000 中止提示拆分。

**理由**: eleven_v3 限制 3000 字符，eleven_multilingual_v2 限制 10000 字符。SKILL 指令引导 Claude 在构建命令前做长度判断，避免产出必然失败的命令。

### 8. 视频参数约束前置校验

**选择**: 在 reference 文件和 spec 中明确列出 CLI 的参数互斥/依赖规则，要求 Claude 在构建命令前校验。

**理由**: CLI 有硬校验（`--last-frame` 依赖 `--first-frame`、`--first-frame` 与 `--ref-image` 互斥等），不应靠 CLI 报错兜底。Skill 设计中将这些规则前置为 Claude 的决策规则。

### 9. Preflight 环境检查

**选择**: 工作流第一步增加 preflight 检查，验证 `tapsvc-aigc` 二进制可用性和环境变量配置。

**理由**: CLI 运行依赖 `TAPSVC_BASE_URL` 和 `TAPSVC_API_KEY`，缺失时直接执行会产生不友好的错误。preflight 提前检查并给出配置指引。最终 `tapsvc-aigc` 二进制将通过安装脚本放置到 `${CLAUDE_SKILL_DIR}/scripts/` 目录下。

### 10. 音频提示词优化策略与图/视频不同

**选择**: 音频的"优化提示词"步骤定义为文本格式化（规范化数字/缩写、控制标点节奏、插入 Audio Tags），而非重写内容。

**理由**: 音频输入是要朗读的文本内容，不是创作型 prompt。ElevenLabs 的最佳实践集中在文本规范化和情绪标签控制。

## Risks / Trade-offs

- **[Skill 未被自动发现]** → `skills/` 在项目根目录，不在 `.claude/skills/`，Claude Code 不会自动发现。用户需手动安装或通过 `--add-dir` 加载。设计文档中已标注此为有意选择。
- **[模型 API 变更]** → references 中的模型参数和限制可能随 API 更新而变化。通过将这些信息集中在 reference 文件中，更新时只需修改对应文件。
- **[上下文窗口消耗]** → SKILL.md 正文 + reference 文件可能较长。通过渐进式披露（SKILL.md < 500 行，reference 按需加载）控制消耗。
- **[安装脚本未包含]** → 当前 change 仅产出 Skill 文件，安装脚本（拷贝二进制到 `scripts/`、注册到 `.claude/skills/`）将在后续 change 中实现。
