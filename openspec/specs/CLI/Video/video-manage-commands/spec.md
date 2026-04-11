## ADDED Requirements

### Requirement: Get 子命令
`VideoCommand::Get` SHALL 接受一个 `task_id` 参数，查询并展示单个视频生成任务的详情。

#### Scenario: 查询成功
- **WHEN** 用户执行 `video get <task-id>`
- **THEN** SHALL 调用 ARK API 获取任务详情，格式化输出以下字段：ID、Model、Status、Duration、Ratio、Resolution、Created、Updated、Video URL（如有）、Error（如有）

#### Scenario: 任务不存在
- **WHEN** 用户指定的 task_id 不存在
- **THEN** SHALL 输出 API 返回的错误信息并以非零状态码退出

### Requirement: List 子命令
`VideoCommand::List` SHALL 支持按条件过滤和分页查询视频生成任务。

#### Scenario: 无过滤条件列表
- **WHEN** 用户执行 `video list`
- **THEN** SHALL 调用 ARK API 列出任务，显示总数和每个任务的摘要信息

#### Scenario: 按状态过滤
- **WHEN** 用户指定 `--status succeeded`
- **THEN** SHALL 仅返回状态为 `succeeded` 的任务
- **AND** 状态值 SHALL 通过 clap `value_parser` 限制为 `queued`、`running`、`succeeded`、`failed`、`cancelled`，传入非法值时 clap 直接报错

#### Scenario: 按模型过滤
- **WHEN** 用户指定 `--model doubao-seedance-2-0-260128`
- **THEN** SHALL 仅返回使用该模型的任务

#### Scenario: 按 task ID 过滤
- **WHEN** 用户指定 `--task-ids id1 id2`
- **THEN** SHALL 仅返回指定 ID 的任务

#### Scenario: 分页参数
- **WHEN** 用户指定 `--page 2 --page-size 20`
- **THEN** SHALL 请求第 2 页、每页 20 条
- **AND** 默认值 SHALL 为 page=1、page-size=10

### Requirement: Delete 子命令
`VideoCommand::Delete` SHALL 接受一个 `task_id` 参数，删除指定的视频生成任务。

#### Scenario: 删除成功
- **WHEN** 用户执行 `video delete <task-id>`
- **THEN** SHALL 调用 ARK API 删除任务，打印确认信息

#### Scenario: 删除失败
- **WHEN** 指定的 task_id 不存在或无法删除
- **THEN** SHALL 输出 API 返回的错误信息并以非零状态码退出
