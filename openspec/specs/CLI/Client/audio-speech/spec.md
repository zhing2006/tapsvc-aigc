## ADDED Requirements

### Requirement: SpeechRequest 类型定义
`crates/tapsvc-aigc-openai` crate SHALL 定义 `SpeechRequest` 结构体，包含 `/v1/audio/speech` 端点所需的请求字段。

#### Scenario: 必填字段
- **WHEN** 构造 `SpeechRequest`
- **THEN** SHALL 包含 `model: String`、`input: String`、`voice: String` 三个必填字段

#### Scenario: 可选字段
- **WHEN** 构造 `SpeechRequest`
- **THEN** SHALL 包含可选字段 `response_format: Option<String>`（默认 `mp3`）和 `speed: Option<f32>`（默认 `1.0`）

#### Scenario: JSON 序列化
- **WHEN** 序列化 `SpeechRequest`
- **THEN** SHALL 使用 `serde::Serialize` 派生宏，`None` 值的可选字段 SHALL 跳过序列化（`skip_serializing_if`）

### Requirement: speech 方法
`OpenAiClient` SHALL 提供 `speech()` 异步方法，发送 TTS 请求并返回二进制音频数据。

#### Scenario: 请求发送
- **WHEN** 调用 `client.speech(&req)` 
- **THEN** SHALL POST JSON 请求体到 `{base_url}/v1/audio/speech`，携带 `Authorization: Bearer {api_key}` header

#### Scenario: 成功响应
- **WHEN** API 返回 2xx 状态码
- **THEN** SHALL 以 `bytes()` 方式读取完整响应 body，返回 `Vec<u8>`

#### Scenario: 错误响应
- **WHEN** API 返回非 2xx 状态码
- **THEN** SHALL 返回 `Error::Api`，包含 status code、error message 和可选的 `Retry-After` header 值

#### Scenario: 自动重试
- **WHEN** 遇到网络错误、429 或 5xx 错误
- **THEN** SHALL 通过 `retry()` 执行器自动重试，遵循 `RetryConfig` 策略

### Requirement: audio 模块导出
`crates/tapsvc-aigc-openai` crate SHALL 导出 `audio` 模块。

#### Scenario: lib.rs 导出
- **WHEN** 检查 `crates/tapsvc-aigc-openai/src/lib.rs`
- **THEN** SHALL 包含 `pub mod audio;` 声明
