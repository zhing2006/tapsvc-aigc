## ADDED Requirements

### Requirement: Image Skill reference 文件

Skill SHALL 在 `references/image-generation.md` 中提供图片生成与编辑的完整参考文档，包含以下内容：

1. 支持模型对比表（gpt-image-1.5 DEFAULT、gemini-3-pro-image-preview、gemini-3.1-flash-image-preview），列出各模型的参数限制差异
2. 图片生成提示词最佳实践（结构、风格关键字、摄影术语、文字渲染、负面提示）
3. 图片编辑提示词最佳实践（变更隔离、保持不变量、mask 用法）
4. 各模型的特有注意事项和限制

#### Scenario: Agent 加载 image reference

- **WHEN** 用户请求生成或编辑图片
- **THEN** Agent SHALL 读取 `references/image-generation.md` 获取模型信息和提示词最佳实践

### Requirement: Image 提示词优化

Agent SHALL 在执行图片生成/编辑命令之前，根据 `references/image-generation.md` 中的最佳实践优化用户提示词。

优化内容包括：
- 补充具体的材质、光照、构图、视角描述
- 添加适当的风格关键字和质量提示
- 对于需要文字渲染的场景，使用引号包裹文本并指定排版细节
- 对于编辑场景，明确指定变更范围和不变量

#### Scenario: 优化图片生成提示词

- **WHEN** 用户提供简短的图片描述（如"一只猫"）
- **THEN** Agent SHALL 将其扩展为包含主体、环境、风格、光照、细节的结构化提示词

#### Scenario: 优化图片编辑提示词

- **WHEN** 用户请求编辑图片
- **THEN** Agent SHALL 在提示词中明确指定"change only X"和"keep everything else the same"的保护指令

### Requirement: Image 默认模型

当用户未指定模型时，图片生成和编辑 SHALL 默认使用 `gpt-image-1.5`。

#### Scenario: 使用默认图片模型

- **WHEN** 用户请求生成图片但未指定模型
- **THEN** Agent SHALL 使用 `-m gpt-image-1.5` 参数

### Requirement: Image mask 编辑模型限制

当用户需要使用 mask 进行图片编辑时，Agent MUST 使用 `gpt-image-1.5` 模型，无论用户是否指定了其他模型。

#### Scenario: mask 编辑强制使用 gpt-image-1.5

- **WHEN** 用户请求 mask 编辑并指定了 Gemini 模型
- **THEN** Agent SHALL 告知用户 mask 编辑仅 gpt-image-1.5 支持，并切换到 gpt-image-1.5

### Requirement: Image 生成数量限制

由于 litellm 代理限制，所有图片模型均仅支持 `n=1`。Agent MUST NOT 使用 `--n` 参数或 MUST 固定为 `--n 1`。

当用户要求生成多张图片时，Agent SHALL 告知此限制并建议通过多次执行命令来生成多张图片。

#### Scenario: 用户要求生成多张图片

- **WHEN** 用户请求一次生成多张图片（如"生成 4 张不同风格的猫"）
- **THEN** Agent SHALL 告知 n=1 限制，并通过多次执行 CLI 命令（使用不同提示词）来满足需求

### Requirement: Image CLI 命令构建

Agent SHALL 根据用户意图正确构建 `tapsvc-aigc image generate` 或 `tapsvc-aigc image edit` 命令，包含所有必要参数。

#### Scenario: 构建图片生成命令

- **WHEN** Agent 完成提示词优化
- **THEN** Agent SHALL 构建包含 `-m`、`-p`（或 `--prompt-file`）以及用户指定的可选参数（`--size`、`--quality`、`--background`、`-o`）的完整命令，MUST NOT 使用 `--n` 大于 1

#### Scenario: 构建图片编辑命令

- **WHEN** 用户提供输入图片路径和编辑指令
- **THEN** Agent SHALL 构建包含 `-m`、`--image`、`-p`（或 `--prompt-file`）以及可选的 `--mask`、`--size`、`-o` 参数的完整命令，MUST NOT 使用 `--n` 大于 1
