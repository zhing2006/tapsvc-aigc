## MODIFIED Requirements

### Requirement: Cargo workspace 布局
项目 SHALL 采用 Cargo workspace 管理多 crate，workspace root 位于项目根目录 `Cargo.toml`。

#### Scenario: workspace 包含 4 个 member crate
- **WHEN** 检查 workspace `Cargo.toml` 的 `[workspace].members`
- **THEN** SHALL 包含 `crates/tapsvc-aigc`、`crates/tapsvc-aigc-openai`、`crates/tapsvc-aigc-ark`、`crates/tapsvc-aigc-core` 四个成员
