## ADDED Requirements

### Requirement: ImageCommand Edit variant
`ImageCommand` enum SHALL 新增 `Edit` variant，定义 image edit 子命令的 CLI 参数。

#### Scenario: 必填参数
- **WHEN** 用户执行 `tapsvc-aigc image edit`
- **THEN** SHALL 要求 `--model`（`-m`）和 `--image` 参数

#### Scenario: prompt 参数
- **WHEN** 用户执行 `image edit`
- **THEN** SHALL 支持 `--prompt`（`-p`）和 `--prompt-file` 参数，至少提供其一

#### Scenario: 可选参数及默认值
- **WHEN** 用户执行 `image edit`
- **THEN** SHALL 支持以下可选参数：`--mask`（无默认值）、`--size`（默认 `1024x1024`）、`--n`（默认 1，范围 1-10）、`--response-format`（默认 `png`）、`--output`（`-o`，默认自动命名）

### Requirement: 文件格式校验
CLI SHALL 在发送请求前对输入文件的格式进行校验。

#### Scenario: image 格式校验
- **WHEN** 用户指定 `--image photo.bmp`（扩展名非 PNG/JPEG/WebP）
- **THEN** SHALL 报错退出，提示仅支持 PNG/JPEG/WebP 格式

#### Scenario: image 合法格式
- **WHEN** 用户指定 `--image photo.png` 或 `photo.jpg` 或 `photo.jpeg` 或 `photo.webp`
- **THEN** SHALL 接受该文件

#### Scenario: mask 格式校验
- **WHEN** 用户指定 `--mask mask.jpg`（非 PNG）
- **THEN** SHALL 报错退出，提示 mask 仅支持 PNG 格式

### Requirement: 文件大小校验
CLI SHALL 在发送请求前对输入文件的大小进行校验。

#### Scenario: image 大小校验
- **WHEN** 用户指定的 `--image` 文件大于 25MB
- **THEN** SHALL 报错退出，提示文件超过 25MB 限制

#### Scenario: mask 大小校验
- **WHEN** 用户指定的 `--mask` 文件大于 4MB
- **THEN** SHALL 报错退出，提示文件超过 4MB 限制

### Requirement: 复用 image generate 辅助函数
`image edit` 的 CLI 处理逻辑 SHALL 复用已有的辅助函数。

#### Scenario: prompt 合并
- **WHEN** 用户同时指定 `--prompt` 和 `--prompt-file`
- **THEN** SHALL 使用 `resolve_prompt()` 合并（file 内容在前，prompt 在后）

#### Scenario: 输出路径构建
- **WHEN** 需要确定输出文件路径
- **THEN** SHALL 使用 `build_output_paths()` 按现有规则生成路径（自动命名前缀为 `edited_` 替代 `image_`）

#### Scenario: 格式转扩展名
- **WHEN** 需要确定输出文件扩展名
- **THEN** SHALL 使用 `format_to_ext()` 进行映射

### Requirement: base64 解码与文件写入
`image edit` 的文件写入逻辑 SHALL 与 image generate 一致。

#### Scenario: 正常写入
- **WHEN** API 返回包含 `b64_json` 的响应
- **THEN** SHALL 对 `b64_json` 进行 base64 解码，将二进制数据写入输出文件，并在 stdout 打印文件路径

#### Scenario: 全部数据缺失
- **WHEN** API 返回 200 但所有 `ImageData` 的 `b64_json` 均为 `None`
- **THEN** SHALL 报错退出（非零 exit code）

### Requirement: edit_image 调用
CLI SHALL 从环境变量构造 `OpenAiClient`，读取文件字节后调用 `edit_image` 方法。

#### Scenario: 请求构建
- **WHEN** 用户执行 `image edit --model gpt-image-1.5 --image input.png --prompt "add a hat" --response-format jpeg`
- **THEN** SHALL 读取 `input.png` 文件内容为 `Vec<u8>`，构建 `EditImageRequest`
- **AND** `output_format` SHALL 设置为 `--response-format` 的值（如 `"jpeg"`）
