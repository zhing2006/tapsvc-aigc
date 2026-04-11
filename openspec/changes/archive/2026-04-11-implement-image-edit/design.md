## Context

image generate 已完成，`OpenAiClient` 具备 retry 能力，`cmd/image.rs` 中有可复用的辅助函数（`resolve_prompt`、`build_output_paths`、`format_to_ext`）。image edit 需要新增 multipart/form-data 请求能力，同时复用现有的响应解析和文件写入逻辑。

`docs/design.md` 已完成 image edit 的 API 设计：
- 端点: `POST {base_url}/v1/images/edits`（multipart/form-data）
- image 支持 PNG/JPEG/WebP，< 25MB
- mask 仅支持 PNG，< 4MB，仅 gpt-image-1.5 有效
- 响应格式与 image generate 完全一致（`ImageResponse` + `ImageData`，`b64_json`）

## Goals / Non-Goals

**Goals:**
- 实现 `image edit` 子命令，支持通过 multipart/form-data 上传图片进行编辑
- 在 `OpenAiClient` 中新增 `edit_image()` 方法
- 最大化复用 image generate 已有的辅助函数和响应类型
- CLI 层对文件格式和大小进行校验，提供友好的错误提示

**Non-Goals:**
- 不实现 mask 的自动生成或图形化编辑
- 不在 CLI 层做 image 内容的格式转换（如 JPEG → PNG）
- 不在 CLI 层做 mask 与 image 的尺寸一致性校验（需要图片解码依赖，由 API 侧校验）
- 不在 CLI 层按模型名判断 mask 支持情况（Gemini 不支持 mask，但由 LiteLLM 报错，CLI 不维护模型能力列表）

## Decisions

### D1: multipart/form-data 构建方式

**选择**: 使用 reqwest 内置的 `multipart::Form` + `multipart::Part`

**理由**: reqwest 0.13 原生支持 multipart，无需引入额外依赖。只需在 workspace 中为 reqwest 添加 `multipart` feature。

**替代方案**: 手动构建 multipart body（工作量大，容易出错）。

### D2: 请求类型设计

**选择**: `edit_image()` 方法直接接受一个 `EditImageRequest` 结构体，内部包含 `image_bytes: Vec<u8>`、`image_filename: String` 等原始数据。方法内部构建 `multipart::Form`。

**理由**: 
- 与 `create_image()` 接受 `CreateImageRequest` 的模式一致
- 文件读取和校验在 CLI 层完成，client 层只负责 HTTP 调用
- `EditImageRequest` 不需要 `Serialize` derive（multipart 不走 JSON 序列化）

**替代方案**: 让 `edit_image()` 直接接受 `multipart::Form`（破坏封装，暴露 reqwest 实现细节）。

### D3: 文件校验策略

**选择**: 在 CLI 层（`cmd/image.rs`）对文件格式和大小进行校验：
- image: 按扩展名校验 PNG/JPEG/WebP，大小 < 25MB
- mask: 按扩展名校验 PNG，大小 < 4MB
- mask 与 image 的尺寸一致性不在 CLI 层校验（需要引入图片解码库，成本过高）

**理由**: 提前在本地捕获错误，避免上传大文件后才被 API 拒绝，节省时间和带宽。

### D4: 响应类型复用

**选择**: `edit_image()` 返回值类型直接复用 `ImageResponse`。

**理由**: image edit API 的响应格式与 image generate 完全一致（`created` + `data[].b64_json`）。无需定义新类型。

### D5: retry 逻辑

**选择**: `edit_image()` 内部集成 retry，与 `create_image()` 相同。

**理由**: multipart 请求一样可能遇到网络错误和 429/5xx。retry 时需要重新构建 `multipart::Form`（reqwest 的 `Form` 不可 clone），因此 retry 闭包内需要每次重建 form。

### D6: output_format 参数

**选择**: `CreateImageRequest` 和 `EditImageRequest` 都新增 `output_format: Option<String>` 字段，CLI 的 `--response-format` 值同时传给 `output_format`。`response_format` 继续固定 `"b64_json"`。

**理由**: OpenAI gpt-image-1/1.5 API 有独立的 `output_format` 参数（`png`/`jpeg`/`webp`）控制实际图片编码格式，与 `response_format`（`url`/`b64_json`，控制交付方式）是两个概念。LiteLLM 虽未将 `output_format` 列为一等参数，但会将未知参数作为 provider kwargs 透传给 OpenAI。增加此字段后，至少对 OpenAI 模型能真正控制输出格式，确保文件扩展名与实际编码一致。

**注意**: 此变更同时影响 image generate — 需要在 `CreateImageRequest` 中也加 `output_format` 字段，并在 `cmd/image.rs` 的 generate 分支中传入。

### D7: build_output_paths 前缀参数化

**选择**: 将 `build_output_paths()` 的硬编码前缀 `"image"` 改为参数 `prefix`。generate 传 `"image"`，edit 传 `"edited"`。

**理由**: 现有实现将 `"image_"` 前缀硬编码在函数内，无法被 edit 子命令复用。参数化后两个子命令共享同一函数。

## Risks / Trade-offs

- **[Risk] retry 重建 multipart 开销** — 每次 retry 需要重新构建 `multipart::Form`，包括克隆 image/mask 的字节数据。对于大文件（接近 25MB）可能有内存压力。 → 实际场景中 retry 次数有限（最多 3 次），且 image edit 通常是交互式使用，可接受。
- **[Risk] 文件扩展名不准确** — 按扩展名判断文件格式可能不准确（如 `.png` 文件实际是 JPEG）。 → API 会返回具体错误信息，用户可据此修正。CLI 层校验仅作为快速提示，不做 magic bytes 检测。
