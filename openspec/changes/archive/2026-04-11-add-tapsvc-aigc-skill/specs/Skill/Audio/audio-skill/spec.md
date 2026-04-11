## ADDED Requirements

### Requirement: Audio Skill reference 文件

Skill SHALL 在 `references/audio-generation.md` 中提供音频生成的完整参考文档，包含以下内容：

1. 支持模型对比表（elevenlabs/eleven_v3 DEFAULT、elevenlabs/eleven_multilingual_v2），列出各模型的参数限制差异
2. 10 个默认音色映射表（CLI 名称 → ElevenLabs 名称 → Voice ID）
3. 自定义音色使用说明（直接传入 ElevenLabs Voice ID）
4. 文本格式化最佳实践（数字/缩写规范化、标点控制节奏）
5. Audio Tags 使用指南（情绪标签、反应标签、语气标签）
6. Stability 设置说明（Creative / Natural / Robust）

#### Scenario: Agent 加载 audio reference

- **WHEN** 用户请求生成音频
- **THEN** Agent SHALL 读取 `references/audio-generation.md` 获取音色表和文本格式化最佳实践

### Requirement: Audio 默认音色表

Skill SHALL 在 reference 文件中列出以下 10 个默认音色映射：

| CLI Voice | ElevenLabs Voice | Voice ID |
|-----------|-----------------|----------|
| alloy | Rachel | 21m00Tcm4TlvDq8ikWAM |
| amber | Paul | 5Q0t7uMcjvnagumLfvZi |
| ash | Domi | AZnzlk1XvdvUeBnXmlld |
| august | Fin | D38z5RcWu1voky8WS1ja |
| blue | Clyde | 2EiwWnXFnvU5JabPnv8n |
| coral | Aria | 9BWtsMINqrJLrRacOk9x |
| lily | Sarah | EXAVITQu4vr4xnSDxMaL |
| onyx | Drew | 29vD33N1CtxCmqQRPOHJ |
| sage | Roger | CwhRBWXzGAHq8TQ4Fs17 |
| verse | Dave | CYw3kZ02Hs0563khs1Fj |

#### Scenario: 用户使用默认音色

- **WHEN** 用户请求音频生成并选择"alloy"音色
- **THEN** Agent SHALL 使用 `--voice alloy` 参数

### Requirement: Audio 自定义音色

当用户提供不在默认 10 个音色中的名称时，Agent SHALL 将其视为 ElevenLabs Voice ID 直接传入 `--voice` 参数。

#### Scenario: 用户使用自定义 Voice ID

- **WHEN** 用户提供 ElevenLabs Voice ID（如 `pNInz6obpgDQGcFmaJgB`）
- **THEN** Agent SHALL 使用 `--voice pNInz6obpgDQGcFmaJgB` 直接传入

#### Scenario: 用户描述想要的声音特征

- **WHEN** 用户描述想要的声音特征（如"年轻女性、温柔"）但未指定具体音色
- **THEN** Agent SHALL 从默认 10 个音色中推荐最匹配的，并说明推荐理由

### Requirement: Audio 文本优化

Agent SHALL 在执行音频生成命令之前，根据 `references/audio-generation.md` 中的最佳实践优化用户提供的文本。

优化策略与图/视频的"重写 prompt"不同，音频优化聚焦于文本格式化：
- 规范化数字、缩写、符号为可朗读文字
- 根据语境插入适当的 Audio Tags（如 `[laughs]`、`[sighs]`、`[excited]`）
- 调整标点控制停顿节奏（省略号增加停顿、感叹号增强语气）
- 确保文本长度满足模型最低要求（eleven_v3 建议 > 250 字符）

#### Scenario: 优化包含数字的文本

- **WHEN** 用户提供包含"$42.50"的文本
- **THEN** Agent SHALL 将其规范化为"forty-two dollars and fifty cents"（英文）或对应中文表述

#### Scenario: 插入情绪 Audio Tags

- **WHEN** 用户文本包含明显的情绪意图（如对白中的叹息）
- **THEN** Agent SHALL 在适当位置插入 Audio Tags（如 `[sighs]`），并告知用户已添加

### Requirement: Audio 模型自动选择

Agent SHALL 根据输入文本长度自动选择合适的模型：

- 文本 ≤ 3000 字符且用户未指定模型 → 使用 `elevenlabs/eleven_v3`（DEFAULT）
- 文本 > 3000 字符且 ≤ 10000 字符 → 自动切换到 `elevenlabs/eleven_multilingual_v2`，并告知用户切换原因
- 文本 > 10000 字符 → 中止执行，提示用户拆分文本

当用户显式指定模型时，Agent 仍 SHALL 检查文本长度是否超出该模型限制，超出时 MUST 警告用户。

#### Scenario: 使用默认音频模型（短文本）

- **WHEN** 用户请求生成音频且未指定模型，输入文本 ≤ 3000 字符
- **THEN** Agent SHALL 使用 `-m elevenlabs/eleven_v3` 参数

#### Scenario: 长文本自动切换模型

- **WHEN** 用户请求生成音频且未指定模型，输入文本超过 3000 字符但不超过 10000 字符
- **THEN** Agent SHALL 自动切换到 `-m elevenlabs/eleven_multilingual_v2`，并告知用户"输入文本超过 eleven_v3 的 3000 字符限制，已自动切换到 eleven_multilingual_v2"

#### Scenario: 超长文本中止

- **WHEN** 输入文本超过 10000 字符
- **THEN** Agent SHALL 中止执行，提示用户将文本拆分为多段分别生成

#### Scenario: 用户指定模型但文本超限

- **WHEN** 用户显式指定 `elevenlabs/eleven_v3` 但输入文本超过 3000 字符
- **THEN** Agent MUST NOT 执行该命令，SHALL 告知用户文本超出模型限制，并要求用户选择：切换到 `elevenlabs/eleven_multilingual_v2`，或拆分文本后重试

### Requirement: Audio CLI 命令构建

Agent SHALL 根据用户意图正确构建 `tapsvc-aigc audio speech` 命令，包含所有必要参数。

#### Scenario: 构建音频生成命令

- **WHEN** Agent 完成文本优化
- **THEN** Agent SHALL 构建包含 `-m`、`--voice`、`-i`（或 `--input-file`）以及用户指定的可选参数（`--format`、`--speed`、`-o`）的完整命令
