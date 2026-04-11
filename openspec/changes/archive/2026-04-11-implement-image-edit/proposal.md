## Why

image generate 子命令已完成，但 `image edit` 仍为 `todo!()`。`docs/design.md` 中已完成 image edit 的 API 设计（multipart/form-data 请求、文件格式/大小限制、Gemini mask 不支持等），需要将设计落地为可运行的代码。

## What Changes

- 在 `tapsvc-aigc-openai` crate 中新增 `edit_image()` 方法，通过 multipart/form-data 调用 `/v1/images/edits` 端点
- 新增 `EditImageRequest` 类型定义，包含 image/mask 文件数据和元数据参数
- 在 `ImageCommand` enum 中新增 `Edit` variant，定义 CLI 参数（`--image`、`--mask`、`--size`、`--n` 等）
- 实现 `cmd/image.rs` 中 `Edit` 分支的完整处理逻辑，复用已有的 `resolve_prompt()`、`build_output_paths()`、`format_to_ext()` 等辅助函数
- reqwest 添加 `multipart` feature 以支持文件上传
- `CreateImageRequest` 和 `EditImageRequest` 新增 `output_format` 字段，CLI 的 `--response-format` 值同时传给 `output_format`（控制实际图片编码格式），`response_format` 继续固定 `"b64_json"`
- `build_output_paths()` 前缀参数化（generate: `"image"`，edit: `"edited"`）

## Capabilities

### New Capabilities

- `CLI/Client/image-edit`: OpenAI 兼容的图片编辑 API 客户端，包括 multipart/form-data 请求构建、`EditImageRequest` 类型定义和 `edit_image()` HTTP 调用
- `CLI/Image/image-edit-command`: CLI 层 `image edit` 子命令的完整处理逻辑，包括文件读取、prompt 合并、输出文件写入

### Modified Capabilities

- `CLI/Build/dependency-management`: reqwest 新增 `multipart` feature
- `CLI/Client/image-generation`: `CreateImageRequest` 新增 `output_format` 字段
- `CLI/Image/image-command`: `build_output_paths()` 前缀参数化；generate 分支传入 `output_format`

## Impact

- **代码**: `crates/tapsvc-aigc-openai/src/` 新增 image edit 类型和方法；`crates/tapsvc-aigc/src/cli.rs` 新增 `Edit` variant；`crates/tapsvc-aigc/src/cmd/image.rs` 新增 edit 处理逻辑
- **依赖**: reqwest 新增 `multipart` feature
- **API**: 通过 `TAPSVC_BASE_URL` 代理调用 `{base_url}/v1/images/edits` 端点（multipart/form-data）
