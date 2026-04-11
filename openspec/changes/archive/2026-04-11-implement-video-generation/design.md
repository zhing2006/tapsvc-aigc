## Context

CLI 的视频生成功能已有 `VideoCommand::Generate` 的参数定义和 `cmd/video.rs` 的空壳（`todo!()`），以及 `tapsvc-aigc-ark` crate 的基础 `ArkClient`（仅 `new`/`http`/`base_url`/`api_key`）。需要对接 Volcengine ARK Content Generation API（Seedance 2.0），这是一个异步任务制 API，与 OpenAI 的同步响应模式完全不同。

API 端点通过统一代理访问：`{base_url}/volcengine/api/v3/contents/generations/tasks`。

当前 `cli.rs` 中的视频参数与实际 API 能力有多处不符（分辨率列表、缺少参考图/视频/音频参数、默认值等），需要重写。

## Goals / Non-Goals

**Goals:**

- 实现 ARK Content Generation API client（create / get / list / delete）
- 实现 `video generate` 完整流程：本地文件 base64 编码 → 提交任务 → 轮询 → 下载视频
- 新增 `video get` / `video list` / `video delete` 子命令
- 重写 CLI 视频参数定义，对齐实际 API
- 更新 `docs/design.md` 相关章节

**Non-Goals:**

- Draft 模式（先出草稿再精修）不在此次范围内
- Callback URL（webhook 回调）机制不实现
- 视频上传 / 转码等非生成类能力
- `return_last_frame`、`service_tier`、`execution_expires_after` 等高级参数暂不暴露到 CLI

## Decisions

### 1. 轮询逻辑放在 cmd 层，不放在 ARK client 中

ARK client（`tapsvc-aigc-ark`）只做纯 HTTP 请求/响应的映射，提供 `create_video_task`、`get_video_task`、`list_video_tasks`、`delete_video_task` 四个方法。轮询循环、超时控制、进度展示全部放在 `cmd/video.rs` 中。

**理由：** 与 OpenAI client 保持一致的设计——library crate 是纯 API 调用层，不含 UI 逻辑（如 indicatif 进度条）或策略逻辑（如轮询间隔）。这样 ARK client 可被其他项目复用而不带入 CLI 依赖。

### 2. 本地文件 base64 编码放在 cmd 层

ARK client 的 content item 接收的是已编码好的 URL 字符串（`data:image/png;base64,...` 或 `https://...`）。文件读取和 base64 编码由 `cmd/video.rs` 完成。

**理由：** ARK client 不应依赖文件系统操作。图片/音频的编码是 CLI 特有的便利功能，client 只关心 URL。

### 3. 互斥规则在 cmd 层校验

API 的输入互斥规则（first/last frame 与 ref image/video 互斥、last frame 必须搭配 first frame 等）在 `cmd/video.rs` 的 handle 函数入口处校验，不依赖 clap 的 conflict 机制。

**理由：** 互斥关系较复杂（跨多个参数的条件组合），clap 的 `conflicts_with` 只能处理简单的两两冲突。在代码中手动校验可以给出更清晰的错误提示。

### 4. 模型名称不做映射，直接透传

用户通过 `-m` 传入 `doubao-seedance-2-0-260128` 或 `doubao-seedance-2-0-fast-260128`，CLI 直接透传给 API。

**理由：** 减少维护负担。火山引擎未来可能增加新模型，CLI 不需要为每个新模型更新映射表。

### 5. ARK client 使用 core crate 的 retry 机制

`tapsvc-aigc-ark` 的 `Error` 类型实现 `tapsvc_aigc_core::Retryable` trait，与 `tapsvc-aigc-openai` 保持一致的 retry 策略。

**理由：** 复用已有的指数退避 + jitter + Retry-After 机制，保持项目内一致性。

### 6. `--generate-audio` 默认为 true，使用 `--no-audio` flag 禁用

**理由：** 视频默认有声音更符合用户直觉。Python 参考实现也采用这一默认值。

### 7. 视频下载使用 reqwest 而非额外 HTTP 库

视频文件下载（从 `video_url` 下载 mp4）在 `cmd/video.rs` 中直接使用 reqwest 完成。

**理由：** 项目已依赖 reqwest 且启用了 stream feature，无需引入新依赖。

## Risks / Trade-offs

**[视频生成耗时长]** → 视频生成通常需要 30 秒到几分钟。提供 `--poll-interval` 和 `--timeout` 参数让用户控制。超时时输出 task_id，告知用户可通过 `video get <task-id>` 稍后查询。

**[video_url 有时效性]** → 生成的视频下载链接 24 小时内有效。CLI 在成功后立即下载，输出中提醒此限制。

**[本地大文件 base64 编码内存占用]** → 图片/音频 base64 编码会将文件大小增加约 33%。对于 CLI 工具的典型使用场景（单个图片/音频文件，通常几 MB），这不构成问题。

**[轮询对 API 的压力]** → 默认 10 秒间隔，用户可通过 `--poll-interval` 调整。不设下限，信任用户判断。
