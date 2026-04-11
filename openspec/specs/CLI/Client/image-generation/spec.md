## ADDED Requirements

### Requirement: image 模块导出
`tapsvc-aigc-openai` crate SHALL 在 `lib.rs` 中声明并导出 `image` 模块，公开所有请求/响应类型。

#### Scenario: 模块可访问
- **WHEN** 外部 crate 引用 `tapsvc_aigc_openai::image`
- **THEN** SHALL 能访问 `CreateImageRequest`、`ImageResponse`、`ImageData` 类型

### Requirement: CreateImageRequest 类型定义
SHALL 定义 `CreateImageRequest` 结构体，使用 `serde::Serialize` 派生。

#### Scenario: 必填字段
- **WHEN** 构建 `CreateImageRequest`
- **THEN** `model` 和 `prompt` 字段 SHALL 为 `String` 类型，必须提供

#### Scenario: 可选字段
- **WHEN** 构建 `CreateImageRequest`
- **THEN** SHALL 包含以下 `Option` 字段：`n: Option<u32>`、`size: Option<String>`、`quality: Option<String>`、`response_format: Option<String>`、`background: Option<String>`、`output_format: Option<String>`
- **AND** 可选字段序列化时 SHALL 跳过 `None` 值（`#[serde(skip_serializing_if = "Option::is_none")]`）

### Requirement: ImageResponse 类型定义
SHALL 定义 `ImageResponse` 结构体，使用 `serde::Deserialize` 派生。

#### Scenario: 响应结构
- **WHEN** 解析 API JSON 响应
- **THEN** SHALL 包含 `created: u64` 和 `data: Vec<ImageData>` 字段

### Requirement: ImageData 类型定义
SHALL 定义 `ImageData` 结构体，使用 `serde::Deserialize` 派生。

#### Scenario: 数据结构
- **WHEN** 解析 `data` 数组中的元素
- **THEN** SHALL 包含 `b64_json: Option<String>` 和 `revised_prompt: Option<String>` 字段

### Requirement: OpenAiClient create_image 方法
`OpenAiClient` SHALL 提供异步方法 `create_image`，向 API 发送图片生成请求，内部集成 retry。

#### Scenario: 成功调用
- **WHEN** 调用 `client.create_image(&request)` 且 API 返回 200
- **THEN** SHALL 返回 `Ok(ImageResponse)`，包含解析后的响应数据

#### Scenario: 请求构建
- **WHEN** 发送请求
- **THEN** SHALL POST 到 `{base_url}/v1/images/generations`
- **AND** SHALL 设置 `Authorization: Bearer {api_key}` header
- **AND** SHALL 设置 `Content-Type: application/json`

#### Scenario: 可重试错误自动重试
- **WHEN** API 返回 429、500、502、503、504 或发生网络错误
- **THEN** SHALL 通过 core retry 执行器自动重试，按指数退避（2s/4s/8s + jitter）等待

#### Scenario: 不可重试 API 错误
- **WHEN** API 返回 400、401、403、422 等非可重试状态码
- **THEN** SHALL 立即返回 `Err(Error::Api { status, message, retry_after: None })`，不进行重试

### Requirement: Error::Api 包含 retry_after 字段
`Error::Api` 变体 SHALL 包含 `retry_after: Option<Duration>` 字段，用于支持 Retryable trait。

#### Scenario: 响应含 Retry-After header
- **WHEN** API 返回非 2xx 状态码且响应包含 `Retry-After` header（秒数）
- **THEN** SHALL 解析该 header 值为 `Duration`，存入 `Error::Api { retry_after: Some(duration), .. }`

#### Scenario: 响应无 Retry-After header
- **WHEN** API 返回非 2xx 状态码且响应不包含 `Retry-After` header
- **THEN** SHALL 设置 `Error::Api { retry_after: None, .. }`

### Requirement: Error 类型实现 Retryable trait
`tapsvc-aigc-openai` 的 `Error` 枚举 SHALL 实现 `tapsvc_aigc_core::Retryable` trait。

#### Scenario: 网络错误可重试
- **WHEN** 错误为 `Error::Request`（reqwest 网络错误）
- **THEN** `is_retryable()` SHALL 返回 `true`

#### Scenario: API 错误按状态码判断
- **WHEN** 错误为 `Error::Api { status, .. }`
- **THEN** `is_retryable()` SHALL 根据 `status` 调用 core 的可重试状态码判断
- **AND** `retry_after()` SHALL 返回 `Error::Api` 中的 `retry_after` 字段值

#### Scenario: 反序列化错误不可重试
- **WHEN** 错误为 `Error::Deserialize`
- **THEN** `is_retryable()` SHALL 返回 `false`
