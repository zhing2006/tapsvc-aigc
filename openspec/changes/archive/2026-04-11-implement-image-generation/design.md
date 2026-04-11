## Context

CLI 骨架已搭建完成（cli.rs 参数定义、cmd/image.rs `todo!()` 占位），`tapsvc-aigc-openai` crate 有 `OpenAiClient` 结构体但尚无 image 模块。需要实现从 CLI 参数到 API 调用再到文件输出的完整链路。

项目设计文档 `docs/design.md` 已明确 API 端点格式。`base_url` 不含 `/v1` 前缀：
- Image/Audio (OpenAI 兼容): `{base_url}/v1/images/generations`、`{base_url}/v1/audio/speech`
- Video (Volcengine ARK): `{base_url}/volcengine/api/v3/contents/generations/tasks`

## Goals / Non-Goals

**Goals:**

- 新增 `tapsvc-aigc-core` crate，提供通用 retry 执行器
- 实现 `tapsvc-aigc-openai` crate 中的 image generation API 客户端，集成 retry
- 实现 `cmd/image.rs` 中完整的命令处理逻辑
- 支持 prompt + prompt_file 拼接合并
- 支持多图输出自动编号和缺省时间戳命名
- 同步更新 `docs/design.md` 修正端点路径和 prompt 说明

**Non-Goals:**

- 不做 CLI 层 String 参数枚举校验（size、quality、background 交给 API 端校验）
- 不实现 image edit / image variation 端点
- 不引入 async-openai 等第三方 OpenAI SDK

## Decisions

### Decision 1: 手写 API 客户端，不用 async-openai

**选择**: 在 `tapsvc-aigc-openai` 中手写 `image` 模块

**备选**: 使用 `async-openai` crate（v0.31.0-alpha）

**理由**:
- async-openai 当前为 alpha 版本，API 不稳定
- 其 enum 类型体系（`ImageSize`、`ImageQuality`）与"参数交给 API 校验"的设计矛盾
- image generations 端点本身非常简单（一个 POST + JSON），手写约 50-80 行
- 避免引入 derive_builder、backoff 等不必要的传递依赖

### Decision 2: prompt 和 prompt_file 拼接合并

**选择**: 两者不互斥，拼接为最终 prompt（file 内容在前 + `\n` + prompt 文本在后）

**备选**: 互斥（二选一）

**理由**:
- prompt_file 适合放大段风格/背景描述，prompt 适合写具体指令
- 先背景后指令的顺序更符合 LLM 理解习惯
- 至少需要提供其一，两者都缺时报错

> 注意：`docs/design.md` 原先描述为"二选一"，需同步更新为可拼接语义。

### Decision 3: 参数以 String 直传 API，`--n` 例外

**选择**: CLI 层 `size`、`quality`、`background`、`response_format` 保持 `String` 类型，直接序列化到请求 JSON。但 `--n` 在 CLI 层约束为 1-10（`clap::value_parser!(u32).range(1..=10)`）。

**备选**: 全部不校验，或全部用 enum

**理由**:
- `size`/`quality`/`background` 的合法值因模型而异，交给 API 合理
- `--n` 是所有模型通用的范围约束（1-10），0 或超大值无意义，不值得留给服务端兜底
- clap 原生 range 校验一行搞定，无额外代码负担

### Decision 4: 自动命名规则 + chrono 依赖

**选择**: 基于时间戳的自动命名，新增 `chrono` crate（`default-features = false`, features = `["clock", "alloc"]`）

- 无 `-o` 时: `image_YYYYMMDD_HHMMSS.{ext}`（n=1）或 `image_YYYYMMDD_HHMMSS_{N}.{ext}`（n>1）
- 有 `-o` + n=1: 直接使用指定文件名
- 有 `-o` + n>1: 拆分 stem/ext 后编号，如 `cat_1.png`、`cat_2.png`

**备选**: 使用 epoch 时间戳（`image_1744364252.png`），无额外依赖

**理由**: epoch 可读性差；chrono 是 Rust 生态最基础的库。`clock` 提供 `Local::now()`，`alloc` 提供字符串格式化能力。ext 从 `--response-format` 推断。

### Decision 5: 请求/响应类型结构

**选择**: 在 `tapsvc-aigc-openai` crate 中用 serde derive 定义纯数据类型

```
CreateImageRequest {
    model: String,
    prompt: String,
    n: Option<u32>,
    size: Option<String>,
    quality: Option<String>,
    response_format: Option<String>,
    background: Option<String>,
}

ImageResponse {
    created: u64,
    data: Vec<ImageData>,
}

ImageData {
    b64_json: Option<String>,
    revised_prompt: Option<String>,
}
```

`OpenAiClient` 新增方法: `async fn create_image(&self, req: &CreateImageRequest) -> Result<ImageResponse, Error>`

方法内部: 构建 URL (`{base_url}/v1/images/generations`)、设置 Authorization header、POST JSON、通过 core retry 执行器自动重试、解析响应。

### Decision 6: 新增 tapsvc-aigc-core crate — 通用 retry 组件

**选择**: 新增 workspace 成员 `crates/tapsvc-aigc-core`，提供泛型 retry 执行器。本次 change 先让 openai client 依赖它；ark 在后续 video change 中接入。

**备选 A**: retry 逻辑只放在 openai crate 中，ark 后续再抄一份
**备选 B**: retry 逻辑只提供 RetryConfig + delay 计算工具，各 client 自己写循环

**理由**:
- openai 和 ark 都需要 retry，抽到 core 避免重复
- 泛型执行器比纯工具方法更 DRY，各 client 不需要各写一遍 retry loop

**retry 参数:**
- base_delay: 2s
- factor: 2（指数底数）
- max_jitter: 可配置（默认 1s）
- max_retries: 3（最多重试 3 次，即总尝试次数 = 1 次初始 + 3 次重试 = 4 次）
- max_delay: 30s
- 实际延迟序列: 2s → 4s → 8s（+ jitter）
- 429 有 `Retry-After` header 时优先使用其值

**可重试状态码**: 429, 500, 502, 503, 504
**可重试网络错误**: 连接超时、DNS 失败、连接重置等 reqwest 网络错误
**不可重试**: 除 429 外的所有 4xx

**API 形态:**

```rust
pub struct RetryConfig {
    pub base_delay: Duration,
    pub factor: u32,
    pub max_jitter: Duration,
    pub max_retries: u32,
    pub max_delay: Duration,
}

/// 泛型 retry 执行器
/// E 须实现 Retryable trait 以判断是否可重试 + 提取 retry_after
pub async fn retry<F, Fut, T, E>(
    config: &RetryConfig,
    operation: F,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: Retryable,

pub trait Retryable {
    fn is_retryable(&self) -> bool;
    fn retry_after(&self) -> Option<Duration>;
}
```

### Decision 7: 全部 b64_json 为 None 时报错退出

**选择**: 单个 `b64_json` 为 None 时跳过并警告（stderr），但如果最终零个文件写出，报错退出（非零 exit code）。

**备选**: 始终 exit 0

**理由**: API 返回 200 但无有效图片时，exit 0 + stdout 空对脚本调用不友好。

### Decision 8: 端点路径统一

**选择**: `base_url` 不含 `/v1` 前缀。OpenAI 兼容端点拼接为 `{base_url}/v1/images/generations`。

**理由**: `base_url` 始终为代理根地址（如 `https://llm-proxy.tapsvc.com`），各 client 自行拼接具体路径。OpenAI 兼容端点拼 `/v1/...`，ARK 端点拼 `/volcengine/api/v3/...`。三种 API 共享同一个 `TAPSVC_BASE_URL` 环境变量。

> `docs/design.md` 中所有 `{base_url}/images/generations` 需修正为 `{base_url}/v1/images/generations`，`{base_url}/audio/speech` 同理。

## Risks / Trade-offs

- **[无 CLI String 参数校验]** → 用户传错 size/quality/background 只能靠 API 报错。API 错误信息需要清晰地传递给用户。
- **[base64 内存占用]** → 大图（如 4K）的 base64 数据可能占用较多内存。当前可接受，多图（n=10）时单进程峰值可能达数百 MB。暂不优化，后续可考虑流式解码。
- **[时间戳命名冲突]** → 同一秒内多次调用可能冲突。概率极低，暂不处理。
- **[retry 隐藏延迟]** → 最坏 3 次重试累计约 14s + jitter。对图片生成（本身就要数秒到十几秒）可接受，但需要在 stderr 输出重试信息让用户知道在等什么。
