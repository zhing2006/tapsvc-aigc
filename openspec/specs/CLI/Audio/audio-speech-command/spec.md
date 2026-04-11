## ADDED Requirements

### Requirement: 文本输入解析
`crates/tapsvc-aigc/src/cmd/audio.rs` SHALL 支持从 `--input` 参数或 `--input-file` 文件读取待转换文本，二者互斥，必须且只能提供一个。

#### Scenario: 从参数读取
- **WHEN** 用户指定 `--input "Hello world"`
- **THEN** SHALL 使用该文本作为 API 请求的 `input` 字段

#### Scenario: 从文件读取
- **WHEN** 用户指定 `--input-file article.txt`
- **THEN** SHALL 读取文件内容作为 API 请求的 `input` 字段

#### Scenario: 两者都未提供
- **WHEN** 用户既未指定 `--input` 也未指定 `--input-file`
- **THEN** SHALL 报错退出，提示 "either --input or --input-file must be provided"

#### Scenario: 两者同时提供
- **WHEN** 用户同时指定 `--input` 和 `--input-file`
- **THEN** SHALL 报错退出，提示 "--input and --input-file are mutually exclusive"

### Requirement: API 调用与文件写入
`crates/tapsvc-aigc/src/cmd/audio.rs` SHALL 调用 `OpenAiClient::speech()` 并将返回的二进制数据写入输出文件。

#### Scenario: 指定输出路径
- **WHEN** 用户指定 `--output speech.mp3`
- **THEN** SHALL 将音频数据写入 `speech.mp3`，并在 stdout 打印文件路径

#### Scenario: 自动命名
- **WHEN** 用户未指定 `--output`
- **THEN** SHALL 使用 `speech_{timestamp}.{format}` 格式自动生成文件名，timestamp 格式为 `YYYYMMDD_HHMMSS`

#### Scenario: 创建父目录
- **WHEN** 输出路径的父目录不存在
- **THEN** SHALL 自动创建父目录

### Requirement: format 参数映射
CLI 的 `--format` 参数 SHALL 映射为 API 请求的 `response_format` 字段。

#### Scenario: 格式传递
- **WHEN** 用户指定 `--format opus`
- **THEN** SHALL 在请求体中设置 `"response_format": "opus"`

#### Scenario: 默认格式
- **WHEN** 用户未指定 `--format`
- **THEN** SHALL 默认使用 `mp3`
