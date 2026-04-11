## ADDED Requirements

### Requirement: Video Skill reference 文件

Skill SHALL 在 `references/video-generation.md` 中提供视频生成的完整参考文档，包含以下内容：

1. 支持模型对比表（doubao-seedance-2-0-fast-260128 DEFAULT、doubao-seedance-2-0-260128），列出两个模型的差异（速度 vs 质量）
2. 提示词结构最佳实践（主体 + 动作 + 场景 + 镜头 + 风格）
3. 镜头语言词汇表（推/拉/摇/移/跟/环绕/升降/变焦）
4. 风格关键字参考（动画、写实、漫画、黏土、体素等）
5. 角色一致性技巧（面部特征、服装、发型的显式描述）
6. 多动作序列描述技巧（按时间线排列动作）
7. 各参数说明（resolution、aspect_ratio、duration、camera_fixed 等）

#### Scenario: Agent 加载 video reference

- **WHEN** 用户请求生成视频
- **THEN** Agent SHALL 读取 `references/video-generation.md` 获取提示词结构和镜头语言参考

### Requirement: Video 提示词优化

Agent SHALL 在执行视频生成命令之前，根据 `references/video-generation.md` 中的最佳实践优化用户提示词。

优化内容包括：
- 将用户描述结构化为"主体 + 动作 + 场景 + 镜头 + 风格"格式
- 补充角色外貌、服装、姿态的具体描述
- 添加适当的镜头运动描述（如"镜头缓慢推进"、"环绕拍摄"）
- 对于多动作场景，按时间线顺序排列

#### Scenario: 优化视频生成提示词

- **WHEN** 用户提供简短的视频描述（如"一个人在沙滩上跑步"）
- **THEN** Agent SHALL 扩展为包含角色细节、动作描述、场景氛围、镜头运动的结构化提示词

#### Scenario: 多动作视频提示词

- **WHEN** 用户描述包含多个连续动作的场景
- **THEN** Agent SHALL 按时间线顺序排列各动作，确保描述清晰

### Requirement: Video 默认模型

当用户未指定模型时，视频生成 SHALL 默认使用 `doubao-seedance-2-0-fast-260128`。

#### Scenario: 使用默认视频模型

- **WHEN** 用户请求生成视频但未指定模型
- **THEN** Agent SHALL 使用 `-m doubao-seedance-2-0-fast-260128` 参数

#### Scenario: 用户选择高质量模型

- **WHEN** 用户明确要求高质量或指定 standard 模型
- **THEN** Agent SHALL 使用 `-m doubao-seedance-2-0-260128` 参数

### Requirement: Video 仅支持 generate 和 get 命令

Skill SHALL 仅覆盖 `video generate` 和 `video get` 两个命令，MUST NOT 引导用户使用 `video list` 或 `video delete`。

#### Scenario: Skill 不引导 video list

- **WHEN** 用户通过 Skill 使用视频功能
- **THEN** Agent SHALL NOT 主动建议或使用 `tapsvc-aigc video list` 命令

#### Scenario: Skill 不引导 video delete

- **WHEN** 用户通过 Skill 使用视频功能
- **THEN** Agent SHALL NOT 主动建议或使用 `tapsvc-aigc video delete` 命令

### Requirement: Video 参数约束前置校验

Agent SHALL 在构建 `video generate` 命令之前，校验以下参数约束规则，违反时 MUST 中止并提示用户，而非让 CLI 报错：

| 约束 | 规则 |
|------|------|
| `--last-frame` | REQUIRES `--first-frame` |
| `--ref-audio` | REQUIRES `--ref-image` OR `--ref-video` |
| `--first-frame` | EXCLUSIVE WITH `--ref-image` 和 `--ref-video` |
| `--duration` | 仅接受 4-15 或 -1（auto） |
| `--ref-image` | 最多 9 个 |
| `--ref-video` | 最多 3 个，仅接受 URL |
| `--ref-audio` | 最多 3 个 |
| 输入 | 至少需要 `--prompt`/`--prompt-file`、`--first-frame`、`--ref-image` 或 `--ref-video` 之一 |

#### Scenario: --last-frame 缺少 --first-frame

- **WHEN** 用户提供了尾帧图片但未提供首帧图片
- **THEN** Agent SHALL 提示用户需要同时提供 `--first-frame`

#### Scenario: --first-frame 与 --ref-image 互斥

- **WHEN** 用户同时提供了首帧图片和参考图片
- **THEN** Agent SHALL 告知用户两者互斥，询问使用哪种模式

#### Scenario: --duration 超出范围

- **WHEN** 用户指定的时长不在 4-15 范围内且不为 -1
- **THEN** Agent SHALL 提示有效范围为 4-15 秒或 -1（自动）

#### Scenario: --ref-video 使用本地文件

- **WHEN** 用户提供本地文件路径作为参考视频
- **THEN** Agent SHALL 告知 `--ref-video` 仅接受 URL，建议先上传视频获取 URL

### Requirement: Video CLI 命令构建

Agent SHALL 根据用户意图正确构建 `tapsvc-aigc video generate` 命令，通过参数约束前置校验后包含所有必要参数。

#### Scenario: 构建文本生视频命令

- **WHEN** 用户提供文本描述生成视频且通过参数校验
- **THEN** Agent SHALL 构建包含 `-m`、`-p`（或 `--prompt-file`）以及可选参数（`--resolution`、`--aspect-ratio`、`--duration`、`--seed`、`-o`）的完整命令

#### Scenario: 构建图生视频命令

- **WHEN** 用户提供首帧图片生成视频且通过参数校验
- **THEN** Agent SHALL 构建包含 `--first-frame`、可选 `--last-frame`、`-p` 以及其他参数的完整命令

#### Scenario: 查询视频生成任务状态

- **WHEN** 视频生成超时或用户要求查询任务状态
- **THEN** Agent SHALL 使用 `tapsvc-aigc video get <task_id>` 查询
