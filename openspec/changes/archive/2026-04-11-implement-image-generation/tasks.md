## 1. Core Retry 组件

- [x] 1.1 创建 `crates/tapsvc-aigc-core/` crate 骨架（`Cargo.toml`、`src/lib.rs`），加入 workspace members，更新 workspace `Cargo.toml` 中 `tapsvc-aigc-core` 依赖声明
- [x] 1.2 定义 `RetryConfig` 结构体（base_delay、factor、max_jitter、max_retries、max_delay）和 `Default` 实现（2s/2/1s/3/30s）
- [x] 1.3 定义 `Retryable` trait（`is_retryable()`、`retry_after()`）和 `is_retryable_status()` 辅助函数（429/500/502/503/504）
- [x] 1.4 实现泛型 `retry()` 异步执行器：指数退避 + jitter + Retry-After 优先 + 延迟上限。`max_retries` 语义为重试次数，总尝试 = 1 + max_retries
- [x] 1.5 在 workspace `Cargo.toml` 中添加 `rand` 依赖声明

## 2. OpenAI Client Image 模块

- [x] 2.1 在 `tapsvc-aigc-openai/Cargo.toml` 中添加 `tapsvc-aigc-core` 依赖
- [x] 2.2 在 `tapsvc-aigc-openai/src/` 中创建 `image.rs`，定义 `CreateImageRequest`（Serialize）、`ImageResponse` 和 `ImageData`（Deserialize）类型
- [x] 2.3 为 `Error` 枚举实现 `Retryable` trait（网络错误可重试、API 错误按状态码判断、反序列化错误不可重试）；`Error::Api` 增加 `retry_after: Option<Duration>` 字段，从响应 header 提取
- [x] 2.4 在 `OpenAiClient` 上实现 `create_image` 异步方法：通过 core `retry()` 执行器发送请求到 `{base_url}/v1/images/generations`
- [x] 2.5 在 `lib.rs` 中声明并导出 `image` 模块

## 3. CLI prompt 合并逻辑

- [x] 3.1 在 `cmd/image.rs` 中实现 prompt 解析函数：读取 `--prompt-file`（如有）、拼接 `--prompt`（如有）、两者都缺时报错
- [x] 3.2 处理 `--prompt-file` 文件不存在的错误情况

## 4. 输出文件命名

- [x] 4.1 在 workspace `Cargo.toml` 中添加 `chrono` 依赖声明（`default-features = false, features = ["clock", "alloc"]`）
- [x] 4.2 实现自动命名函数：无 `-o` 时生成 `image_YYYYMMDD_HHMMSS[_N].{ext}`，有 `-o` + n>1 时拆分 stem/ext 编号
- [x] 4.3 从 `--response-format` 推断文件扩展名

## 5. CLI 命令处理主流程

- [x] 5.1 修改 `cli.rs` 中 `--n` 参数添加 `value_parser = clap::value_parser!(u32).range(1..=10)`
- [x] 5.2 在 `cmd/image.rs` 的 `handle` 函数中组装完整流程：构造 `OpenAiClient`、合并 prompt、构建 `CreateImageRequest`、调用 API
- [x] 5.3 实现 base64 解码 + 文件写入逻辑：单个 b64_json 为 None 时跳过并警告，全部为 None 时报错退出
- [x] 5.4 输出文件路径到 stdout，输出 `revised_prompt` 到 stderr

## 6. 文档同步

- [x] 6.1 更新 `docs/design.md`：所有 OpenAI 端点路径加 `/v1/` 前缀（`{base_url}/v1/images/generations`、`{base_url}/v1/audio/speech`），ARK 端点改为 `{base_url}/volcengine/api/v3/...`
- [x] 6.2 更新 `docs/design.md`：prompt 和 prompt_file 由"二选一"改为"可拼接"说明
- [x] 6.3 更新 `docs/design.md`：错误处理表格补充重试策略细节（可重试状态码、退避参数 2s/4s/8s、总尝试 4 次）
- [x] 6.4 更新 `docs/design.md`：workspace 结构补充 `tapsvc-aigc-core` crate

## 7. 验证

- [x] 7.1 运行 `cargo build` 确认编译通过
- [x] 7.2 运行 `cargo fmt --all -- --check` 和 `cargo clippy --all-targets -- -D warnings` 确认代码质量
