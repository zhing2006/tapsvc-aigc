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
| `gpt-image-2` | OpenAI | 新一代图片生成模型（默认）— 文本渲染、多语种、色彩中性度更好；仅接受 `size=auto`，不支持 `background=transparent` |
| `gpt-image-1.5` | OpenAI | 上一代旗舰；需要参数级 `background=transparent` 或固定尺寸时使用 |
| `gemini-3-pro-image-preview` | Google | Gemini 3 Pro 图片生成 |
| `gemini-3.1-flash-image-preview` | Google | Gemini 3.1 Flash 图片生成 |

> **gpt-image-2 限制：**
> - **`size` 仅接受 `auto`** — 路由层自动选择尺寸，显式 `WxH` 会被代理拒绝；通过 prompt 措辞（"square composition" / "portrait" / "wide landscape 16:9"）引导比例
> - **`background` 不支持 `transparent`** — 透明背景请写进 prompt（"transparent background, isolated subject on alpha channel"），或回退到 `gpt-image-1.5`
> - **响应可能为 URL** — 某些代理路由会把 gpt-image-2 输出放到对象存储并仅回 `url`，CLI 会自动下载

> **LiteLLM 代理下 Gemini 模型的已知限制：**
> - **`n` 只支持 1** — Gemini 图片生成仅支持单张输出，`n > 1` 会返回 400 错误
> - **`size` 映射为 `aspectRatio`** — LiteLLM 将 `WxH` 格式自动转换为 Gemini 的 `aspectRatio`（如 `1792x1024` → `16:9`、`1536x1024` → `3:2`），此功能已在 [litellm#18948](https://github.com/BerriAI/litellm/pull/18948) 中修复（仅 Google AI Studio 路径，VertexAI 路径仍有问题）
> - **`mask` 不支持** — Gemini 无原生 mask 概念，image edit 时传入 `--mask` 会被 LiteLLM 拒绝（[litellm#17719](https://github.com/BerriAI/litellm/issues/17719)），CLI 不做模型级拦截，Gemini 编辑仅通过 prompt 描述

### 2.2 语音合成 (`/v1/audio/speech`)

| 模型 | 提供商 | 说明 |
|------|--------|------|
| `elevenlabs/eleven_multilingual_v2` | ElevenLabs | 多语言 TTS，支持长文本（最多 10000 字符） |
| `elevenlabs/eleven_v3` | ElevenLabs | 最新一代 TTS，最大表现力（最多 3000 字符） |

> **Voice 名称：** LiteLLM 内置了 OpenAI 声音名称到 ElevenLabs Voice ID 的映射。用户也可以直接传入 ElevenLabs Voice ID，LiteLLM 会原样透传。
>
> | OpenAI Voice | ElevenLabs Voice ID | 描述 |
> |---|---|---|
> | `alloy` | `21m00Tcm4TlvDq8ikWAM` | Rachel — 中性 |
> | `amber` | `5Q0t7uMcjvnagumLfvZi` | Paul — 温暖 |
> | `ash` | `AZnzlk1XvdvUeBnXmlld` | Domi — 有活力 |
> | `august` | `D38z5RcWu1voky8WS1ja` | Fin — 专业 |
> | `blue` | `2EiwWnXFnvU5JabPnv8n` | Clyde — 低沉 |
> | `coral` | `9BWtsMINqrJLrRacOk9x` | Aria — 有表现力 |
> | `lily` | `EXAVITQu4vr4xnSDxMaL` | Sarah — 友好 |
> | `onyx` | `29vD33N1CtxCmqQRPOHJ` | Drew — 有力 |
> | `sage` | `CwhRBWXzGAHq8TQ4Fs17` | Roger — 沉稳 |
> | `verse` | `CYw3kZ02Hs0563khs1Fj` | Dave — 对话感 |
>
> **输出格式映射：** LiteLLM 自动将 OpenAI 格式名转换为 ElevenLabs 原生格式（`mp3` → `mp3_44100_128`、`pcm` → `pcm_44100`、`opus` → `opus_48000_128`）。`aac`、`flac`、`wav` 的支持取决于 LiteLLM 版本。

### 2.3 视频生成 (Volcengine ARK)

视频生成走 Volcengine ARK 原生 API（非 OpenAI 兼容端点），通过 `-m` 参数直接传入 API model ID：

- `doubao-seedance-2-0-260128` — 标准版，质量最优
- `doubao-seedance-2-0-fast-260128` — 快速版，速度优先

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
| `--prompt-file` | 否 | — | 从文件读取提示词，可与 `--prompt` 同时使用（file 内容在前拼接） |
| `--size` | 否 | `auto` | 图片尺寸 (`auto`, `1024x1024`, `1536x1024`, `1024x1536`)。`auto` 时不发送字段，由代理/模型决定；**gpt-image-2 仅接受 `auto`** |
| `--n` | 否 | `1` | 生成数量 (1-10) |
| `--quality` | 否 | `auto` | 质量级别 (`auto`, `high`, `medium`, `low`) |
| `--response-format` | 否 | `png` | 输出图片格式 (`png`, `jpeg`, `webp`) |
| `--background` | 否 | `auto` | 背景类型 (`transparent`, `opaque`, `auto`) |
| `--output, -o` | 否 | 当前目录自动命名 | 输出文件路径 |

### 3.3 图片编辑

```bash
# 基本用法 — 根据 prompt 编辑图片
tapsvc-aigc image edit --model gpt-image-1.5 --image input.png --prompt "add a hat to the person" -o edited.png

# 带 mask 的局部编辑
tapsvc-aigc image edit \
  --model gpt-image-1.5 \
  --image input.png \
  --mask mask.png \
  --prompt "replace the background with a beach" \
  -o beach.png

# 从文件读取 prompt
tapsvc-aigc image edit --model gpt-image-1.5 --image input.png --prompt-file edit_instructions.txt -o result.png
```

**子命令参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--model, -m` | 是 | — | 模型名称 |
| `--image` | 是 | — | 输入图片路径（PNG/JPEG/WebP，< 25MB） |
| `--prompt, -p` | 是* | — | 编辑提示词 |
| `--prompt-file` | 否 | — | 从文件读取提示词，可与 `--prompt` 同时使用（file 内容在前拼接） |
| `--mask` | 否 | — | 编辑区域蒙版（PNG，< 4MB，透明区域为编辑区域；仅 gpt-image-1.5 支持，Gemini 不支持。CLI 仅校验格式和大小，不在本地校验是否与输入图片同尺寸） |
| `--size` | 否 | `auto` | 输出图片尺寸（`auto` 时不发送字段；**gpt-image-2 仅接受 `auto`**） |
| `--n` | 否 | `1` | 生成数量 (1-10) |
| `--response-format` | 否 | `png` | 输出图片格式 (`png`, `jpeg`, `webp`) |
| `--output, -o` | 否 | 当前目录自动命名 | 输出文件路径 |

### 3.4 语音合成

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

# 调整 stability（低值更富表现力，高值更稳定）
tapsvc-aigc audio speech --model elevenlabs/eleven_v3 --voice alloy --stability 0.3 --similarity 0.8 --input "dramatic reading" -o expressive.mp3
```

**子命令参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--model, -m` | 是 | — | 模型名称 |
| `--voice` | 是 | — | 语音名称（OpenAI 映射名如 `alloy`、`coral` 等，或 ElevenLabs Voice ID） |
| `--input, -i` | 是* | — | 要转换的文本 |
| `--input-file` | 否 | — | 从文件读取文本，与 `--input` 二选一 |
| `--format` | 否 | `mp3` | 输出格式 (`mp3`, `opus`, `aac`, `flac`, `wav`, `pcm`) |
| `--speed` | 否 | `1.0` | 语速 |
| `--stability` | 否 | — | 语音稳定性（0.0-1.0，低值更富表现力，高值更稳定） |
| `--similarity` | 否 | — | 声音相似度（0.0-1.0，高值更接近原始音色） |
| `--output, -o` | 否 | 当前目录自动命名 | 输出文件路径 |

### 3.5 视频生成

#### 3.5.1 video generate

```bash
# 文生视频
tapsvc-aigc video generate \
  -m doubao-seedance-2-0-260128 \
  --prompt "a dog running on the beach, cinematic" \
  --duration 5 \
  --resolution 720p \
  -o dog.mp4

# 首帧图生视频
tapsvc-aigc video generate \
  -m doubao-seedance-2-0-fast-260128 \
  --prompt "make the character walk forward" \
  --first-frame start.jpg \
  --duration 5 \
  -o walk.mp4

# 首尾帧控制
tapsvc-aigc video generate \
  -m doubao-seedance-2-0-260128 \
  --prompt "smooth transition" \
  --first-frame start.jpg \
  --last-frame end.jpg \
  -o transition.mp4

# 多参考图模式
tapsvc-aigc video generate \
  -m doubao-seedance-2-0-260128 \
  --prompt "combine these into a video" \
  --ref-image a.jpg --ref-image b.jpg \
  -o combined.mp4

# 从文件读取 prompt
tapsvc-aigc video generate \
  -m doubao-seedance-2-0-260128 \
  --prompt-file prompt.txt \
  -o result.mp4
```

**generate 参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--model, -m` | 是 | — | API model ID（直接透传） |
| `--prompt, -p` | 否* | — | 生成提示词 |
| `--prompt-file` | 否 | — | 从文件读取提示词，可与 `--prompt` 同时使用（file 在前拼接） |
| `--first-frame` | 否 | — | 首帧图片（图生视频，与 `--ref-image`/`--ref-video` 互斥） |
| `--last-frame` | 否 | — | 尾帧图片（需搭配 `--first-frame`） |
| `--ref-image` | 否 | — | 参考图片（可重复，最多 9 张，与 `--first-frame` 互斥） |
| `--ref-video` | 否 | — | 参考视频 URL（可重复，最多 3 个，仅支持 URL，与 `--first-frame` 互斥） |
| `--ref-audio` | 否 | — | 参考音频（可重复，最多 3 个，需搭配 `--ref-image` 或 `--ref-video`） |
| `--resolution` | 否 | `720p` | 分辨率（`480p`、`720p`） |
| `--aspect-ratio` | 否 | `adaptive` | 宽高比（`16:9`、`4:3`、`1:1`、`3:4`、`9:16`、`21:9`、`adaptive`） |
| `--duration` | 否 | `5` | 视频时长，4-15 秒或 -1 自动 |
| `--no-audio` | 否 | `false` | 禁用音频生成（默认生成音频） |
| `--watermark` | 否 | `false` | 添加水印 |
| `--web-search` | 否 | `false` | 启用网络搜索增强 |
| `--camera-fixed` | 否 | `false` | 固定镜头 |
| `--seed` | 否 | — | 随机种子 |
| `--poll-interval` | 否 | `10` | 轮询间隔（秒） |
| `--timeout` | 否 | `300` | 超时时间（秒） |
| `--output, -o` | 否 | `video_{timestamp}.mp4` | 输出文件路径 |

> *`--prompt`/`--prompt-file` 至少需要与 `--first-frame`、`--ref-image`、`--ref-video` 中的一种共同提供，或单独提供 prompt 进行文生视频。

#### 3.5.2 video get / list / delete

```bash
# 查询单个任务状态
tapsvc-aigc video get <task-id>

# 列出任务（支持过滤和分页）
tapsvc-aigc video list --status succeeded --page 1 --page-size 20

# 删除任务
tapsvc-aigc video delete <task-id>
```

**list 参数：**

| 参数 | 必须 | 默认值 | 说明 |
|------|------|--------|------|
| `--status, -s` | 否 | — | 按状态过滤（`queued`/`running`/`succeeded`/`failed`/`cancelled`） |
| `--model, -m` | 否 | — | 按模型过滤 |
| `--task-ids` | 否 | — | 按 task ID 过滤 |
| `--page, -p` | 否 | `1` | 页码 |
| `--page-size, -n` | 否 | `10` | 每页数量 |

## 4. API 对接细节

### 4.1 图片生成 — OpenAI 兼容

通过代理直接调用，请求/响应格式均为标准 OpenAI 格式。

**请求：**
```json
POST {base_url}/v1/images/generations
Authorization: Bearer {api_key}

{
  "model": "gpt-image-1.5",
  "prompt": "a cat in space",
  "n": 1,
  "size": "1024x1024",
  "quality": "auto",
  "response_format": "b64_json",
  "output_format": "png",
  "background": "auto"
}
```

> `response_format` 固定为 `"b64_json"`，确保返回 base64 编码的图片数据。`output_format` 控制实际图片编码格式（`png`/`jpeg`/`webp`），由 CLI 的 `--response-format` 参数值映射而来，作为 provider kwargs 透传给 OpenAI。

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
1. 构建请求体，POST 到 `{base_url}/v1/images/generations`
2. 解析响应，base64 解码 `b64_json` 后写入文件
3. 多张图片时，若指定 `--output result.png`，输出为 `result_1.png`, `result_2.png`, ...；未指定时自动命名为 `image_YYYYMMDD_HHMMSS_1.png`, `image_YYYYMMDD_HHMMSS_2.png`, ...

### 4.2 图片编辑 — OpenAI 兼容

通过代理调用，请求格式为 multipart/form-data（非 JSON）。

**请求：**
```
POST {base_url}/v1/images/edits
Authorization: Bearer {api_key}
Content-Type: multipart/form-data

model=gpt-image-1.5
image=@input.png
mask=@mask.png          (可选)
prompt=add a hat to the person
n=1
size=1024x1024
output_format=png
```

> **不发 `response_format` 字段** — gpt-image-1.5 的 edit 端点不接受 `response_format` 参数（仅 dall-e-2 支持），gpt-image-1.5 默认返回 base64 编码。`image` 支持 PNG/JPEG/WebP 格式，< 25MB。`mask` 仅支持 PNG 格式，< 4MB，透明区域表示需要编辑的区域。`mask` 仅 gpt-image-1.5 支持，Gemini 模型传入 `--mask` 会被 LiteLLM 拒绝。`output_format` 控制输出图片编码格式，由 CLI 的 `--response-format` 参数值映射而来。

**响应：**
```json
{
  "created": 1234567890,
  "data": [
    {
      "b64_json": "iVBORw0KGgo..."
    }
  ]
}
```

**CLI 逻辑：**
1. 读取 `--image` 和 `--mask`（如有）文件内容
2. 构建 multipart/form-data 请求体，POST 到 `{base_url}/v1/images/edits`
3. 解析响应，base64 解码 `b64_json` 后写入文件
4. 多张图片时，若指定 `--output result.png`，输出为 `result_1.png`, `result_2.png`, ...；未指定时自动命名为 `edited_YYYYMMDD_HHMMSS_1.png`, `edited_YYYYMMDD_HHMMSS_2.png`, ...

### 4.3 语音合成 — OpenAI 兼容

通过代理调用，代理负责转换为 ElevenLabs 原生格式。

**请求：**
```json
POST {base_url}/v1/audio/speech
Authorization: Bearer {api_key}

{
  "model": "elevenlabs/eleven_multilingual_v2",
  "input": "Hello, world!",
  "voice": "alloy",
  "response_format": "mp3",
  "speed": 1.0,
  "voice_settings": {
    "stability": 0.5,
    "similarity_boost": 0.75
  }
}
```

> `voice` 支持 OpenAI 映射名（如 `alloy`、`coral`）或 ElevenLabs 原生 Voice ID。LiteLLM 对不在映射表中的 voice 名称会原样透传给 ElevenLabs。

**响应：** 二进制音频流 (`application/octet-stream`)

**CLI 逻辑：**
1. 构建请求体，POST 到 `{base_url}/v1/audio/speech`
2. 一次性读取响应 body 到内存
3. 将音频数据写入输出文件

### 4.4 视频生成 — Volcengine ARK API

Seedance 2.0 API 使用异步任务模式，通过统一代理访问。model ID 由 `-m` 参数直接传入，不做映射。

#### 4.4.1 提交任务 (create)

```json
POST {base_url}/volcengine/api/v3/contents/generations/tasks
Authorization: Bearer {api_key}

{
  "model": "doubao-seedance-2-0-260128",
  "content": [
    { "type": "text", "text": "a dog running on the beach" },
    { "type": "image_url", "image_url": { "url": "data:image/png;base64,..." }, "role": "first_frame" }
  ],
  "duration": 5,
  "resolution": "720p",
  "ratio": "adaptive",
  "watermark": false,
  "generate_audio": true
}
```

`content` 数组支持以下类型：

| type | role 值 | 说明 |
|------|---------|------|
| `text` | — | 文本 prompt |
| `image_url` | `first_frame` / `last_frame` / `reference_image` | 图片（本地文件 base64 编码或 URL） |
| `video_url` | `reference_video` | 参考视频（仅 URL） |
| `audio_url` | `reference_audio` | 参考音频（本地文件 base64 编码或 URL） |

**响应：**
```json
{ "id": "task_xxx" }
```

#### 4.4.2 查询任务 (get)

```
GET {base_url}/volcengine/api/v3/contents/generations/tasks/{task_id}
Authorization: Bearer {api_key}
```

**响应：**
```json
{
  "id": "task_xxx",
  "model": "doubao-seedance-2-0-260128",
  "status": "succeeded",
  "content": { "video_url": "https://...", "last_frame_url": "https://..." },
  "duration": 5,
  "ratio": "adaptive",
  "resolution": "720p",
  "created_at": 1234567890,
  "updated_at": 1234567890
}
```

任务状态：`queued` → `running` → `succeeded` / `failed` / `cancelled`

#### 4.4.3 列出任务 (list)

```
GET {base_url}/volcengine/api/v3/contents/generations/tasks?page_num=1&page_size=10&filter.status=succeeded
Authorization: Bearer {api_key}
```

**响应：**
```json
{ "total": 42, "items": [ ... ] }
```

#### 4.4.4 删除任务 (delete)

```
DELETE {base_url}/volcengine/api/v3/contents/generations/tasks/{task_id}
Authorization: Bearer {api_key}
```

#### CLI 逻辑（generate）

1. 读取本地图片/音频文件 → base64 编码为 data URI
2. 组装 `content[]` + 生成参数 → 提交任务，获取 `task_id`
3. 按 `--poll-interval` 轮询任务状态，stderr 展示进度
4. 状态变为 `succeeded` 后下载 `video_url` 到输出路径
5. 超过 `--timeout` 则输出 task_id 供稍后查询
6. 注意：`video_url` 24 小时内有效

## 5. Workspace 与 Crate 结构

项目采用 Cargo workspace，拆分为 4 个 crate，Rust edition 统一使用 **2024**。

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
│   ├── tapsvc-aigc-core/          # 共享基础层 library crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # 公开 API
│   │       └── retry.rs          # 通用 retry 执行器（指数退避 + jitter）
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
    "crates/tapsvc-aigc-core",
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
| `tapsvc-aigc-core` | library | 共享基础层：通用 retry 执行器（指数退避 + jitter + Retry-After） |
| `tapsvc-aigc-openai` | library | OpenAI 兼容 API 的类型定义和 HTTP 调用（images + audio） |
| `tapsvc-aigc-ark` | library | Volcengine ARK API 的类型定义和 HTTP 调用（video） |

> **设计原则**：两个 library crate 不依赖 CLI 相关 crate（clap、indicatif 等），
> 保持纯粹的 API 客户端职责，可被其他项目复用。

## 6. 错误处理

| 场景 | 处理方式 |
|------|----------|
| 网络错误 / 代理不可达 | 自动重试，最多 3 次重试（总尝试 4 次），指数退避 2s→4s→8s + 随机 jitter，最终报错退出 |
| API 返回 4xx（非 429） | 不重试，解析错误信息，展示给用户 |
| API 返回 429 (限流) | 自动重试，优先使用 `Retry-After` header 的延迟值 |
| API 返回 500/502/503/504 | 自动重试，同网络错误退避策略 |
| 视频生成超时 | 输出 task_id，告知用户可稍后手动查询 |
| 输出文件已存在 | 默认覆盖，可加 `--no-overwrite` 选项 |
| 视频 URL 过期 | 提示 24 小时有效期限制 |

## 7. 输出行为

- 默认将进度信息输出到 stderr（不干扰 stdout 重定向）
- 生成成功后在 stdout 打印输出文件路径
- `--quiet` 选项抑制进度信息
- `--verbose` 选项输出详细请求/响应日志
