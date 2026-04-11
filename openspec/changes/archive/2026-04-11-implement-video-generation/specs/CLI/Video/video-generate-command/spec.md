## ADDED Requirements

### Requirement: VideoCommand 枚举定义
`cli.rs` 中 `VideoCommand` SHALL 包含 `Generate`、`Get`、`List`、`Delete` 四个变体。

#### Scenario: 枚举变体完整
- **WHEN** 定义 `VideoCommand`
- **THEN** SHALL 派生 `clap::Subcommand`，包含 `Generate`、`Get`、`List`、`Delete` 四个变体

### Requirement: Generate 子命令 prompt 输入参数
`VideoCommand::Generate` SHALL 支持文本 prompt 输入，可通过直接传入或从文件读取，两者可同时使用。

#### Scenario: 通过 --prompt 传入文本
- **WHEN** 用户指定 `--prompt "a dog running"`
- **THEN** SHALL 将该文本作为 content 中的 text item 发送

#### Scenario: 通过 --prompt-file 从文件读取
- **WHEN** 用户指定 `--prompt-file prompt.txt`
- **THEN** SHALL 读取文件内容作为 prompt 文本

#### Scenario: prompt 和 prompt-file 同时使用
- **WHEN** 用户同时指定 `--prompt "extra instructions" --prompt-file prompt.txt`
- **THEN** SHALL 将 prompt-file 内容在前、prompt 文本在后拼接（以换行分隔），作为完整 prompt 发送

#### Scenario: prompt 可选
- **WHEN** 用户未指定 `--prompt` 也未指定 `--prompt-file`，但提供了 `--first-frame`、`--ref-image` 或 `--ref-video` 中的至少一种
- **THEN** SHALL 正常执行（纯媒体输入模式，无文本 prompt）

### Requirement: Generate 子命令图片帧参数
`VideoCommand::Generate` SHALL 支持 `--first-frame` 和 `--last-frame` 参数用于帧控制模式。

#### Scenario: 首帧图生视频
- **WHEN** 用户指定 `--first-frame start.jpg`
- **THEN** SHALL 将该图片 base64 编码后作为 `role: "first_frame"` 的 image_url content item

#### Scenario: 首尾帧控制
- **WHEN** 用户同时指定 `--first-frame start.jpg --last-frame end.jpg`
- **THEN** SHALL 分别编码为 `role: "first_frame"` 和 `role: "last_frame"` 的 image_url content item

#### Scenario: last-frame 必须搭配 first-frame
- **WHEN** 用户指定了 `--last-frame` 但未指定 `--first-frame`
- **THEN** SHALL 报错并提示 `--last-frame` 需要搭配 `--first-frame`

### Requirement: Generate 子命令参考媒体参数
`VideoCommand::Generate` SHALL 支持 `--ref-image`、`--ref-video`、`--ref-audio` 参数用于多模态参考模式。

#### Scenario: 多张参考图片
- **WHEN** 用户指定 `--ref-image a.jpg --ref-image b.jpg`（最多 9 张）
- **THEN** SHALL 将每张图片 base64 编码后作为 `role: "reference_image"` 的 image_url content item

#### Scenario: 参考视频（仅 URL）
- **WHEN** 用户指定 `--ref-video https://example.com/v.mp4`（最多 3 个）
- **THEN** SHALL 将 URL 作为 `role: "reference_video"` 的 video_url content item
- **AND** `--ref-video` 仅支持 URL（`http://`/`https://` 开头），不支持本地文件路径

#### Scenario: ref-video 传入本地路径
- **WHEN** 用户指定 `--ref-video local_video.mp4`（非 URL）
- **THEN** SHALL 报错并提示 `--ref-video` 仅支持 URL，不支持本地文件

#### Scenario: 参考音频
- **WHEN** 用户指定 `--ref-audio audio.wav`（最多 3 个）
- **THEN** SHALL 将音频文件 base64 编码（或直接使用 URL）作为 `role: "reference_audio"` 的 audio_url content item

#### Scenario: ref-audio 必须搭配 ref-image 或 ref-video
- **WHEN** 用户指定了 `--ref-audio` 但未指定 `--ref-image` 也未指定 `--ref-video`
- **THEN** SHALL 报错并提示 `--ref-audio` 需要搭配 `--ref-image` 或 `--ref-video`

#### Scenario: ref-image 数量上限
- **WHEN** 用户指定的 `--ref-image` 超过 9 张
- **THEN** SHALL 报错并提示最多支持 9 张参考图片

#### Scenario: ref-video 数量上限
- **WHEN** 用户指定的 `--ref-video` 超过 3 个
- **THEN** SHALL 报错并提示最多支持 3 个参考视频

#### Scenario: ref-audio 数量上限
- **WHEN** 用户指定的 `--ref-audio` 超过 3 个
- **THEN** SHALL 报错并提示最多支持 3 个参考音频

#### Scenario: first/last frame 与 ref-image/ref-video 互斥
- **WHEN** 用户同时指定了 `--first-frame` 和 `--ref-image`（或 `--ref-video`）
- **THEN** SHALL 报错并提示两种模式互斥

### Requirement: Generate 子命令无输入校验
至少需要提供一种输入（prompt、prompt-file、first-frame、ref-image、ref-video 之一）。

#### Scenario: 无任何输入
- **WHEN** 用户未指定 `--prompt`、`--prompt-file`、`--first-frame`、`--ref-image`、`--ref-video` 中的任何一个
- **THEN** SHALL 报错并提示至少需要提供一种输入

### Requirement: Generate 子命令生成参数
`VideoCommand::Generate` SHALL 支持模型、分辨率、宽高比、时长等生成控制参数。

#### Scenario: 模型参数
- **WHEN** 用户指定 `-m doubao-seedance-2-0-260128`
- **THEN** SHALL 直接将该值透传给 API 的 `model` 字段

#### Scenario: 分辨率参数
- **WHEN** 用户指定 `--resolution 720p`
- **THEN** SHALL 设置 API 的 `resolution` 字段
- **AND** 仅接受 `480p`、`720p` 两个值，默认为 `720p`

#### Scenario: 宽高比参数
- **WHEN** 用户指定 `--aspect-ratio 16:9`
- **THEN** SHALL 设置 API 的 `ratio` 字段
- **AND** 接受 `16:9`、`4:3`、`1:1`、`3:4`、`9:16`、`21:9`、`adaptive`，默认为 `adaptive`

#### Scenario: 时长参数
- **WHEN** 用户指定 `--duration 10`
- **THEN** SHALL 设置 API 的 `duration` 字段
- **AND** 接受 4-15 秒或 -1（自动），默认为 `5`
- **AND** CLI 参数类型 SHALL 为 `i32`（以支持 -1 值）

#### Scenario: 音频生成默认开启
- **WHEN** 用户未指定 `--no-audio`
- **THEN** SHALL 设置 API 的 `generate_audio` 为 `true`

#### Scenario: 禁用音频
- **WHEN** 用户指定 `--no-audio`
- **THEN** SHALL 设置 API 的 `generate_audio` 为 `false`

#### Scenario: 水印参数
- **WHEN** 用户指定 `--watermark`
- **THEN** SHALL 设置 API 的 `watermark` 为 `true`，默认为 `false`

#### Scenario: 网络搜索增强
- **WHEN** 用户指定 `--web-search`
- **THEN** SHALL 在 API 请求中添加 `tools: [{"type": "web_search"}]`

#### Scenario: 固定镜头
- **WHEN** 用户指定 `--camera-fixed`
- **THEN** SHALL 设置 API 的 `camera_fixed` 为 `true`

#### Scenario: 随机种子
- **WHEN** 用户指定 `--seed 42`
- **THEN** SHALL 设置 API 的 `seed` 为 `42`

### Requirement: Generate 子命令轮询控制参数

#### Scenario: 轮询间隔
- **WHEN** 用户指定 `--poll-interval 15`
- **THEN** SHALL 每隔 15 秒查询一次任务状态，默认为 `10` 秒

#### Scenario: 超时
- **WHEN** 用户指定 `--timeout 600`
- **THEN** SHALL 在 600 秒后若任务未完成则报超时错误，默认为 `300` 秒

### Requirement: Generate 执行流程
`cmd/video.rs` 中 generate 的 handle 函数 SHALL 执行完整的任务提交-轮询-下载流程。

#### Scenario: 成功生成并下载
- **WHEN** 提交任务后轮询状态变为 `succeeded`
- **THEN** SHALL 从响应中提取 `content.video_url`，下载视频到输出路径
- **AND** SHALL 在 stderr 展示轮询进度（状态、已耗时）
- **AND** SHALL 在 stdout 打印输出文件路径

#### Scenario: 生成失败
- **WHEN** 轮询状态变为 `failed`
- **THEN** SHALL 输出 API 返回的错误信息并以非零状态码退出

#### Scenario: 超时处理
- **WHEN** 轮询时间超过 `--timeout` 且任务仍未完成
- **THEN** SHALL 输出 task_id 并提示用户可通过 `video get <task-id>` 稍后查询

### Requirement: Generate 输出文件命名
#### Scenario: 指定输出路径
- **WHEN** 用户指定 `-o output.mp4`
- **THEN** SHALL 将视频保存到 `output.mp4`

#### Scenario: 自动命名
- **WHEN** 用户未指定 `-o`
- **THEN** SHALL 使用 `video_{timestamp}.mp4` 格式自动命名

### Requirement: 本地文件 base64 编码
`cmd/video.rs` SHALL 将本地图片和音频文件编码为 data URI 格式。`--ref-video` 不适用此规则（仅支持 URL）。

#### Scenario: 本地图片文件
- **WHEN** `--first-frame`、`--last-frame`、`--ref-image` 的参数值为本地文件路径（不以 `http://`、`https://`、`data:` 开头）
- **THEN** SHALL 读取文件、根据扩展名确定 MIME type、base64 编码为 `data:image/{fmt};base64,{data}` 格式

#### Scenario: 本地音频文件
- **WHEN** `--ref-audio` 的参数值为本地文件路径（不以 `http://`、`https://`、`data:` 开头）
- **THEN** SHALL 读取文件、根据扩展名确定 MIME type、base64 编码为 `data:audio/{fmt};base64,{data}` 格式

#### Scenario: 图片/音频参数 URL 直接透传
- **WHEN** `--first-frame`、`--last-frame`、`--ref-image`、`--ref-audio` 的参数值以 `http://`、`https://` 或 `data:` 开头
- **THEN** SHALL 直接使用该 URL，不做编码处理

#### Scenario: ref-video 仅接受 HTTP URL
- **WHEN** `--ref-video` 的参数值
- **THEN** SHALL 仅接受以 `http://` 或 `https://` 开头的 URL，不接受 `data:` URI 也不接受本地文件路径
