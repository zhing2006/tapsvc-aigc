## Context

CLI 的 `audio speech` 子命令当前是 `todo!()` 占位。`tapsvc-aigc-openai` crate 已有 `image` 模块和 `OpenAiClient`（含 `create_image`、`edit_image` 方法），需要新增 `audio` 模块。

与 image 的区别在于：image 返回 JSON（含 base64 编码数据），audio 返回二进制流（`application/octet-stream`）。

## Goals / Non-Goals

**Goals:**
- 实现 `OpenAiClient::speech()` 方法，支持 JSON 请求 + 二进制响应
- 实现 `cmd/audio.rs` 的完整 speech 处理逻辑
- 从 CLI 移除 `--instructions` 参数
- 复用现有 retry 和 error 机制

**Non-Goals:**
- 不做文本分片（长文本超限由 API 报错）
- 不做 voice 列表查询（LiteLLM 不支持）
- 不做流式写入（音频文件通常较小，一次性写入即可）

## Decisions

### 1. 响应处理：bytes() 一次性读取

**选择**: 使用 `response.bytes()` 一次性读取完整响应体，而非 streaming chunk 写入。

**理由**: TTS 音频文件通常几百 KB 到几 MB，一次性读取内存完全可以承受。streaming 写入增加了复杂度（需要处理部分写入失败），收益不大。

**备选**: 使用 `response.bytes_stream()` + `tokio::io::copy` 流式写入文件。如果未来需要处理超长文本生成的大音频文件，可以改用此方案。

### 2. speech() 返回 Vec<u8>

**选择**: `speech()` 方法直接返回 `Result<Vec<u8>, Error>`，不定义额外的 response 结构体。

**理由**: 二进制音频没有结构化字段需要解析（不像 image 有 `b64_json`、`revised_prompt`），直接返回原始字节最简洁。

### 3. 移除 --instructions 参数

**选择**: 从 `AudioCommand::Speech` 中删除 `--instructions` 字段。

**理由**: LiteLLM 会 strip 掉不认识的参数（[BerriAI/litellm#20078](https://github.com/BerriAI/litellm/issues/20078)），且 ElevenLabs 原生也没有 `instructions` 概念。保留此参数会误导用户。

## Risks / Trade-offs

- **[Risk] 大文件内存占用** → 当前用 `bytes()` 一次性加载。TTS 音频文件通常较小，可接受。如果未来需要优化，改为流式写入。
- **[Risk] 格式支持差异** → `aac`、`flac`、`wav` 是否被 LiteLLM 支持取决于版本。CLI 不做前端校验，让 API 报错。
