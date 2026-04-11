# tapsvc-aigc CLI 设计文档

## 1. 项目概述

tapsvc-aigc 是一个 Rust CLI 工具，通过 OpenAI 兼容代理调用多种 AIGC 能力：

- **图片生成** — 通过 `/v1/images/generations` 端点
- **语音合成 (TTS)** — 通过 `/v1/audio/speech` 端点
- **视频生成** — 通过 Volcengine ARK API (Seedance 2.0)

## 2. 支持的模型

### 2.1 图片生成 (`/v1/images/generations`)

| 模型 | 提供商 | 说明 |
|------|--------|------|
| `gpt-image-1.5` | OpenAI | OpenAI 最新图片生成模型 |
| `gemini-3-pro-image-preview` | Google | Gemini 3 Pro 图片生成 |
| `gemini-3.1-flash-image-preview` | Google | Gemini 3.1 Flash 图片生成 |

### 2.2 语音合成 (`/v1/audio/speech`)

| 模型 | 提供商 | 说明 |
|------|--------|------|
| `elevenlabs/eleven_multilingual_v2` | ElevenLabs | 多语言 TTS，支持长文本（最多 10000 字符） |
| `elevenlabs/eleven_v3` | ElevenLabs | 最新一代 TTS，最大表现力 |

### 2.3 视频生成 (Volcengine ARK)

| 模型 | Model ID | 说明 |
|------|----------|------|
| Seedance 2.0 | `doubao-seedance-2-0-260128` | 标准版，质量最优 |
| Seedance 2.0 Lite | `doubao-seedance-2-0-fast-260128` | 快速版，速度优先 |

> 视频生成走 Volcengine ARK 原生 API，非 OpenAI 兼容端点。

## 3. CLI 命令设计

```
tapsvc-aigc [全局选项] <子命令> [参数]
```

### 3.1 全局选项

| 选项 | 环境变量 | 说明 |
|------|----------|------|
| — | `TAPSVC_BASE_URL` | API 代理地址（所有 API 共用） |
| — | `TAPSVC_API_KEY` | API Key（所有 API 共用） |
| `--output, -o <PATH>` | — | 输出文件路径 |

> `TAPSVC_BASE_URL` 和 `TAPSVC_API_KEY` 通过环境变量或 `.env` 文件配置，三个 API（image、audio、video）共享同一配置。程序为单文件分发，无配置文件，默认值硬编码在代码中。

### 3.2 图片生成

```bash
# 基本用法
tapsvc-aigc image generate --model gpt-image-1.5 --prompt "a cat in space" -o cat.png

# 指定尺寸和数量
tapsvc-aigc image generate \
  --model gemini-3-pro-image-preview \
  --prompt "a cyberpunk city" \
  --size 1024x1024 \
  --n 2 \
  -o city.png

# 从文件读取 prompt
tapsvc-aigc image generate --model gemini-3.1-flash-image-preview --prompt-file prompt.txt -o result.png
```

**子命令参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--model, -m` | 是 | — | 模型名称 |
| `--prompt, -p` | 是* | — | 生成提示词 |
| `--prompt-file` | 否 | — | 从文件读取提示词，与 `--prompt` 二选一 |
| `--size` | 否 | `1024x1024` | 图片尺寸 (`1024x1024`, `1536x1024`, `1024x1536`, `auto`) |
| `--n` | 否 | `1` | 生成数量 (1-10) |
| `--quality` | 否 | `auto` | 质量级别 (`auto`, `high`, `medium`, `low`) |
| `--response-format` | 否 | `png` | 输出图片格式 (`png`, `jpeg`, `webp`) |
| `--background` | 否 | `auto` | 背景类型 (`transparent`, `opaque`, `auto`) |
| `--output, -o` | 否 | 当前目录自动命名 | 输出文件路径 |

### 3.3 语音合成

```bash
# 基本用法
tapsvc-aigc audio speech --model elevenlabs/eleven_multilingual_v2 --voice alloy --input "Hello world" -o hello.mp3

# 从文件读取文本
tapsvc-aigc audio speech \
  --model elevenlabs/eleven_v3 \
  --voice shimmer \
  --input-file article.txt \
  --format mp3 \
  -o speech.mp3

# 指定语速
tapsvc-aigc audio speech --model elevenlabs/eleven_multilingual_v2 --voice echo --speed 1.2 --input "快速阅读" -o fast.mp3
```

**子命令参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--model, -m` | 是 | — | 模型名称 |
| `--voice` | 是 | — | 语音名称 |
| `--input, -i` | 是* | — | 要转换的文本 |
| `--input-file` | 否 | — | 从文件读取文本，与 `--input` 二选一 |
| `--format` | 否 | `mp3` | 输出格式 (`mp3`, `opus`, `aac`, `flac`, `wav`, `pcm`) |
| `--speed` | 否 | `1.0` | 语速 (0.25 - 4.0) |
| `--instructions` | 否 | — | 语气风格控制 (如 `"Speak in a cheerful tone"`) |
| `--output, -o` | 否 | 当前目录自动命名 | 输出文件路径 |

### 3.4 视频生成

```bash
# 文生视频
tapsvc-aigc video generate \
  --model seedance-2.0 \
  --prompt "a dog running on the beach, cinematic" \
  --duration 5 \
  --resolution 1080p \
  -o dog.mp4

# 图生视频
tapsvc-aigc video generate \
  --model seedance-2.0-lite \
  --prompt "make the character walk forward" \
  --image reference.jpg \
  --duration 5 \
  -o walk.mp4

# 首尾帧控制
tapsvc-aigc video generate \
  --model seedance-2.0 \
  --prompt "smooth transition" \
  --first-frame start.jpg \
  --last-frame end.jpg \
  -o transition.mp4
```

**子命令参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--model, -m` | 是 | — | 模型名称 (`seedance-2.0`, `seedance-2.0-lite`) |
| `--prompt, -p` | 是 | — | 生成提示词 |
| `--prompt-file` | 否 | — | 从文件读取提示词 |
| `--image` | 否 | — | 参考图片（图生视频） |
| `--first-frame` | 否 | — | 首帧图片 |
| `--last-frame` | 否 | — | 尾帧图片 |
| `--duration` | 否 | `5` | 视频时长，4-15 秒 |
| `--resolution` | 否 | `1080p` | 分辨率 (`480p`, `720p`, `1080p`, `2K`) |
| `--aspect-ratio` | 否 | `16:9` | 宽高比 (`1:1`, `16:9`, `9:16`, `4:3`, `3:4`, `21:9`, `adaptive`) |
| `--watermark` | 否 | `false` | 添加水印 |
| `--generate-audio` | 否 | `false` | 同步生成音频 |
| `--poll-interval` | 否 | `5` | 轮询间隔（秒） |
| `--timeout` | 否 | `300` | 超时时间（秒） |
| `--output, -o` | 否 | 当前目录自动命名 | 输出文件路径 |

## 4. API 对接细节

### 4.1 图片生成 — OpenAI 兼容

通过代理直接调用，请求/响应格式均为标准 OpenAI 格式。

**请求：**
```json
POST {base_url}/images/generations
Authorization: Bearer {api_key}

{
  "model": "gpt-image-1.5",
  "prompt": "a cat in space",
  "n": 1,
  "size": "1024x1024",
  "quality": "auto",
  "response_format": "png",
  "background": "auto"
}
```

> GPT image 模型始终返回 base64 编码的图片数据。`response_format` 指定输出图片格式（`png`/`jpeg`/`webp`），而非传输格式。

**响应：**
```json
{
  "created": 1234567890,
  "data": [
    {
      "b64_json": "iVBORw0KGgo...",
      "revised_prompt": "..."
    }
  ]
}
```

**CLI 逻辑：**
1. 构建请求体，POST 到 `{base_url}/images/generations`
2. 解析响应，base64 解码 `b64_json` 后写入文件
3. 多张图片时输出为 `output_1.png`, `output_2.png`, ...

### 4.2 语音合成 — OpenAI 兼容

通过代理调用，代理负责转换为 ElevenLabs 原生格式。

**请求：**
```json
POST {base_url}/audio/speech
Authorization: Bearer {api_key}

{
  "model": "elevenlabs/eleven_multilingual_v2",
  "input": "Hello, world!",
  "voice": "alloy",
  "response_format": "mp3",
  "speed": 1.0,
  "instructions": "Speak in a cheerful tone"
}
```

> `instructions` 为可选字段，用于控制语气、情绪等风格。

**响应：** 二进制音频流 (`application/octet-stream`)

**CLI 逻辑：**
1. 构建请求体，POST 到 `{base_url}/audio/speech`
2. 以流式方式接收响应 body
3. 直接写入输出文件

### 4.3 视频生成 — Volcengine ARK API

Seedance 2.0 API 使用异步任务模式，通过统一代理访问。

**模型名称映射：**
```
seedance-2.0      → doubao-seedance-2-0-260128
seedance-2.0-lite → doubao-seedance-2-0-fast-260128
```

**步骤 1 — 提交任务：**
```json
POST {base_url}/contents/generations/tasks
Authorization: Bearer {api_key}

{
  "model": "doubao-seedance-2-0-260128",
  "content": [
    {
      "type": "text",
      "text": "a dog running on the beach"
    }
  ],
  "duration": 5,
  "resolution": "1080p",
  "ratio": "16:9",
  "watermark": false,
  "generate_audio": false
}
```

**步骤 2 — 轮询状态：**
```
GET {base_url}/contents/generations/tasks/{task_id}
Authorization: Bearer {api_key}
```

**步骤 3 — 下载视频：**
任务状态变为 `succeeded` 后，从响应中提取 `video_url`，下载保存。

**CLI 逻辑：**
1. 提交生成任务，获取 `task_id`
2. 按 `--poll-interval` 轮询任务状态，显示进度
3. 状态变为 `succeeded` 后下载视频到输出路径
4. 超过 `--timeout` 则报错退出
5. 注意：`video_url` 24 小时内有效

## 5. Workspace 与 Crate 结构

项目采用 Cargo workspace，拆分为 3 个 crate，Rust edition 统一使用 **2024**。

### 5.1 Workspace 布局

```
tapsvc-aigc/
├── Cargo.toml                    # workspace root
├── docs/
│   └── design.md
├── crates/
│   ├── tapsvc-aigc/              # CLI 二进制 crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs           # 入口，tokio::main + clap 解析
│   │       ├── cli.rs            # CLI 命令定义（clap derive）
│   │       └── cmd/
│   │           ├── mod.rs
│   │           ├── image.rs      # image 子命令处理
│   │           ├── audio.rs      # audio 子命令处理
│   │           └── video.rs      # video 子命令处理
│   │
│   ├── tapsvc-aigc-openai/       # OpenAI 兼容客户端 library crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # 公开 API
│   │       ├── client.rs         # OpenAI HTTP 客户端（base_url + api_key）
│   │       ├── error.rs          # 错误类型定义
│   │       ├── image.rs          # /v1/images/generations 请求/响应类型 + 调用
│   │       └── audio.rs          # /v1/audio/speech 请求/响应类型 + 调用
│   │
│   └── tapsvc-aigc-ark/          # Volcengine ARK 客户端 library crate
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs            # 公开 API
│           ├── client.rs         # ARK HTTP 客户端（base_url + api_key）
│           ├── error.rs          # 错误类型定义
│           └── video.rs          # 视频生成：提交任务、轮询、类型定义
│
├── LICENSE
└── README.md
```

### 5.2 Workspace Cargo.toml

```toml
[workspace]
resolver = "3"
members = [
    "crates/tapsvc-aigc",
    "crates/tapsvc-aigc-openai",
    "crates/tapsvc-aigc-ark",
]

[workspace.package]
edition = "2024"
license = "Apache-2.0"
repository = "https://github.com/zhing2006/tapsvc-aigc"

[workspace.dependencies]
# 异步运行时
tokio = { version = "1", default-features = false, features = ["rt-multi-thread", "macros", "time", "fs", "signal"] }

# HTTP 客户端
reqwest = { version = "0.13", default-features = false, features = ["rustls", "json", "stream"] }

# 序列化
serde = { version = "1", default-features = false, features = ["derive"] }
serde_json = { version = "1", default-features = false }

# 错误处理
thiserror = { version = "2.0", default-features = false }
anyhow = { version = "1", default-features = false }

# 日志
tracing = { version = "0.1", default-features = false }
tracing-subscriber = { version = "0.3", default-features = false, features = ["env-filter", "fmt"] }

# CLI
clap = { version = "4.6", default-features = false, features = ["std", "derive", "env", "help", "usage", "error-context"] }
dotenvy = { version = "0.15", default-features = false }
indicatif = { version = "0.18", default-features = false }
base64 = { version = "0.22", default-features = false }

# workspace 内部 crate
tapsvc-aigc-openai = { path = "crates/tapsvc-aigc-openai" }
tapsvc-aigc-ark = { path = "crates/tapsvc-aigc-ark" }
```

### 5.3 各 Crate 依赖

**`tapsvc-aigc`** (CLI binary)

```toml
[package]
name = "tapsvc-aigc"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
tapsvc-aigc-openai.workspace = true
tapsvc-aigc-ark.workspace = true
tokio.workspace = true
serde.workspace = true
serde_json.workspace = true
anyhow.workspace = true
tracing.workspace = true
tracing-subscriber.workspace = true
clap.workspace = true
dotenvy.workspace = true
indicatif.workspace = true
base64.workspace = true
```

**`tapsvc-aigc-openai`** (OpenAI 兼容客户端)

```toml
[package]
name = "tapsvc-aigc-openai"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
reqwest.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tracing.workspace = true
tokio = { workspace = true, features = ["time"] }
```

**`tapsvc-aigc-ark`** (Volcengine ARK 客户端)

```toml
[package]
name = "tapsvc-aigc-ark"
version = "0.1.0"
edition.workspace = true
license.workspace = true

[dependencies]
reqwest.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tracing.workspace = true
tokio = { workspace = true, features = ["time"] }
```

### 5.4 Crate 职责划分

| Crate | 类型 | 职责 |
|-------|------|------|
| `tapsvc-aigc` | binary | CLI 交互、参数解析、文件 I/O、进度展示 |
| `tapsvc-aigc-openai` | library | OpenAI 兼容 API 的类型定义和 HTTP 调用（images + audio） |
| `tapsvc-aigc-ark` | library | Volcengine ARK API 的类型定义和 HTTP 调用（video） |

> **设计原则**：两个 library crate 不依赖 CLI 相关 crate（clap、indicatif 等），
> 保持纯粹的 API 客户端职责，可被其他项目复用。

## 6. 错误处理

| 场景 | 处理方式 |
|------|----------|
| 网络错误 / 代理不可达 | 重试 3 次，指数退避，最终报错退出 |
| API 返回 4xx | 解析错误信息，展示给用户 |
| API 返回 429 (限流) | 重试，使用 `Retry-After` header |
| 视频生成超时 | 输出 task_id，告知用户可稍后手动查询 |
| 输出文件已存在 | 默认覆盖，可加 `--no-overwrite` 选项 |
| 视频 URL 过期 | 提示 24 小时有效期限制 |

## 7. 输出行为

- 默认将进度信息输出到 stderr（不干扰 stdout 重定向）
- 生成成功后在 stdout 打印输出文件路径
- `--quiet` 选项抑制进度信息
- `--verbose` 选项输出详细请求/响应日志
