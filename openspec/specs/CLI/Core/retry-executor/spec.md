## ADDED Requirements

### Requirement: tapsvc-aigc-core crate 结构
SHALL 新增 `tapsvc-aigc-core` workspace 成员 crate，作为 client crate 的共享基础层。

#### Scenario: workspace 成员
- **WHEN** 检查 workspace 根 `Cargo.toml`
- **THEN** `members` 数组 SHALL 包含 `"crates/tapsvc-aigc-core"`

#### Scenario: 无 CLI 依赖
- **WHEN** 检查 `tapsvc-aigc-core` 的 `Cargo.toml`
- **THEN** SHALL 不依赖 clap、indicatif、base64 等 CLI 侧 crate

### Requirement: RetryConfig 配置结构
SHALL 定义 `RetryConfig` 结构体，所有字段可配置。

#### Scenario: 字段定义
- **WHEN** 构建 `RetryConfig`
- **THEN** SHALL 包含以下字段：`base_delay: Duration`、`factor: u32`、`max_jitter: Duration`、`max_retries: u32`、`max_delay: Duration`

#### Scenario: Default 实现
- **WHEN** 调用 `RetryConfig::default()`
- **THEN** SHALL 返回 `base_delay=2s, factor=2, max_jitter=1s, max_retries=3, max_delay=30s`

### Requirement: Retryable trait
SHALL 定义 `Retryable` trait，用于判断错误是否可重试以及提取 Retry-After 延迟。

#### Scenario: trait 方法
- **WHEN** 检查 `Retryable` trait 定义
- **THEN** SHALL 包含 `fn is_retryable(&self) -> bool` 和 `fn retry_after(&self) -> Option<Duration>` 两个方法

### Requirement: 可重试状态码判断
SHALL 提供函数判断 HTTP 状态码是否可重试。

#### Scenario: 可重试状态码
- **WHEN** 传入状态码 429、500、502、503、504
- **THEN** SHALL 返回 `true`

#### Scenario: 不可重试状态码
- **WHEN** 传入状态码 400、401、403、404、422
- **THEN** SHALL 返回 `false`

### Requirement: 泛型 retry 执行器
SHALL 提供异步泛型函数 `retry`，接受 `RetryConfig` 和一个返回 `Result<T, E>` 的闭包，自动执行重试循环。`max_retries` 表示最多重试次数，总尝试次数 = 1（初始）+ max_retries（重试）。

#### Scenario: 首次成功
- **WHEN** operation 首次调用返回 `Ok(value)`
- **THEN** SHALL 直接返回 `Ok(value)`，不进行任何等待

#### Scenario: 可重试错误后成功
- **WHEN** operation 前两次返回可重试错误，第三次返回 `Ok(value)`
- **THEN** SHALL 返回 `Ok(value)`，中间按指数退避等待

#### Scenario: 超过最大重试次数
- **WHEN** `max_retries=3` 且 operation 连续 4 次（1 次初始 + 3 次重试）均返回可重试错误
- **THEN** SHALL 返回第 4 次尝试的 `Err`

#### Scenario: 不可重试错误立即返回
- **WHEN** operation 返回不可重试错误（如 400 Bad Request）
- **THEN** SHALL 立即返回 `Err`，不进行重试

#### Scenario: 指数退避延迟计算
- **WHEN** `base_delay=2s, factor=2`
- **THEN** 第 1 次重试延迟 SHALL 为 `2s + jitter`，第 2 次为 `4s + jitter`，第 3 次为 `8s + jitter`

#### Scenario: 延迟上限
- **WHEN** 计算出的延迟超过 `max_delay`
- **THEN** SHALL 使用 `max_delay` 替代

#### Scenario: Retry-After header 优先
- **WHEN** 可重试错误的 `retry_after()` 返回 `Some(duration)`
- **THEN** SHALL 使用该 duration 替代计算出的指数退避延迟

### Requirement: openai client crate 依赖 core
`tapsvc-aigc-openai` SHALL 在 `Cargo.toml` 中依赖 `tapsvc-aigc-core`。

#### Scenario: 依赖声明
- **WHEN** 检查 `tapsvc-aigc-openai` 的 `Cargo.toml`
- **THEN** SHALL 包含 `tapsvc-aigc-core` 依赖
