## Why

项目 tapsvc-aigc 已完成设计文档，但尚无任何代码。需要搭建 Cargo workspace 框架，包括 3 个 crate 的目录结构、依赖声明、以及最小可编译的骨架代码，为后续功能实现打下基础。

## What Changes

- 创建 Cargo workspace root `Cargo.toml`，声明 3 个 member crate
- 创建 `crates/tapsvc-aigc/` — CLI binary crate，包含 `main.rs`、`cli.rs` 骨架
- 创建 `crates/tapsvc-aigc-openai/` — OpenAI 兼容客户端 library crate，包含 `lib.rs`、`client.rs` 骨架
- 创建 `crates/tapsvc-aigc-ark/` — Volcengine ARK 客户端 library crate，包含 `lib.rs`、`client.rs` 骨架
- 统一在 `[workspace.dependencies]` 管理所有依赖版本，所有 crate 禁用 default features，手动选择最小 feature 集
- 统一使用同一个 `TAPSVC_API_KEY` 和 `TAPSVC_BASE_URL` 环境变量（通过 `.env` 文件加载）
- 添加 `.env.example` 模板文件

## Capabilities

### New Capabilities

- `CLI/Build/workspace-structure`: Cargo workspace 布局、crate 拆分、Rust 2024 edition 配置
- `CLI/Build/dependency-management`: workspace 级依赖管理，最小化 feature 选择，避免 crate 重复
- `CLI/Core/cli-skeleton`: CLI 入口与 clap 命令骨架（image / audio / video 子命令定义）
- `CLI/Client/client-skeleton`: OpenAI 和 ARK HTTP 客户端骨架（reqwest client 初始化、统一配置）
- `CLI/Core/env-config`: 环境变量与 `.env` 文件加载机制

### Modified Capabilities

（无，全新项目）

## Impact

- 新增整个 `crates/` 目录结构
- 新增 workspace root `Cargo.toml`
- 新增 `.env.example`
- 依赖：tokio, reqwest (rustls), serde, clap, thiserror, anyhow, tracing, tracing-subscriber, indicatif, base64, dotenvy
- 所有依赖禁用 default features，手动指定最小 feature 集
