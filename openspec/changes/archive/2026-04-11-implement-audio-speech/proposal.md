## Why

CLI 的 `audio speech` 子命令目前是 `todo!()` 占位。需要实现完整的 TTS 功能，通过 LiteLLM 代理调用 ElevenLabs 模型（`eleven_multilingual_v2`、`eleven_v3`），让用户能从命令行将文本转换为语音文件。

## What Changes

- 在 `tapsvc-aigc-openai` crate 中新增 `audio` 模块，实现 `/v1/audio/speech` 请求/响应处理
- 在 `OpenAiClient` 上新增 `speech()` 方法，发送 JSON 请求并接收二进制音频响应（`response.bytes()` 一次性读取）
- 实现 `cmd/audio.rs` 中的 `AudioCommand::Speech` 处理逻辑，包括文本输入解析、API 调用和文件输出
- 从 CLI 中移除 `--instructions` 参数（LiteLLM 不支持透传给 ElevenLabs）

## Capabilities

### New Capabilities

- `CLI/Client/audio-speech`: OpenAI 兼容客户端的 `/v1/audio/speech` 请求类型定义和 HTTP 调用（JSON 请求、二进制响应）
- `CLI/Audio/audio-speech-command`: `audio speech` CLI 子命令处理逻辑（文本输入、API 调用、文件写入）

### Modified Capabilities

- `CLI/Core/cli-skeleton`: 移除 `AudioCommand::Speech` 的 `--instructions` 参数

## Impact

- **代码**: `crates/tapsvc-aigc-openai/src/audio.rs`（新建）、`crates/tapsvc-aigc-openai/src/client.rs`、`crates/tapsvc-aigc-openai/src/lib.rs`、`crates/tapsvc-aigc/src/cmd/audio.rs`、`crates/tapsvc-aigc/src/cli.rs`
- **API**: 新增对 `POST /v1/audio/speech` 端点的调用
- **依赖**: 无新增依赖，复用现有 `reqwest`（stream feature 已启用）
