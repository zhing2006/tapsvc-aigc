## MODIFIED Requirements

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
- **THEN** SHALL 显示以下参数：`--model`, `--voice`, `--input`, `--input-file`, `--format`, `--speed`, `--output`

#### Scenario: video generate 子命令参数
- **WHEN** 运行 `tapsvc-aigc video generate --help`
- **THEN** SHALL 显示以下参数：`--model`, `--prompt`, `--prompt-file`, `--image`, `--first-frame`, `--last-frame`, `--duration`, `--resolution`, `--aspect-ratio`, `--watermark`, `--generate-audio`, `--poll-interval`, `--timeout`, `--output`
