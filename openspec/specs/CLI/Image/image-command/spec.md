## ADDED Requirements

### Requirement: prompt 合并逻辑
CLI SHALL 支持 `--prompt` 和 `--prompt-file` 同时提供，拼接为最终 prompt。

#### Scenario: 仅提供 prompt
- **WHEN** 用户仅指定 `--prompt "a cat"`
- **THEN** SHALL 使用 `"a cat"` 作为最终 prompt

#### Scenario: 仅提供 prompt_file
- **WHEN** 用户仅指定 `--prompt-file style.txt`（文件内容为 `"cyberpunk style"`）
- **THEN** SHALL 读取文件内容 `"cyberpunk style"` 作为最终 prompt

#### Scenario: 两者同时提供
- **WHEN** 用户同时指定 `--prompt-file style.txt`（内容 `"cyberpunk style"`）和 `--prompt "a cat"`
- **THEN** SHALL 拼接为 `"cyberpunk style\na cat"`（file 内容在前，prompt 在后，以换行分隔）

#### Scenario: 两者都未提供
- **WHEN** 用户既未指定 `--prompt` 也未指定 `--prompt-file`
- **THEN** SHALL 报错退出，提示至少需要提供其一

#### Scenario: prompt_file 不存在
- **WHEN** 用户指定的 `--prompt-file` 路径不存在
- **THEN** SHALL 报错退出，提示文件未找到

### Requirement: --n 参数范围约束
CLI SHALL 在 clap 层对 `--n` 参数约束为 1-10。

#### Scenario: 合法值
- **WHEN** 用户指定 `--n 5`
- **THEN** SHALL 接受该值

#### Scenario: 越界值
- **WHEN** 用户指定 `--n 0` 或 `--n 11`
- **THEN** clap SHALL 在参数解析阶段报错退出，无需到达业务逻辑

### Requirement: 自动命名规则
CLI SHALL 在未指定 `-o` 时自动生成输出文件名，指定 `-o` 且 n>1 时自动编号。

#### Scenario: 无 -o 且 n=1
- **WHEN** 用户未指定 `-o` 且 `--n 1`，`--response-format png`
- **THEN** SHALL 输出到当前目录，文件名格式为 `image_YYYYMMDD_HHMMSS.png`

#### Scenario: 无 -o 且 n>1
- **WHEN** 用户未指定 `-o` 且 `--n 3`，`--response-format png`
- **THEN** SHALL 输出 `image_YYYYMMDD_HHMMSS_1.png`、`image_YYYYMMDD_HHMMSS_2.png`、`image_YYYYMMDD_HHMMSS_3.png`

#### Scenario: 有 -o 且 n=1
- **WHEN** 用户指定 `-o cat.png` 且 `--n 1`
- **THEN** SHALL 输出到 `cat.png`

#### Scenario: 有 -o 且 n>1
- **WHEN** 用户指定 `-o cat.png` 且 `--n 3`
- **THEN** SHALL 输出 `cat_1.png`、`cat_2.png`、`cat_3.png`

### Requirement: base64 解码与文件写入
CLI SHALL 将 API 返回的 base64 图片数据解码后写入文件。

#### Scenario: 单图写入
- **WHEN** API 返回包含一个 `b64_json` 的响应
- **THEN** SHALL 对 `b64_json` 进行 base64 解码，将二进制数据写入输出文件

#### Scenario: 多图写入
- **WHEN** API 返回包含多个 `b64_json` 的响应（n>1）
- **THEN** SHALL 依次解码每个 `b64_json`，按编号规则写入对应文件

#### Scenario: 单个 b64_json 缺失
- **WHEN** API 响应的某个 `ImageData` 中 `b64_json` 为 `None`
- **THEN** SHALL 跳过该项并输出警告信息到 stderr

#### Scenario: 全部 b64_json 缺失
- **WHEN** API 返回 200 但所有 `ImageData` 的 `b64_json` 均为 `None`
- **THEN** SHALL 报错退出（非零 exit code），提示无有效图片数据

### Requirement: OpenAiClient 构造与调用
CLI SHALL 从环境变量构造 `OpenAiClient` 并调用 `create_image` 方法。

#### Scenario: 客户端构造
- **WHEN** 执行 image generate 命令
- **THEN** SHALL 从 `TAPSVC_BASE_URL` 和 `TAPSVC_API_KEY` 环境变量构造 `OpenAiClient`

#### Scenario: 请求构建
- **WHEN** 用户指定 `--model gpt-image-1.5 --prompt "a cat" --size 1024x1024 --n 2 --quality high --response-format jpeg --background auto`
- **THEN** SHALL 构建 `CreateImageRequest`，所有参数正确映射到请求字段
- **AND** `output_format` SHALL 设置为 `--response-format` 的值（如 `"jpeg"`）
- **AND** `response_format` SHALL 固定为 `"b64_json"`

### Requirement: 输出信息
CLI SHALL 在生成成功后输出文件路径到 stdout。

#### Scenario: 成功输出
- **WHEN** 图片生成并保存成功
- **THEN** SHALL 在 stdout 打印每个输出文件的路径（每行一个）

#### Scenario: revised_prompt 信息
- **WHEN** API 返回了 `revised_prompt`
- **THEN** SHALL 在 stderr 输出 revised prompt 信息
