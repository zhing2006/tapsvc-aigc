## ADDED Requirements

### Requirement: 统一环境变量
项目 SHALL 使用 `TAPSVC_TAPSVC_API_KEY` 和 `TAPSVC_TAPSVC_BASE_URL` 两个环境变量配置所有 API 访问，三个 API（image、audio、video）共享同一配置。

#### Scenario: 环境变量读取
- **WHEN** 程序启动
- **THEN** SHALL 从环境变量读取 `TAPSVC_API_KEY` 和 `TAPSVC_BASE_URL`

#### Scenario: 环境变量缺失
- **WHEN** `TAPSVC_API_KEY` 或 `TAPSVC_BASE_URL` 未设置
- **THEN** SHALL 输出明确的错误信息并退出

### Requirement: .env 文件支持
程序 SHALL 在启动时通过 `dotenvy` 加载 `.env` 文件，环境变量优先级高于 `.env` 文件。

#### Scenario: .env 文件加载
- **WHEN** 当前目录存在 `.env` 文件
- **THEN** SHALL 加载其中的变量，已存在的环境变量不被覆盖

#### Scenario: .env 文件不存在
- **WHEN** 当前目录无 `.env` 文件
- **THEN** SHALL 不报错，继续从环境变量读取

### Requirement: .env.example 模板
项目 SHALL 提供 `.env.example` 文件作为配置模板。

#### Scenario: 模板内容
- **WHEN** 检查 `.env.example`
- **THEN** SHALL 包含 `TAPSVC_API_KEY` 和 `TAPSVC_BASE_URL` 的示例条目及注释说明
