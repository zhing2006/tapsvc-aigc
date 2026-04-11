## 1. ARK Client 类型定义与 Error 处理

- [x] 1.1 在 `tapsvc-aigc-ark/src/video.rs` 中定义请求/响应类型：`ContentItem`（enum，使用 `#[serde(tag = "type")]` 内部标签 + `#[serde(rename)]` 派生 Serialize，含 Text/ImageUrl/VideoUrl/AudioUrl 变体，各变体通过辅助结构体如 `ImageUrlData { url }` 生成嵌套 JSON）、`VideoTaskTool`、`CreateVideoTaskRequest`、`VideoTaskId`、`VideoTask`、`VideoContent`、`VideoTaskError`、`VideoTaskList`、`ListVideoTasksFilter`（含 `page_num`、`page_size`、`status`、`model`、`task_ids` 字段，全部 `Option`）
- [x] 1.2 更新 `tapsvc-aigc-ark/src/error.rs`：删除现有的 `Timeout { seconds }` 变体（超时是 cmd 层职责），为 `Error::Api` 变体添加 `retry_after: Option<Duration>` 字段，实现 `tapsvc_aigc_core::Retryable` trait
- [x] 1.3 更新 `tapsvc-aigc-ark/src/lib.rs`：导出 `video` 模块
- [x] 1.4 更新 `tapsvc-aigc-ark/Cargo.toml`：添加 `tapsvc-aigc-core` 依赖

## 2. ARK Client API 方法

- [x] 2.1 更新 `tapsvc-aigc-ark/src/client.rs`：添加 `RetryConfig` 字段，在 `new()` 中初始化
- [x] 2.2 实现 `ArkClient::create_video_task(&self, req: &CreateVideoTaskRequest) -> Result<VideoTaskId, Error>`：POST 到 `/volcengine/api/v3/contents/generations/tasks`，集成 retry
- [x] 2.3 实现 `ArkClient::get_video_task(&self, task_id: &str) -> Result<VideoTask, Error>`：GET `/volcengine/api/v3/contents/generations/tasks/{task_id}`，集成 retry
- [x] 2.4 实现 `ArkClient::list_video_tasks(&self, filter: &ListVideoTasksFilter) -> Result<VideoTaskList, Error>`：GET 带查询参数（`page_num`、`page_size` 直接映射，`status` → `filter.status`、`model` → `filter.model`、`task_ids` 逐个展开为 `filter.task_ids`），集成 retry
- [x] 2.5 实现 `ArkClient::delete_video_task(&self, task_id: &str) -> Result<(), Error>`：DELETE，集成 retry

## 3. CLI 参数重写

- [x] 3.1 重写 `cli.rs` 中 `VideoCommand::Generate` 参数：删除 `--image`；新增 `--ref-image`（`Vec<String>`，`action = append`）、`--ref-video`（`Vec<String>`，`action = append`）、`--ref-audio`（`Vec<String>`，`action = append`）、`--web-search`（bool flag）、`--camera-fixed`（bool flag）、`--seed`（`Option<u64>`）；`--resolution` 限制为 `480p`/`720p`；`--aspect-ratio` 添加 `adaptive` 并设为默认；`--duration` 类型改为 `i32`（支持 -1 表示自动）；改用 `--no-audio` flag 替代 `--generate-audio`（默认生成音频）；`--prompt` 改为 `Option`；`--poll-interval` 默认改为 `10`
- [x] 3.2 添加 `VideoCommand::Get` 变体：位置参数 `task_id: String`
- [x] 3.3 添加 `VideoCommand::List` 变体：`--status`（`Option<String>`，使用 clap `value_parser` 限制为 `queued`/`running`/`succeeded`/`failed`/`cancelled`）、`--model`（`Option<String>`）、`--task-ids`（`Option<Vec<String>>`）、`--page`（`u32`，默认 1）、`--page-size`（`u32`，默认 10）
- [x] 3.4 添加 `VideoCommand::Delete` 变体：位置参数 `task_id: String`

## 4. CMD 层实现

- [x] 4.1 在 `cmd/video.rs` 中实现本地文件 base64 编码工具函数（图片、音频），以及 URL-or-file 判断逻辑（`http://`/`https://`/`data:` 开头的直接透传，否则读取文件编码）
- [x] 4.2 实现 generate handler 参数校验：互斥规则（first/last frame vs ref-image/ref-video、last-frame 需搭配 first-frame、ref-audio 需搭配 ref-image/ref-video）、至少需要一种输入（prompt/prompt-file/first-frame/ref-image/ref-video）、数量上限（ref-image ≤ 9、ref-video ≤ 3、ref-audio ≤ 3）、duration 合法范围（4-15 或 -1）、ref-video 仅支持 URL（非 URL 报错提示）
- [x] 4.3 实现 generate handler：读取并合并 prompt/prompt-file（同时存在时 file 在前 + 换行 + prompt 在后）、构建 `ContentItem` 数组和 `CreateVideoTaskRequest`
- [x] 4.4 实现 generate handler：轮询循环（按 poll-interval 间隔查询状态、stderr 进度展示、timeout 检测、超时时输出 task_id 提示用 `video get` 查询）
- [x] 4.5 实现 generate handler：成功后下载视频（从 video_url 用 reqwest 下载 mp4、写入输出文件、未指定 -o 时使用 `video_{timestamp}.mp4` 自动命名）
- [x] 4.6 实现 get handler：调用 `get_video_task` 并格式化输出任务详情（ID、Model、Status、Duration、Ratio、Resolution、Created、Updated、Video URL、Error）
- [x] 4.7 实现 list handler：从 CLI 参数构建 `ListVideoTasksFilter`、调用 `list_video_tasks`、格式化输出总数和每个任务的摘要信息
- [x] 4.8 实现 delete handler：调用 `delete_video_task`、打印确认信息

## 5. 更新 docs/design.md

- [x] 5.1 更新 §2.3：删除模型名称映射表，说明直接使用 API model ID
- [x] 5.2 重写 §3.5：新的参数表（含 generate/get/list/delete 四个子命令）和使用示例
- [x] 5.3 更新 §4.4：修正分辨率列表、删除模型映射说明、新增 get/list/delete 端点文档、更新请求/响应示例

## 6. 验证

- [x] 6.1 `cargo fmt --all -- --check` 通过
- [x] 6.2 `cargo clippy --all-targets -- -D warnings` 通过
- [x] 6.3 `cargo build` 成功
