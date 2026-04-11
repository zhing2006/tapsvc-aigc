## ADDED Requirements

### Requirement: OpenAI 客户端结构
`tapsvc-aigc-openai` crate SHALL 提供 `OpenAiClient` 结构体，封装 `reqwest::Client`、`base_url` 和 `api_key`。

#### Scenario: 客户端构造
- **WHEN** 调用 `OpenAiClient::new(base_url, api_key)`
- **THEN** SHALL 返回配置好的客户端实例，内部持有 `reqwest::Client`

#### Scenario: 模块导出
- **WHEN** 检查 `tapsvc-aigc-openai` 的 `lib.rs`
- **THEN** SHALL 导出 `client` 模块和 `OpenAiClient`

### Requirement: ARK 客户端结构
`tapsvc-aigc-ark` crate SHALL 提供 `ArkClient` 结构体，封装 `reqwest::Client`、`base_url` 和 `api_key`。

#### Scenario: 客户端构造
- **WHEN** 调用 `ArkClient::new(base_url, api_key)`
- **THEN** SHALL 返回配置好的客户端实例，内部持有 `reqwest::Client`

#### Scenario: 模块导出
- **WHEN** 检查 `tapsvc-aigc-ark` 的 `lib.rs`
- **THEN** SHALL 导出 `client` 模块和 `ArkClient`

### Requirement: library crate 无 CLI 依赖
两个 library crate SHALL 不依赖任何 CLI 相关 crate（如 clap、indicatif），保持纯 API 客户端职责。

#### Scenario: 依赖纯净性
- **WHEN** 检查 `tapsvc-aigc-openai` 和 `tapsvc-aigc-ark` 的 `Cargo.toml`
- **THEN** SHALL 不包含 `clap`、`indicatif`、`dirs`、`toml`、`base64` 等 CLI 侧依赖

### Requirement: 错误类型定义
每个 library crate SHALL 使用 `thiserror` 定义自己的错误类型。

#### Scenario: OpenAI 错误类型
- **WHEN** 检查 `tapsvc-aigc-openai`
- **THEN** SHALL 定义 `Error` 枚举，至少包含 HTTP 请求错误变体

#### Scenario: ARK 错误类型
- **WHEN** 检查 `tapsvc-aigc-ark`
- **THEN** SHALL 定义 `Error` 枚举，至少包含 HTTP 请求错误变体
