## Context

tapsvc-aigc 是一个 Rust CLI 工具，聚合图片生成、语音合成、视频生成三种 AIGC 能力。项目已有详细的设计文档（`docs/design.md`），但尚无代码。本次变更搭建 Cargo workspace 框架，使项目达到可编译、可运行 `--help` 的状态。

三个 API（OpenAI 兼容的 image/audio、Volcengine ARK 的 video）统一通过同一个代理访问，共享 `TAPSVC_BASE_URL` 和 `TAPSVC_API_KEY`。

## Goals / Non-Goals

**Goals:**
- Cargo workspace 可 `cargo build` 通过
- CLI 可运行 `--help` 显示所有子命令
- 两个 library crate 可被 CLI crate 正确引用
- 依赖 feature 最小化，无 default features
- `.env` 环境变量加载可用

**Non-Goals:**
- 实际的 API 调用逻辑（后续变更）
- 配置文件读写逻辑（后续变更）
- 进度条、重试、错误恢复等（后续变更）
- 测试代码（后续变更）

## Decisions

### Decision 1: TLS backend 选择 rustls (aws-lc-rs)

使用 reqwest 0.13 的 `rustls` feature，底层为 aws-lc-rs 加密库。

**Why**: reqwest 0.13 已将 rustls 设为默认推荐。纯 Rust 实现，Windows 上无需 OpenSSL。aws-lc-rs 是 AWS 维护的高性能加密库，替代了 ring。

**Alternatives**: `native-tls`（依赖系统 TLS 库，Linux 上需要 OpenSSL dev headers）。

### Decision 2: 统一 TAPSVC_BASE_URL + TAPSVC_API_KEY

三个 API 共享同一个 `TAPSVC_BASE_URL` 和 `TAPSVC_API_KEY` 环境变量，不再区分 OpenAI 代理和 ARK 代理。

**Why**: 用户通过统一代理访问所有 API，简化配置。CLI 和 library crate 只需一套凭据。使用 `TAPSVC_` 前缀避免与其他工具的环境变量冲突。

**Alternatives**: 分别配置 `OPENAI_BASE_URL` / `OPENAI_API_KEY` + `ARK_BASE_URL` / `ARK_API_KEY`（增加配置复杂度，且用户实际使用统一代理）。

### Decision 3: 图片传输使用 base64 in JSON

图生视频的参考图片通过 JSON body 中的 base64 data URL 传输，不使用 multipart form upload。

**Why**: OpenAI API 和 Seedance 2.0 API 都支持 `data:image/<format>;base64,<data>` 格式。统一走 JSON body 简化代码，不需要 reqwest 的 `multipart` feature。

**Alternatives**: multipart form upload（需额外 feature，两个 API 的 multipart 格式不同）。

### Decision 4: dotenvy 加载 .env

使用 `dotenvy` crate 在程序启动时加载 `.env` 文件。

**Why**: `dotenvy` 是 `dotenv` 的活跃维护 fork，近期下载量 2400 万/期。`.env` 是标准的本地开发凭据管理方式。

**Alternatives**: 不使用 .env，要求用户手动 `export`（不便于本地开发）。

### Decision 5: 各 crate 的 tokio feature 分配

- **workspace 级别**: `default-features = false`
- **CLI crate** (`tapsvc-aigc`): `rt-multi-thread`, `macros`, `time`, `fs`, `signal`
- **Library crates**: `time`（仅用于轮询 sleep）

**Why**: CLI 需要完整运行时和信号处理。Library crate 只需 `time` 用于轮询间隔。通过 workspace 统一版本，crate 各自声明所需 features，Cargo 会自动合并。

### Decision 6: reqwest default-features = false

workspace 级别声明 `reqwest` 为 `default-features = false`，手动添加 `rustls`, `json`, `stream`。

**Why**: 避免引入不需要的 `charset`, `http2`, `system-proxy` 等 default features。

### Decision 7: GPT image 模型的 response_format 语义

GPT image 模型（gpt-image-1、gpt-image-1.5 等）的 `response_format` 参数含义与旧版 DALL-E 不同：
- **旧版 DALL-E**: `response_format` = `b64_json` | `url`（传输格式）
- **GPT image**: `response_format` = `png` | `jpeg` | `webp`（输出图片格式），始终返回 base64

CLI 的 `--response-format` 采用 GPT image 语义（`png`/`jpeg`/`webp`），默认 `png`。

**Why**: 旧版 DALL-E 模型将于 2026-05-12 停止支持，面向新模型设计。

## Risks / Trade-offs

- **[aws-lc-rs 编译]** → aws-lc-rs 需要 C 编译器（cmake）。Windows 上通常已有 MSVC。若编译失败，可回退到 `native-tls`。
- **[统一 TAPSVC_BASE_URL 限制]** → 如果未来需要对不同 API 使用不同代理，需要重新引入多配置。当前先简化，后续可扩展。
- **[no default features 风险]** → 某些 crate 的 default features 包含必要功能。已逐个确认所选 features 覆盖实际需求。
