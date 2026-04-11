## Why

CLI 的 `video generate` 子命令目前仅有 CLI 参数定义，实际实现为 `todo!()`。需要对接 Volcengine ARK Content Generation API（Seedance 2.0），完成视频生成全流程（提交任务、轮询状态、下载结果），并新增 `video get`、`video list`、`video delete` 子命令用于任务管理。同时需要更新 `docs/design.md` 中与实际 API 不符的参数描述（分辨率、模型映射、默认值等）。

## What Changes

- 在 `tapsvc-aigc-ark` crate 中实现 ARK Content Generation API client（create / get / list / delete 四个端点）
- 在 `cmd/video.rs` 中实现 `video generate` 的完整流程：本地文件 base64 编码、构建请求、轮询、下载视频
- 新增 `video get`、`video list`、`video delete` 子命令
- 重写 `cli.rs` 中 `VideoCommand` 的参数定义，对齐实际 API 能力：
  - 删除 `--image` 参数，用 `--first-frame` + `--ref-image` 替代
  - 分辨率缩减为 `480p` / `720p`
  - 新增 `--ref-image`（可重复，最多 9 张）、`--ref-video`（可重复，最多 3 个）、`--ref-audio`（可重复，最多 3 个）
  - 新增 `--web-search`、`--camera-fixed`、`--seed` 参数
  - `--generate-audio` 默认改为 true，改用 `--no-audio` flag 禁用
  - `--prompt` 改为可选（纯图生视频场景无需 prompt）
  - aspect ratio 新增 `adaptive` 选项并设为默认值
- 更新 `docs/design.md`：
  - 删除模型名称映射（直接使用 API model ID）
  - 修正分辨率列表
  - 重写 §3.5 视频生成命令参数表和示例
  - 新增 get / list / delete 子命令文档
  - 更新 §4.4 API 对接细节

## Capabilities

### New Capabilities

- `CLI/Video/video-generate-command`: video generate 子命令的 CLI 参数定义与执行流程（轮询、下载、进度展示）
- `CLI/Video/video-manage-commands`: video get / list / delete 子命令的 CLI 参数定义与执行逻辑
- `CLI/Client/video-generation`: ARK Content Generation API client，包含请求/响应类型和 HTTP 调用（create / get / list / delete）

### Modified Capabilities

（无需修改现有 spec 的行为定义）

## Impact

- **代码变更**：`crates/tapsvc-aigc-ark/src/`（新增 `video.rs`，扩展 `client.rs`）、`crates/tapsvc-aigc/src/cli.rs`、`crates/tapsvc-aigc/src/cmd/video.rs`
- **依赖**：`tapsvc-aigc-ark` 需要使用 `tapsvc-aigc-core` 的 retry 机制；CLI binary 可能需要 `base64` crate 用于本地文件编码
- **文档**：`docs/design.md` §2.3、§3.5、§4.4 需要更新
- **API**：通过 `{base_url}/volcengine/api/v3/contents/generations/tasks` 端点与 Volcengine ARK 交互
