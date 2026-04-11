## MODIFIED Requirements

### Requirement: CreateImageRequest 类型定义
SHALL 定义 `CreateImageRequest` 结构体，使用 `serde::Serialize` 派生。

#### Scenario: 必填字段
- **WHEN** 构建 `CreateImageRequest`
- **THEN** `model` 和 `prompt` 字段 SHALL 为 `String` 类型，必须提供

#### Scenario: 可选字段
- **WHEN** 构建 `CreateImageRequest`
- **THEN** SHALL 包含以下 `Option` 字段：`n: Option<u32>`、`size: Option<String>`、`quality: Option<String>`、`response_format: Option<String>`、`background: Option<String>`、`output_format: Option<String>`
- **AND** 可选字段序列化时 SHALL 跳过 `None` 值（`#[serde(skip_serializing_if = "Option::is_none")]`）
