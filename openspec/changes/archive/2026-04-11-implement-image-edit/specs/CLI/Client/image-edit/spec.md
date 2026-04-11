## ADDED Requirements

### Requirement: EditImageRequest 类型定义
SHALL 定义 `EditImageRequest` 结构体，包含 image edit API 所需的全部参数。该结构体不需要 `Serialize` derive（multipart 请求不走 JSON 序列化）。

#### Scenario: 必填字段
- **WHEN** 构建 `EditImageRequest`
- **THEN** SHALL 包含以下必填字段：`model: String`、`prompt: String`、`image_bytes: Vec<u8>`、`image_filename: String`

#### Scenario: 可选字段
- **WHEN** 构建 `EditImageRequest`
- **THEN** SHALL 包含以下可选字段：`mask_bytes: Option<Vec<u8>>`、`mask_filename: Option<String>`、`n: Option<u32>`、`size: Option<String>`、`output_format: Option<String>`

### Requirement: OpenAiClient edit_image 方法
`OpenAiClient` SHALL 提供异步方法 `edit_image`，通过 multipart/form-data 向 API 发送图片编辑请求，内部集成 retry。

#### Scenario: 成功调用
- **WHEN** 调用 `client.edit_image(&request)` 且 API 返回 200
- **THEN** SHALL 返回 `Ok(ImageResponse)`，复用 image generate 的响应类型

#### Scenario: 请求构建
- **WHEN** 发送请求
- **THEN** SHALL POST 到 `{base_url}/v1/images/edits`
- **AND** SHALL 设置 `Authorization: Bearer {api_key}` header
- **AND** SHALL 构建 multipart/form-data 请求体

#### Scenario: multipart 字段构建
- **WHEN** 构建 multipart form
- **THEN** SHALL 包含 `model`（text）、`prompt`（text）、`image`（file part，使用原始文件名和对应的 MIME type）、`response_format`（text，固定 `"b64_json"`）
- **AND** 如果 `mask_bytes` 存在，SHALL 添加 `mask` file part
- **AND** 如果 `n` 存在，SHALL 添加 `n` text part
- **AND** 如果 `size` 存在，SHALL 添加 `size` text part
- **AND** 如果 `output_format` 存在，SHALL 添加 `output_format` text part

#### Scenario: 可重试错误自动重试
- **WHEN** API 返回 429、500、502、503、504 或发生网络错误
- **THEN** SHALL 通过 core retry 执行器自动重试
- **AND** 每次 retry SHALL 重新构建 `multipart::Form`（Form 不可 clone）

#### Scenario: 不可重试 API 错误
- **WHEN** API 返回 400、401、403、422 等非可重试状态码
- **THEN** SHALL 立即返回 `Err(Error::Api { status, message, retry_after })`，不进行重试

### Requirement: image 模块导出 EditImageRequest
`tapsvc-aigc-openai` crate 的 `image` 模块 SHALL 导出 `EditImageRequest` 类型。

#### Scenario: 模块可访问
- **WHEN** 外部 crate 引用 `tapsvc_aigc_openai::image`
- **THEN** SHALL 能访问 `EditImageRequest` 类型
