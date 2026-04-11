## 1. Workspace Root

- [x] 1.1 创建 workspace root `Cargo.toml`，声明 `[workspace]` members、resolver、package 元数据
- [x] 1.2 在 `[workspace.dependencies]` 声明所有共享依赖，统一版本与 features（全部 `default-features = false`）

## 2. Library Crate — tapsvc-aigc-openai

- [x] 2.1 创建 `crates/tapsvc-aigc-openai/Cargo.toml`，引用 workspace 依赖（reqwest, serde, serde_json, thiserror, tracing, tokio）
- [x] 2.2 创建 `src/lib.rs`，声明 `client` 模块并导出 `OpenAiClient`
- [x] 2.3 创建 `src/client.rs`，实现 `OpenAiClient` 结构体（持有 reqwest::Client、base_url、api_key）和 `new()` 构造函数
- [x] 2.4 创建 `src/error.rs`，使用 thiserror 定义 `Error` 枚举

## 3. Library Crate — tapsvc-aigc-ark

- [x] 3.1 创建 `crates/tapsvc-aigc-ark/Cargo.toml`，引用 workspace 依赖
- [x] 3.2 创建 `src/lib.rs`，声明 `client` 模块并导出 `ArkClient`
- [x] 3.3 创建 `src/client.rs`，实现 `ArkClient` 结构体和 `new()` 构造函数
- [x] 3.4 创建 `src/error.rs`，使用 thiserror 定义 `Error` 枚举

## 4. CLI Crate — tapsvc-aigc

- [x] 4.1 创建 `crates/tapsvc-aigc/Cargo.toml`，引用 workspace 依赖及两个 library crate
- [x] 4.2 创建 `src/cli.rs`，使用 clap derive 定义完整命令结构（image generate / audio speech / video generate）
- [x] 4.3 创建 `src/main.rs`，包含 `#[tokio::main]`、dotenvy 加载、tracing 初始化、clap 解析、graceful shutdown 骨架
- [x] 4.4 创建 `src/cmd/mod.rs` 及 `image.rs`、`audio.rs`、`video.rs` 子命令处理骨架（todo!() 占位）

## 5. 环境配置

- [x] 5.1 创建 `.env.example` 模板文件
- [x] 5.2 确认 `.gitignore` 包含 `.env` 规则

## 6. 验证

- [x] 6.1 `cargo build` 编译通过
- [x] 6.2 `cargo tree --duplicates` 确认无不必要的 crate 重复
- [x] 6.3 运行 `cargo run -- --help` 确认子命令正确显示
