## ADDED Requirements

### Requirement: Cargo workspace 布局
项目 SHALL 采用 Cargo workspace 管理多 crate，workspace root 位于项目根目录 `Cargo.toml`。

#### Scenario: workspace 包含 4 个 member crate
- **WHEN** 检查 workspace `Cargo.toml` 的 `[workspace].members`
- **THEN** SHALL 包含 `crates/tapsvc-aigc`、`crates/tapsvc-aigc-openai`、`crates/tapsvc-aigc-ark`、`crates/tapsvc-aigc-core` 四个成员

### Requirement: Rust edition 统一为 2024
所有 crate SHALL 使用 Rust edition 2024，通过 `[workspace.package]` 统一声明。

#### Scenario: workspace package 配置
- **WHEN** 检查 workspace `Cargo.toml` 的 `[workspace.package]`
- **THEN** `edition` SHALL 为 `"2024"`，`license` SHALL 为 `"Apache-2.0"`

### Requirement: resolver 使用 v3
workspace SHALL 使用 `resolver = "3"`。

#### Scenario: resolver 版本
- **WHEN** 检查 workspace `Cargo.toml` 的 `[workspace].resolver`
- **THEN** 值 SHALL 为 `"3"`

### Requirement: CLI crate 为 binary
`tapsvc-aigc` crate SHALL 为 binary crate，编译输出可执行文件。

#### Scenario: binary crate 结构
- **WHEN** 检查 `crates/tapsvc-aigc/src/main.rs`
- **THEN** 文件 SHALL 存在且包含 `fn main()` 入口

### Requirement: library crate 结构
`tapsvc-aigc-openai` 和 `tapsvc-aigc-ark` SHALL 为 library crate。

#### Scenario: library crate 入口
- **WHEN** 检查 `crates/tapsvc-aigc-openai/src/lib.rs` 和 `crates/tapsvc-aigc-ark/src/lib.rs`
- **THEN** 两个文件 SHALL 存在
