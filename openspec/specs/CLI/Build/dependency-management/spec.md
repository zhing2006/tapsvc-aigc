## ADDED Requirements

### Requirement: workspace 级依赖版本管理
所有共享依赖 SHALL 在 workspace root `Cargo.toml` 的 `[workspace.dependencies]` 中统一声明版本，各 crate 通过 `.workspace = true` 引用。

#### Scenario: 共享依赖版本一致
- **WHEN** 多个 crate 依赖同一个外部 crate（如 `reqwest`、`serde`、`tokio`）
- **THEN** 版本号 SHALL 仅在 workspace root 声明一次，各 crate 的 `Cargo.toml` 中 SHALL 使用 `<crate>.workspace = true`

### Requirement: 禁用 default features
所有依赖 SHALL 设置 `default-features = false`，手动选择需要的 feature。

#### Scenario: 无隐式 feature 引入
- **WHEN** 检查 `[workspace.dependencies]` 中的每个依赖
- **THEN** 每个依赖 SHALL 包含 `default-features = false`（除非该 crate 无 default features）

### Requirement: 最小化 feature 选择
每个依赖 SHALL 仅启用实际使用的 feature，不启用多余的 feature。

#### Scenario: tokio feature 最小集
- **WHEN** 检查 tokio 的 features
- **THEN** SHALL 仅包含 `rt-multi-thread`、`macros`、`time`、`fs`、`signal`

#### Scenario: reqwest feature 最小集
- **WHEN** 检查 reqwest 的 features
- **THEN** SHALL 仅包含 `rustls`、`json`、`stream`

#### Scenario: clap feature 最小集
- **WHEN** 检查 clap 的 features
- **THEN** SHALL 仅包含 `derive`、`env`

### Requirement: 版本约束策略
核心 crate（大用户量、稳定 API）SHALL 锁主版本号；功能性 crate SHALL 锁到次版本号。

#### Scenario: 核心 crate 主版本约束
- **WHEN** 检查 `tokio`、`serde`、`serde_json`、`anyhow` 的版本
- **THEN** SHALL 使用主版本约束（如 `"1"`）

#### Scenario: 功能性 crate 次版本约束
- **WHEN** 检查 `reqwest`、`clap`、`thiserror`、`tracing`、`tracing-subscriber`、`indicatif`、`base64`、`dotenvy` 的版本
- **THEN** SHALL 使用次版本约束（如 `"0.13"`、`"4.6"`）

### Requirement: 最小化 crate 重复
workspace 依赖选择 SHALL 避免引入同一 crate 的多个不兼容版本。

#### Scenario: cargo tree 无重复
- **WHEN** 运行 `cargo tree --duplicates`
- **THEN** SHALL 无需要关注的重复 crate（允许 build dependencies 和 proc-macro 的合理重复）
