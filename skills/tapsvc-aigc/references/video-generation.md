# Video Generation Reference

## Models and Providers

Video uses provider-native async APIs, not `/v1/images/*` or an OpenAI video
endpoint. The CLI selects the provider from the model ID.

| Model | Provider | Best for | Resolution | Duration |
|---|---|---|---|---|
| `doubao-seedance-2-0-fast-260128` (DEFAULT) | Volcengine | Drafts and rapid iteration | 480p, 720p | 4-15 or `-1` auto |
| `doubao-seedance-2-0-260128` | Volcengine | Final, high-quality work | 480p, 720p, 1080p, 4k | 4-15 or `-1` auto |
| `happyhorse-1.1-t2v` | DashScope | Text-to-video | 720p, 1080p | 3-15 |
| `happyhorse-1.1-i2v` | DashScope | Animate one first frame | 720p, 1080p | 3-15 |
| `happyhorse-1.1-r2v` | DashScope | Compose from 1-9 reference images | 720p, 1080p | 3-15 |
| `happyhorse-1.0-video-edit` | DashScope | Edit one existing video, optionally with 1-5 reference images | 720p, 1080p | follows input video |

Choose Seedance for multimodal reference video/audio workflows and complex
camera choreography. Choose the exact HappyHorse variant when the task maps to
one of its focused modes. Use the full Seedance model for 1080p/4k final output.

## Parameter Constraints

Validate these constraints before building the command.
Stop and inform the user on violation — do NOT rely on CLI error messages.

| Parameter | Constraint | Notes |
|-----------|------------|-------|
| `--last-frame` | REQUIRES `--first-frame` | Cannot use end frame alone |
| `--ref-audio` | REQUIRES `--ref-image` OR `--ref-video` | Reference audio needs visual reference |
| `--first-frame` | EXCLUSIVE WITH `--ref-image`, `--ref-video` | First-frame mode and reference mode are mutually exclusive |
| `--duration` | 4-15 or -1 | -1 means auto, other values must be in 4-15 range |
| `--ref-image` | Max 9 | Repeatable, but no more than 9 |
| `--ref-video` | Max 3, URL only | Local file paths not supported |
| `--ref-audio` | Max 3 | Supports both local files and URLs |
| Input | At least one required | `--prompt`/`--prompt-file`, `--first-frame`, `--ref-image`, or `--ref-video` |

### HappyHorse Constraints

| Model | Required input | Optional input | Unsupported |
|---|---|---|---|
| `happyhorse-1.1-t2v` | prompt | none | all media |
| `happyhorse-1.1-i2v` | exactly one `--first-frame` | prompt | ratio, last frame, reference media |
| `happyhorse-1.1-r2v` | prompt and 1-9 `--ref-image` | none | first/last frame, video/audio refs |
| `happyhorse-1.0-video-edit` | prompt and exactly one public `--ref-video` URL | 0-5 `--ref-image` | ratio, first/last frame, audio refs, duration control |

All HappyHorse variants support 720p/1080p and seed 0-2147483647. They do not
support `--no-audio`, `--camera-fixed`, or `--web-search` in this CLI. For
reference-to-video, refer to images in the prompt as `[Image 1]`, `[Image 2]`,
and so on, in CLI argument order.

### Input Mode Quick Reference

```
Text-to-video:      -p <prompt>
Image-to-video:     --first-frame <path> [-p <prompt>] [--last-frame <path>]
Ref image + text:   --ref-image <path>... -p <prompt>
Ref video + text:   --ref-video <url>... -p <prompt>
With audio ref:     --ref-image/--ref-video + --ref-audio <path>...
```

## CLI Usage

```bash
# Video generation
tapsvc-aigc video generate -m <model> -p <prompt> [--prompt-file <path>] \
  [--first-frame <path>] [--last-frame <path>] \
  [--ref-image <path>...] [--ref-video <url>...] [--ref-audio <path>...] \
  [--resolution <480p|720p|1080p|4k>] \
  [--aspect-ratio <16:9|4:3|1:1|3:4|4:5|5:4|9:16|9:21|21:9|adaptive>] \
  [--duration <model-specific>] [--no-audio] [--camera-fixed] [--seed <n>] \
  [--poll-interval <sec>] [--timeout <sec>] [-o <output>]

# Query task status
tapsvc-aigc video get <task_id>

# Query a HappyHorse task
tapsvc-aigc video get <task_id> --provider happyhorse
```

> **Note**: Video generation is async. CLI auto-polls until completion or timeout.
> On timeout, it outputs the task_id — use `video get <task_id>` to check status.
> Video download URL is valid for 24 hours.

Do NOT use `video list` or `video delete`.

## Prompt Guidance Sources

Prefer first-party guidance over generic prompt collections:

- [Volcengine Seedance 2.0 prompt guide](https://www.volcengine.com/docs/82379/2222480)
- [ByteDance Seedance 2.0 product examples](https://seed.bytedance.com/zh/seedance2_0)
- HappyHorse API examples for [text-to-video](https://help.aliyun.com/zh/model-studio/happyhorse-text-to-video-api-reference),
  [image-to-video](https://help.aliyun.com/zh/model-studio/happyhorse-image-to-video-api-reference),
  [reference-to-video](https://help.aliyun.com/zh/model-studio/happyhorse-reference-to-video-api-reference),
  and [video editing](https://help.aliyun.com/zh/model-studio/happyhorse-video-edit-api-reference)

The Seedance guide is a full prompt guide. The HappyHorse pages are API
references with official examples, so the HappyHorse production templates
below combine those examples with conservative video-prompt practice. Do not
invent a universal negative-prompt list.

## Seedance 2.0 Prompt Practice

### Choose the task before writing the prompt

| Task | Recommended wording |
|---|---|
| Text / first-frame video | `Subject + action + scene + shot/camera + audio + style/quality + constraints` |
| Multimodal reference | `Reference image/video/audio N for one named property, then describe the new video` |
| Edit a video | `Strictly edit Video N: change X to Y; preserve A/B/C` |
| Extend a video | `Extend Video N forward/backward: next action or story beat` |

For an edit or extension, say `Video 1` rather than `reference Video 1`.
Seedance's official guide warns that the latter can be interpreted as a new
reference-generation task.

### Write instructions, not adjective piles

Treat the prompt as a small directing specification:

1. **Bind every subject.** With ordered reference inputs, consistently use
   `人物A@图片1`, `产品@图片2`, `视频1`, and `音频1`. The numbering follows CLI
   argument order. Do not alternate between names, pronouns, and vague phrases
   such as "the other person".
2. **Assign one responsibility to each asset.** For example: image 1 anchors
   the face, image 2 anchors clothing, video 1 supplies movement, and audio 1
   supplies voice or rhythm. Put the most important reference first. Do not use
   the maximum number of assets by default; conflicting references reduce
   control. The official guide recommends roughly 4-5 purposeful assets for a
   complex task.
3. **Storyboard in event order.** For complex video, use `镜头1`, `镜头2`,
   `镜头3`; within each shot write camera/cut, subject action and expression,
   spatial change, then audio. Prefer natural pacing. Exact intervals such as
   `0-3s` are not stable enough to use unless the user explicitly requires them.
4. **Use one camera movement per shot.** `中景缓慢推近`, `平稳横移`, or
   `固定机位` is clearer than requesting push, pull, pan, and orbit together.
   Cut to another shot when the camera behavior changes.
5. **Make action observable.** Name the body part plus amplitude, speed, and
   force: `右手缓慢抬至肩部`, `快速转头`, `用力蹬地`. Describe transitions:
   `借转身惯性顺势抬手`. Express emotion through behavior rather than only
   `very sad` or `very angry`.
6. **Keep difficult motion achievable.** Prefer continuous, physically
   connected motion. For dense fights, chases, or montage, generate shorter
   clips and edit them together instead of overloading one prompt.
7. **Close with only relevant boundaries.** State the desired style and quality,
   then constraints that matter, such as `保持无字幕`, `不要生成 Logo`, or
   `不要生成水印`. A long boilerplate negative list can conflict with the scene.

### Seedance high-quality examples

Text-to-video, written as natural shots rather than fragile timestamps:

```text
电影级写实，夜晚潮湿的城市街道，霓虹灯在路面形成冷蓝与洋红色倒影。
镜头1：低机位中景平稳跟拍，一名穿黑色夹克的年轻舞者缓慢走入半圆形人群中央，
他先低头调整呼吸，再抬起右手示意音乐开始；现场只有鞋底摩擦地面的声音和远处车流声。
镜头2：切至侧面全景，舞者快速完成一组连贯的 popping 动作，肩、肘、手腕依次发力，
围观者随节拍后退半步；鼓点逐渐增强。
镜头3：切至正面近景，舞者用力定格，胸口轻微起伏，随后露出克制的笑；人群爆发欢呼。
动作衔接自然，真实重心与惯性，电影质感，细节丰富，色彩自然，保持无字幕，不要生成 Logo 或水印。
```

First-frame animation: preserve appearance and spend the prompt budget on
motion, camera, and change over time.

```text
以首帧人物外观、服装和初始构图为准。固定中近景，她先轻轻眨眼，随后缓慢抬头看向窗外，
右手将耳边一缕头发自然别到耳后，窗帘被微风轻微吹动。晨光逐渐变暖，动作幅度克制、衔接自然，
人物面部与服装全程保持一致，无台词，只有轻微风声和室内环境声。
```

Multimodal reference: explicitly define identity and the job of each asset.

```text
将人物A@图片1定义为女剑客，将人物B@图片2定义为蒙面守卫。人物外观分别严格参考对应图片。
动作节奏参考视频1，但不复用视频1中的人物和场景；鼓点节奏参考音频1。
镜头1：废弃停车场全景缓慢推近，人物A与人物B相对站立，人物A右手缓慢握紧剑柄。
镜头2：切至侧面中景，两人按照视频1的动作顺序完成一次格挡与反击，动作重心、惯性和落地缓冲清楚。
镜头3：切至人物A近景，她收剑后保持警觉，远处警报灯闪烁。冷峻写实动作片风格，保持无字幕，
两名人物全程不互换外观，不出现重复人物。
```

```bash
tapsvc-aigc video generate \
  --ref-image swordswoman.png --ref-image guard.png \
  --ref-video https://example.com/action-reference.mp4 \
  --ref-audio beat-reference.mp3 --prompt-file prompt.txt \
  --duration 12 --resolution 1080p -o fight.mp4
```

Video editing uses a **Change + Preserve** instruction:

```text
严格编辑视频1：将桌上的透明香水瓶替换为图片1中的面霜罐，保持原视频的手部动作、运镜、
灯光、背景、时长和音频不变；面霜罐的大小、透视、遮挡和桌面接触阴影与原场景一致。
```

### References and character consistency

- For one person, prefer one clean face close-up plus one full-body styling
  image. Bind them explicitly: `人物A的面部参考图片1，服装和体型参考图片2`.
- Avoid a multi-view contact sheet for a person; Seedance may read the views as
  separate subjects. Clean single-person images are more reliable.
- More than four referenced people is unstable. Split large casts into groups
  or establish them in intermediate images first.
- If duplicate characters appear, restate every subject-to-image mapping and add
  a specific constraint that each named person appears only once. This reduces,
  but cannot guarantee elimination of, duplication.
- Use a reference video for exact motion, camera language, or special-effect
  behavior that is hard to describe reliably in text.

## HappyHorse Prompt Practice

HappyHorse has four purpose-built modes. Use the model's native mode instead of
trying to simulate it with another variant.

### Text-to-video (`happyhorse-1.1-t2v`)

Use concrete nouns and visible actions. A compact template is:

```text
Subject + specific action + environment/lighting + camera + visual style
```

Official example:

```text
一座由硬纸板和瓶盖搭建的微型城市，在夜晚焕发出生机。一列硬纸板火车缓缓驶过，
小灯点缀其间，照亮前路。
```

```bash
tapsvc-aigc video generate -m happyhorse-1.1-t2v \
  -p "一座由硬纸板和瓶盖搭建的微型城市，在夜晚焕发出生机。一列硬纸板火车缓缓驶过，小灯点缀其间，照亮前路。微距低机位平稳跟拍，定格动画质感。" \
  --resolution 1080p --aspect-ratio 16:9 --duration 6 -o city.mp4
```

### Image-to-video (`happyhorse-1.1-i2v`)

The image already defines subject, style, composition, and aspect ratio. Prompt
only the motion, camera, environmental change, and preservation requirements;
do not redescribe a conflicting appearance.

```text
首帧中的猫先压低前腿蓄力，随后沿草地向画面右侧自然奔跑，尾巴随步伐摆动，草叶被轻微带起。
低机位平稳跟拍，速度逐渐加快；保持猫的花纹、体型和场景风格与首帧一致。
```

```bash
tapsvc-aigc video generate -m happyhorse-1.1-i2v \
  --first-frame cat.png --prompt-file prompt.txt \
  --resolution 1080p --duration 5 -o cat.mp4
```

### Reference-to-video (`happyhorse-1.1-r2v`)

Refer to each input in exact CLI order as `[Image 1]`, `[Image 2]`, and so on,
and identify the object within the image. Do not write only "use Image 1".
The official example demonstrates both explicit bindings and a chronological
camera plan:

```text
[Image 1]中身着红色旗袍的女性，镜头先以侧面中景勾勒旗袍修身剪裁与 S 型曲线，
随即切换至低角度仰拍，捕捉她轻抬玉手展开[Image 2]中的折扇的同时，
[Image 3]中的流苏耳坠随头部转动轻盈摆动的细节，最后推近至面部特写，
定格在她指尖轻点扇骨、眼波流转间的含蓄风情，多视角全方位展现东方韵味。
```

```bash
tapsvc-aigc video generate -m happyhorse-1.1-r2v \
  --ref-image woman.png --ref-image fan.png --ref-image earrings.png \
  --prompt-file prompt.txt --aspect-ratio 9:16 --duration 8 -o reference.mp4
```

### Video editing (`happyhorse-1.0-video-edit`)

State the target and replacement, then preserve everything that should not
change. The official minimal example is `让视频中的马头人身角色穿上图片中的条纹毛衣`.
A production version is more explicit:

```text
将视频中马头人身角色原有的上衣替换为参考图片中的条纹毛衣；毛衣跟随角色身体动作自然形变，
保持角色头部、身体比例、动作、背景、镜头运动、时长和音频不变。
```

```bash
tapsvc-aigc video generate -m happyhorse-1.0-video-edit \
  --ref-video https://example.com/input.mp4 --ref-image sweater.png \
  --prompt-file prompt.txt --resolution 720p -o edited.mp4
```

For all HappyHorse modes, sequence multiple actions chronologically, keep
motion physically plausible, and iterate one major change at a time. A fixed
seed can improve repeatability but does not guarantee identical output.

## Camera Vocabulary

| 中文 | English | Use |
|---|---|---|
| 推镜 | Push in / Dolly in | Move toward the subject |
| 拉镜 | Pull out / Dolly out | Reveal more surroundings |
| 横摇 | Pan | Rotate horizontally |
| 俯仰 | Tilt | Rotate vertically |
| 横移 | Truck | Translate horizontally |
| 跟拍 | Tracking shot | Follow a moving subject |
| 环绕 | Orbit | Move around a subject |
| 升降 | Crane up/down | Move vertically |
| 手持 | Handheld | Controlled natural shake |
| 固定 | Static / Locked off | Use `--camera-fixed` for Seedance |

## Other Parameters

| Parameter | Default | Notes |
|-----------|---------|-------|
| `--resolution` | `720p` | Seedance full: up to 4k; fast: 480p/720p; HappyHorse: 720p/1080p |
| `--aspect-ratio` | `adaptive` | `16:9`, `4:3`, `1:1`, `3:4`, `9:16`, `21:9`, `adaptive` |
| `--duration` | `5` | Seedance: 4-15 or `-1`; HappyHorse generation: 3-15; video edit follows input |
| `--no-audio` | false | Disable audio generation |
| `--camera-fixed` | false | Lock camera position |
| `--watermark` | false | Add watermark |
| `--web-search` | false | Enable web search enhancement |
| `--seed` | random | Fix seed for reproducibility |
| `--poll-interval` | `10` | Polling interval in seconds |
| `--timeout` | `300` | Timeout in seconds, outputs task_id on expiry |
