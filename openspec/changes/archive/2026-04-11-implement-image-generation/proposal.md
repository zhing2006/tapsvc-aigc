## Why

CLI 骨架和 workspace 结构已搭建完成，但 image generate 子命令仍为 `todo!()`。需要实现完整的图片生成功能，包括 OpenAI 兼容 API 客户端（请求/响应类型 + HTTP 调用）和 CLI 层的文件输出逻辑，使用户可以通过命令行生成图片。

## What Changes

- 新增 `tapsvc-aigc-core` crate，提供通用 retry 组件（指数退避 + jitter + Retry-After 支持），以及共享错误 trait
- 在 `tapsvc-aigc-openai` crate 中新增 `image` 模块，定义 `/v1/images/generations` 的请求/响应类型和调用方法，集成 retry 逻辑
- 实现 `cmd/image.rs` 中 `ImageCommand::Generate` 的完整处理逻辑
- 支持 `--prompt` 和 `--prompt-file` 合并拼接（非互斥），file 内容在前、prompt 文本在后
- 支持多图输出时的自动编号命名
- 未指定 `-o` 时基于时间戳自动生成文件名（依赖 chrono）
- 参数（size、quality、background 等）以 String 直传 API，不在 CLI 层做 enum 校验；`--n` 在 CLI 层约束为 1-10
- 同步更新 `docs/design.md`，修正端点路径、prompt 拼接说明

## Capabilities

### New Capabilities

- `CLI/Core/retry-executor`: 通用 retry 组件，支持可配置的指数退避、jitter、Retry-After header、可重试状态码判断
- `CLI/Client/image-generation`: OpenAI 兼容的图片生成 API 客户端，包括请求/响应类型定义和 HTTP 调用
- `CLI/Image/image-command`: CLI 层 image generate 子命令的完整处理逻辑，包括 prompt 合并、文件输出、自动命名

### Modified Capabilities

- `CLI/Build/workspace-structure`: workspace members 从 3 个扩展为 4 个（新增 `tapsvc-aigc-core`）

## Impact

- **代码**: 新增 `crates/tapsvc-aigc-core/` crate；`crates/tapsvc-aigc-openai/src/` 新增 `image.rs`；`crates/tapsvc-aigc/src/cmd/image.rs` 从 `todo!()` 变为完整实现
- **依赖**: 新增 `chrono`（时间戳格式化）、新增 `rand`（jitter 随机延迟），均使用 `default-features = false`
- **API**: 通过 `TAPSVC_BASE_URL` 代理调用 `{base_url}/v1/images/generations` 端点
- **文档**: `docs/design.md` 同步更新端点路径和 prompt 语义
