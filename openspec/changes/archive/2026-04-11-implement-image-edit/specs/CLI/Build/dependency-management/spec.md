## MODIFIED Requirements

### Requirement: 最小化 feature 选择
每个依赖 SHALL 仅启用实际使用的 feature，不启用多余的 feature。

#### Scenario: tokio feature 最小集
- **WHEN** 检查 tokio 的 features
- **THEN** SHALL 仅包含 `rt-multi-thread`、`macros`、`time`、`fs`、`signal`

#### Scenario: reqwest feature 最小集
- **WHEN** 检查 reqwest 的 features
- **THEN** SHALL 包含 `rustls`、`json`、`stream`、`multipart`

#### Scenario: clap feature 最小集
- **WHEN** 检查 clap 的 features
- **THEN** SHALL 仅包含 `derive`、`env`
