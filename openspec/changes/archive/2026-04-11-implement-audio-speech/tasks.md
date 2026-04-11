## 1. OpenAI Client — audio 模块

- [x] 1.1 新建 `crates/tapsvc-aigc-openai/src/audio.rs`，定义 `SpeechRequest` 结构体（model, input, voice, response_format, speed），派生 Serialize，可选字段使用 `skip_serializing_if`
- [x] 1.2 在 `OpenAiClient` 上实现 `speech()` 方法：POST JSON 到 `/v1/audio/speech`，使用 retry 执行器，成功时 `response.bytes()` 返回 `Vec<u8>`，失败时返回 `Error::Api`（含 Retry-After）
- [x] 1.3 在 `crates/tapsvc-aigc-openai/src/lib.rs` 中添加 `pub mod audio;`

## 2. CLI 参数调整

- [x] 2.1 从 `crates/tapsvc-aigc/src/cli.rs` 的 `AudioCommand::Speech` 中移除 `--instructions` 字段
- [x] 2.2 移除 `--speed` 参数的范围注释（`0.25 - 4.0`），不做 CLI 层校验，由 API 报错

## 3. audio speech 命令实现

- [x] 3.1 实现 `crates/tapsvc-aigc/src/cmd/audio.rs` 的 `handle()` 函数：解析 `--input` / `--input-file` 文本输入（二者互斥，同时提供或都不提供时报错）
- [x] 3.2 构造 `SpeechRequest`，调用 `client.speech()`，将返回的 `Vec<u8>` 写入输出文件
- [x] 3.3 实现输出路径逻辑：指定 `--output` 时使用该路径，否则自动生成 `speech_{timestamp}.{format}`；自动创建父目录

## 4. 验证

- [x] 4.1 `cargo build` + `cargo fmt` + `cargo clippy` 通过
