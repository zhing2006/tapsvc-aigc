## 1. 依赖更新

- [x] 1.1 workspace root `Cargo.toml` 中为 reqwest 添加 `multipart` feature

## 2. OpenAI Client 层

- [x] 2.1 `CreateImageRequest` 新增 `output_format: Option<String>` 字段（serde skip_serializing_if）
- [x] 2.2 在 `image.rs` 中定义 `EditImageRequest` 结构体（image_bytes、mask_bytes、model、prompt、output_format 等字段）
- [x] 2.3 在 `client.rs` 中实现 `edit_image()` 方法：构建 multipart/form-data、POST 到 `/v1/images/edits`、集成 retry（每次 retry 重建 Form）
- [x] 2.4 在 `lib.rs` 中导出 `EditImageRequest`

## 3. CLI 层 — image generate 修改

- [x] 3.1 `cmd/image.rs` generate 分支：将 `--response-format` 的值传入 `CreateImageRequest.output_format`
- [x] 3.2 `build_output_paths()` 新增 `prefix` 参数，generate 调用时传 `"image"`

## 4. CLI 层 — image edit 实现

- [x] 4.1 在 `cli.rs` 的 `ImageCommand` enum 中新增 `Edit` variant，定义所有 CLI 参数（--image、--mask、--size、--n、--prompt、--prompt-file、--response-format、--output）
- [x] 4.2 在 `cmd/image.rs` 的 `handle()` 中新增 `ImageCommand::Edit` 匹配分支
- [x] 4.3 实现文件格式校验（image: PNG/JPEG/WebP、mask: PNG only）和大小校验（image < 25MB、mask < 4MB）
- [x] 4.4 实现文件读取、`EditImageRequest` 构建（output_format 取自 --response-format）、`edit_image()` 调用
- [x] 4.5 复用 `resolve_prompt()`、`build_output_paths()`（前缀传 `"edited"`）、`format_to_ext()` 和 base64 解码写入逻辑

## 5. 验证

- [x] 5.1 `cargo build` 编译通过
- [x] 5.2 `cargo fmt --all -- --check` 和 `cargo clippy --all-targets -- -D warnings` 通过
