## ADDED Requirements

### Requirement: clap derive 命令结构
CLI SHALL 使用 clap derive API 定义命令结构，包含 `image`、`audio`、`video` 三个子命令。无配置文件，默认值硬编码在代码中。

#### Scenario: 顶层命令解析
- **WHEN** 运行 `tapsvc-aigc --help`
- **THEN** SHALL 显示 `image`、`audio`、`video` 三个子命令

#### Scenario: image generate 子命令参数
- **WHEN** 运行 `tapsvc-aigc image generate --help`
- **THEN** SHALL 显示以下参数：`--model`, `--prompt`, `--prompt-file`, `--size`, `--n`, `--quality`, `--response-format`, `--background`, `--output`

#### Scenario: audio speech 子命令参数
- **WHEN** 运行 `tapsvc-aigc audio speech --help`
- **THEN** SHALL 显示以下参数：`--model`, `--voice`, `--input`, `--input-file`, `--format`, `--speed`, `--instructions`, `--output`

#### Scenario: video generate 子命令参数
- **WHEN** 运行 `tapsvc-aigc video generate --help`
- **THEN** SHALL 显示以下参数：`--model`, `--prompt`, `--prompt-file`, `--image`, `--first-frame`, `--last-frame`, `--duration`, `--resolution`, `--aspect-ratio`, `--watermark`, `--generate-audio`, `--poll-interval`, `--timeout`, `--output`

### Requirement: tokio async 入口
`main.rs` SHALL 使用 `#[tokio::main]` 异步入口。

#### Scenario: async main
- **WHEN** 检查 `main.rs`
- **THEN** `main` 函数 SHALL 标注 `#[tokio::main]` 且返回 `anyhow::Result<()>`

### Requirement: tracing 日志初始化
程序启动时 SHALL 初始化 tracing-subscriber，支持 `RUST_LOG` 环境变量控制日志级别。

#### Scenario: 日志初始化
- **WHEN** 程序启动
- **THEN** SHALL 调用 `tracing_subscriber` 初始化，使用 `EnvFilter` 支持 `RUST_LOG`

### Requirement: 优雅退出
程序 SHALL 支持 Ctrl+C 优雅退出。

#### Scenario: Ctrl+C 信号处理
- **WHEN** 用户按 Ctrl+C
- **THEN** 程序 SHALL 捕获信号并优雅退出，不产生 panic
