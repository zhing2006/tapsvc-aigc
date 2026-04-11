## MODIFIED Requirements

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
