## 1. Skill 主文件

- [x] 1.1 创建 `skills/tapsvc-aigc/SKILL.md`，包含 frontmatter（name、description、allowed-tools: Bash Read Write）和主工作流指令，工作流为：Step 0 preflight 检查（验证二进制可用、检查 .env 或环境变量）→ Step 1 优化提示词 → Step 2 执行命令 → Step 3 交付结果。引用 references 下的三个文件，模态和模型详情由 reference 文件提供。注明安装方式和后续安装脚本规划

## 2. Image Reference

- [x] 2.1 创建 `skills/tapsvc-aigc/references/image-generation.md`，包含模型对比表（gpt-image-1.5 DEFAULT、gemini-3-pro-image-preview、gemini-3.1-flash-image-preview）及各模型参数限制
- [x] 2.2 编写所有模型 n=1 限制说明（litellm 代理限制），以及需要多张时通过多次命令执行的方案
- [x] 2.3 编写图片生成提示词最佳实践（结构化 prompt、风格关键字、摄影术语、文字渲染、负面提示）
- [x] 2.4 编写图片编辑提示词最佳实践（变更隔离、保持不变量、mask 用法、迭代策略）
- [x] 2.5 编写 mask 编辑模型限制说明（仅 gpt-image-1.5 支持）

## 3. Audio Reference

- [x] 3.1 创建 `skills/tapsvc-aigc/references/audio-generation.md`，包含模型对比表（elevenlabs/eleven_v3 DEFAULT、elevenlabs/eleven_multilingual_v2）及各模型参数限制（eleven_v3 最大 3000 字符、eleven_multilingual_v2 最大 10000 字符），明确文本长度自动切换规则
- [x] 3.2 编写 10 个默认音色映射表（CLI 名称 → ElevenLabs 名称 → Voice ID）及自定义 Voice ID 使用说明
- [x] 3.3 编写文本格式化最佳实践（数字/缩写/符号规范化为可朗读文字）
- [x] 3.4 编写 Audio Tags 使用指南（情绪、反应、语气标签列表及使用示例）
- [x] 3.5 编写 Stability 设置和发音控制说明

## 4. Video Reference

- [x] 4.1 创建 `skills/tapsvc-aigc/references/video-generation.md`，包含模型对比表（doubao-seedance-2-0-fast-260128 DEFAULT、doubao-seedance-2-0-260128）及模型差异说明
- [x] 4.2 编写参数约束规则表（--last-frame REQUIRES --first-frame、--first-frame EXCLUSIVE WITH --ref-image/--ref-video、--ref-audio REQUIRES --ref-image OR --ref-video、--duration 4-15 or -1、--ref-image max 9、--ref-video max 3 URL only、--ref-audio max 3）
- [x] 4.3 编写提示词结构最佳实践（主体 + 动作 + 场景 + 镜头 + 风格）
- [x] 4.4 编写镜头语言词汇表（推/拉/摇/移/跟/环绕/升降/变焦及对应英文术语）
- [x] 4.5 编写风格关键字参考和角色一致性技巧
- [x] 4.6 编写多动作序列和多镜头切换描述技巧
