## ADDED Requirements

### Requirement: video 模块导出
`tapsvc-aigc-ark` crate SHALL 在 `lib.rs` 中声明并导出 `video` 模块，公开所有请求/响应类型。

#### Scenario: 模块可访问
- **WHEN** 外部 crate 引用 `tapsvc_aigc_ark::video`
- **THEN** SHALL 能访问 `CreateVideoTaskRequest`、`VideoTask`、`VideoTaskList`、`ContentItem`、`VideoTaskError`、`VideoContent` 等类型

### Requirement: ContentItem 类型定义
SHALL 定义 `ContentItem` 枚举，使用 `#[serde(tag = "type")]` 内部标签 + `#[serde(rename)]` 属性派生 `serde::Serialize`，支持 API 的多态 content 数组。各变体通过辅助结构体（如 `ImageUrlData { url }`）生成嵌套 JSON 形状。

#### Scenario: Text 变体
- **WHEN** 序列化 `ContentItem::Text { text }`
- **THEN** SHALL 输出 `{"type": "text", "text": "..."}`

#### Scenario: ImageUrl 变体
- **WHEN** 序列化 `ContentItem::ImageUrl { url, role }`
- **THEN** SHALL 输出 `{"type": "image_url", "image_url": {"url": "..."}, "role": "..."}`
- **AND** `role` 值 SHALL 为 `"first_frame"`、`"last_frame"` 或 `"reference_image"` 之一

#### Scenario: VideoUrl 变体
- **WHEN** 序列化 `ContentItem::VideoUrl { url, role }`
- **THEN** SHALL 输出 `{"type": "video_url", "video_url": {"url": "..."}, "role": "..."}`
- **AND** `role` 值 SHALL 为 `"reference_video"`

#### Scenario: AudioUrl 变体
- **WHEN** 序列化 `ContentItem::AudioUrl { url, role }`
- **THEN** SHALL 输出 `{"type": "audio_url", "audio_url": {"url": "..."}, "role": "..."}`
- **AND** `role` 值 SHALL 为 `"reference_audio"`

### Requirement: CreateVideoTaskRequest 类型定义
SHALL 定义 `CreateVideoTaskRequest` 结构体，使用 `serde::Serialize` 派生。

#### Scenario: 必填字段
- **WHEN** 构建 `CreateVideoTaskRequest`
- **THEN** `model` SHALL 为 `String` 类型，`content` SHALL 为 `Vec<ContentItem>` 类型，两者必须提供

#### Scenario: 可选字段
- **WHEN** 构建 `CreateVideoTaskRequest`
- **THEN** SHALL 包含以下 `Option` 字段：`resolution`、`ratio`、`duration`、`generate_audio`、`watermark`、`camera_fixed`、`seed`、`tools`
- **AND** 可选字段序列化时 SHALL 跳过 `None` 值

### Requirement: VideoTaskTool 类型定义
SHALL 定义 `VideoTaskTool` 结构体用于表示 tools 数组中的元素。

#### Scenario: web_search tool
- **WHEN** 序列化 `VideoTaskTool { type_: "web_search" }`
- **THEN** SHALL 输出 `{"type": "web_search"}`
- **AND** 字段名 `type_` SHALL 序列化为 `"type"`（使用 `#[serde(rename)]`）

### Requirement: VideoTask 类型定义
SHALL 定义 `VideoTask` 结构体，使用 `serde::Deserialize` 派生，表示 API 返回的任务对象。

#### Scenario: 核心字段
- **WHEN** 解析 API JSON 响应
- **THEN** SHALL 包含 `id: String`、`model: String`、`status: String` 字段

#### Scenario: 可选字段
- **WHEN** 解析 API JSON 响应
- **THEN** SHALL 包含以下 `Option` 或带默认值的字段：`content: Option<VideoContent>`、`error: Option<VideoTaskError>`、`created_at: Option<u64>`、`updated_at: Option<u64>`、`duration: Option<i32>`、`ratio: Option<String>`、`resolution: Option<String>`、`seed: Option<u64>`、`revised_prompt: Option<String>`、`generate_audio: Option<bool>`

### Requirement: VideoContent 类型定义
SHALL 定义 `VideoContent` 结构体，使用 `serde::Deserialize` 派生。

#### Scenario: 字段结构
- **WHEN** 解析任务结果中的 content 对象
- **THEN** SHALL 包含 `video_url: Option<String>` 和 `last_frame_url: Option<String>` 字段

### Requirement: VideoTaskError 类型定义
SHALL 定义 `VideoTaskError` 结构体，使用 `serde::Deserialize` 派生。

#### Scenario: 错误信息
- **WHEN** 解析任务失败时的 error 对象
- **THEN** SHALL 包含 `code: String` 和 `message: String` 字段

### Requirement: VideoTaskId 类型定义
SHALL 定义 `VideoTaskId` 结构体，使用 `serde::Deserialize` 派生，表示 create 端点的响应。

#### Scenario: 响应结构
- **WHEN** 解析 create task 的 API 响应
- **THEN** SHALL 包含 `id: String` 字段

### Requirement: VideoTaskList 类型定义
SHALL 定义 `VideoTaskList` 结构体，使用 `serde::Deserialize` 派生，表示 list 端点的响应。

#### Scenario: 分页结构
- **WHEN** 解析 list tasks 的 API 响应
- **THEN** SHALL 包含 `total: u32` 和 `items: Vec<VideoTask>` 字段

### Requirement: ListVideoTasksFilter 类型定义
SHALL 定义 `ListVideoTasksFilter` 结构体，作为 `list_video_tasks` 方法的参数，封装所有过滤和分页条件。

#### Scenario: 字段定义
- **WHEN** 构建 `ListVideoTasksFilter`
- **THEN** SHALL 包含以下字段：`page_num: Option<u32>`、`page_size: Option<u32>`、`status: Option<String>`、`model: Option<String>`、`task_ids: Option<Vec<String>>`
- **AND** 所有字段 SHALL 为 `Option` 类型，未设置时不发送对应查询参数

#### Scenario: 默认值
- **WHEN** 使用 `ListVideoTasksFilter::default()` 或 `ListVideoTasksFilter` 所有字段为 `None`
- **THEN** SHALL 不附加任何过滤查询参数，使用 API 服务端默认值

### Requirement: ArkClient create_video_task 方法
`ArkClient` SHALL 提供异步方法 `create_video_task`，向 API 提交视频生成任务，内部集成 retry。

#### Scenario: 成功提交
- **WHEN** 调用 `client.create_video_task(&request)` 且 API 返回 200
- **THEN** SHALL 返回 `Ok(VideoTaskId)`

#### Scenario: 请求构建
- **WHEN** 发送请求
- **THEN** SHALL POST 到 `{base_url}/volcengine/api/v3/contents/generations/tasks`
- **AND** SHALL 设置 `Authorization: Bearer {api_key}` header
- **AND** SHALL 设置 `Content-Type: application/json`

#### Scenario: 可重试错误自动重试
- **WHEN** API 返回 429、500、502、503、504 或发生网络错误
- **THEN** SHALL 通过 core retry 执行器自动重试

#### Scenario: 不可重试 API 错误
- **WHEN** API 返回 400、401、403 等非可重试状态码
- **THEN** SHALL 立即返回错误，不进行重试

### Requirement: ArkClient get_video_task 方法
`ArkClient` SHALL 提供异步方法 `get_video_task`，查询单个任务状态，内部集成 retry。

#### Scenario: 成功查询
- **WHEN** 调用 `client.get_video_task(task_id)` 且 API 返回 200
- **THEN** SHALL 返回 `Ok(VideoTask)`

#### Scenario: 请求构建
- **WHEN** 发送请求
- **THEN** SHALL GET `{base_url}/volcengine/api/v3/contents/generations/tasks/{task_id}`
- **AND** SHALL 设置 `Authorization: Bearer {api_key}` header

### Requirement: ArkClient list_video_tasks 方法
`ArkClient` SHALL 提供异步方法 `list_video_tasks`，列出任务并支持过滤和分页，内部集成 retry。

#### Scenario: 成功列出
- **WHEN** 调用 `client.list_video_tasks(filter)` 且 API 返回 200
- **THEN** SHALL 返回 `Ok(VideoTaskList)`

#### Scenario: 查询参数构建
- **WHEN** 发送请求
- **THEN** SHALL GET `{base_url}/volcengine/api/v3/contents/generations/tasks`
- **AND** SHALL 将 `page_num`、`page_size` 作为查询参数
- **AND** SHALL 将 `status` 映射为 `filter.status`、`model` 映射为 `filter.model` 查询参数
- **AND** SHALL 将 `task_ids` 数组中每个 ID 作为独立的 `filter.task_ids` 查询参数

### Requirement: ArkClient delete_video_task 方法
`ArkClient` SHALL 提供异步方法 `delete_video_task`，删除指定任务。

#### Scenario: 成功删除
- **WHEN** 调用 `client.delete_video_task(task_id)` 且 API 返回 200
- **THEN** SHALL 返回 `Ok(())`

#### Scenario: 请求构建
- **WHEN** 发送请求
- **THEN** SHALL DELETE `{base_url}/volcengine/api/v3/contents/generations/tasks/{task_id}`
- **AND** SHALL 设置 `Authorization: Bearer {api_key}` header

### Requirement: ARK Error 类型定义
`tapsvc-aigc-ark` 的 `Error` 枚举 SHALL 包含 `retry_after: Option<Duration>` 的 `Api` 变体。

#### Scenario: 错误变体
- **WHEN** 定义 `Error` 枚举
- **THEN** SHALL 包含 `Request`（reqwest 错误）、`Api { status, message, retry_after }`、`Deserialize`（serde 错误）三个变体
- **AND** SHALL 不包含 `Timeout` 变体（超时是 cmd 层的职责，不属于 client 层）

### Requirement: ARK Error 实现 Retryable trait
`tapsvc-aigc-ark` 的 `Error` 枚举 SHALL 实现 `tapsvc_aigc_core::Retryable` trait。

#### Scenario: 网络错误可重试
- **WHEN** 错误为 `Error::Request`
- **THEN** `is_retryable()` SHALL 返回 `true`

#### Scenario: API 错误按状态码判断
- **WHEN** 错误为 `Error::Api { status, .. }`
- **THEN** `is_retryable()` SHALL 对 429、500、502、503、504 返回 `true`
- **AND** `retry_after()` SHALL 返回 `Error::Api` 中的 `retry_after` 字段值

#### Scenario: 反序列化错误不可重试
- **WHEN** 错误为 `Error::Deserialize`
- **THEN** `is_retryable()` SHALL 返回 `false`

### Requirement: ArkClient 集成 RetryConfig
`ArkClient` SHALL 持有 `RetryConfig` 字段，所有 API 方法 SHALL 使用 `tapsvc_aigc_core::retry` 执行器进行重试。

#### Scenario: 默认配置
- **WHEN** 使用 `ArkClient::new()` 创建 client
- **THEN** SHALL 使用 `RetryConfig::default()` 初始化 retry 配置
