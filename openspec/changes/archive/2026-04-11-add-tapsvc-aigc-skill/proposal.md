## Why

tapsvc-aigc CLI 已经支持图片生成/编辑、音频合成、视频生成等 AIGC 功能，但用户在 Claude Code 中使用时需要手动构造复杂的命令行参数。通过创建 Agent Skill，Claude 可以自动根据用户意图选择模型、优化提示词、构建命令并执行，大幅降低使用门槛。

## What Changes

- 新增 `skills/tapsvc-aigc/SKILL.md`，定义 Skill 元数据、工作流指令和模型默认值
- 新增 `skills/tapsvc-aigc/references/image-generation.md`，包含图片生成/编辑模型对比和提示词最佳实践
- 新增 `skills/tapsvc-aigc/references/audio-generation.md`，包含音频模型音色表、Audio Tags 和文本格式化最佳实践
- 新增 `skills/tapsvc-aigc/references/video-generation.md`，包含视频生成提示词结构、镜头语言和风格词最佳实践
- Skill 工作流：Step 0 preflight 环境检查 → Step 1 优化提示词 → Step 2 执行 CLI 命令 → Step 3 交付结果

## Capabilities

### New Capabilities

- `Skill/Core/skill-workflow`: Skill 主工作流定义，包含 preflight 环境检查（二进制可用性、环境变量）和四步工作流顺序
- `Skill/Image/image-skill`: 图片生成与编辑 Skill 指令，覆盖 gpt-image-1.5（DEFAULT）、gemini-3-pro-image-preview、gemini-3.1-flash-image-preview，包含提示词优化最佳实践
- `Skill/Audio/audio-skill`: 音频生成 Skill 指令，覆盖 elevenlabs/eleven_v3（DEFAULT）、elevenlabs/eleven_multilingual_v2，包含 10 个默认音色、自定义 Voice ID 说明和文本格式化最佳实践
- `Skill/Video/video-skill`: 视频生成 Skill 指令，覆盖 doubao-seedance-2-0-fast-260128（DEFAULT）、doubao-seedance-2-0-260128，包含提示词结构和镜头语言最佳实践，仅支持 generate/get 命令

### Modified Capabilities

（无）

## Impact

- 新增 `skills/` 目录（项目根目录），不影响现有 Rust 代码和构建
- 不涉及任何 API 变更或依赖变更
- 仅影响 Claude Code 的 Skill 发现和加载行为
