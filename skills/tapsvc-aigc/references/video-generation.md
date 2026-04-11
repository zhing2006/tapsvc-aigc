# Video Generation Reference

## Models

| Feature | doubao-seedance-2-0-fast-260128 (DEFAULT) | doubao-seedance-2-0-260128 |
|---------|------------------------------------------|---------------------------|
| Speed | Fast | Standard |
| Quality | Good | Excellent |
| Best for | Rapid iteration, drafts, testing | Final output, high-quality needs |
| Resolution | 480p, 720p | 480p, 720p |
| Duration | 4-15 seconds | 4-15 seconds |
| Input modes | Text, image, video, audio | Text, image, video, audio |

Use `doubao-seedance-2-0-260128` when user asks for "high quality" or "final version".
Default to the fast variant otherwise.

## Parameter Constraints

Agent MUST validate these constraints before building the command.
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
  [--resolution <480p|720p>] [--aspect-ratio <16:9|4:3|1:1|3:4|9:16|21:9|adaptive>] \
  [--duration <4-15|-1>] [--no-audio] [--camera-fixed] [--seed <n>] \
  [--poll-interval <sec>] [--timeout <sec>] [-o <output>]

# Query task status
tapsvc-aigc video get <task_id>
```

> **Note**: Video generation is async. CLI auto-polls until completion or timeout.
> On timeout, it outputs the task_id — use `video get <task_id>` to check status.
> Video download URL is valid for 24 hours.

Do NOT use `video list` or `video delete`.

## Prompt Best Practices

### Structure

Seedance prompts should follow:

```
Subject + Action + Scene + Camera + Style
```

**1. Subject** — Describe appearance, age, clothing, hairstyle, distinguishing features.

**2. Action** — Use specific verbs: "slowly turns to face the camera" over "turns around".
List multiple actions in chronological order.

**3. Scene** — Environment, weather, lighting, atmosphere.
Emphasize sensory detail: "sunlight filtering through leaves", "wet cobblestone after rain".

**4. Camera** — Use standard camera language (see vocabulary table below).
Camera movement is Seedance's strength — use it fully.

**5. Style** — Use genre framing: "European art-house film", "Japanese healing anime".

### Language Choice

Seedance 2.0 has first-class Chinese prompt support:

- 中文题材、中文叙事、国风/广告/短剧场景，优先使用中文提示词
- Camera, style, and photography terms can mix Chinese and English
- For complex long takes, prefer the time-segment format (see below)

Chinese storyboard-style example:
```
[0s-3s] 特写：咖啡缓缓倒入白色陶瓷杯中，热气升腾
[3s-6s] 镜头缓慢拉远，reveal 温馨的咖啡馆内部，晨光穿过落地窗
[6s-10s] 一位穿米色毛衣的女生端起杯子轻抿一口，嘴角微微上扬
```

### Example

```
Bad:  a person running on a beach
Good: A young woman with long black hair, wearing a white sundress,
runs barefoot along a tropical beach at golden hour. Waves gently
lap at her feet as she looks back over her shoulder with a joyful
smile. The camera follows her from a low angle, tracking smoothly
along the shoreline. Cinematic, warm color grading, anamorphic lens.
```

## Camera Language Vocabulary

| 中文 | English | Description |
|------|---------|-------------|
| 推镜 | Push in / Dolly in | Camera moves toward subject, builds tension |
| 拉镜 | Pull out / Dolly out | Camera moves away, reveals surroundings |
| 摇镜 | Pan (left/right) | Camera rotates horizontally |
| 俯仰 | Tilt (up/down) | Camera rotates vertically |
| 移镜 | Truck (left/right) | Camera translates horizontally |
| 跟镜 | Follow / Tracking shot | Camera follows a moving subject |
| 环绕 | Circle around / Orbit | 360° rotation around subject |
| 升镜 | Crane up / Rise | Camera rises upward |
| 降镜 | Crane down / Descend | Camera descends |
| 变焦 | Zoom in/out | Lens focal length change |
| 航拍 | Aerial shot / Drone shot | High overhead perspective |
| 手持 | Handheld | Slight shake, adds immersion |
| 固定 | Static / Locked off | Use `--camera-fixed` flag |

### Combining Camera Movements

Combine movements for complex long-take effects:

```
Camera descends from a bird's-eye view of the city skyline,
slowly crane down between buildings, then push in to a person
walking on the street below.
```

## Style Keywords

| Category | Keywords |
|----------|----------|
| Photorealistic | `photorealistic`, `cinematic`, `documentary style`, `raw footage` |
| Cinematic | `anamorphic lens`, `film grain`, `color grading`, `letterbox` |
| 2D animation | `2D animation`, `anime style`, `hand-drawn`, `cel animation` |
| 3D animation | `3D animation`, `Pixar style`, `claymation`, `stop motion` |
| Special | `voxel`, `pixel art`, `felt texture`, `watercolor animation` |
| Atmosphere | `European art-house`, `retro Hong Kong film`, `Japanese healing`, `noir` |

## Character Consistency

Maintaining character consistency is a key challenge in video generation:

1. **Detail facial features** — explicitly describe face, hair style, hair color, body type
2. **Specify clothing** — color, style, material, accessories
3. **State consistency requirement** — add "consistent faces, clothing, and hairstyles
   throughout without deformation, drift, or artifacts"
4. **Use first frame** — `--first-frame` locks character appearance
5. **Use reference images** — `--ref-image` provides character reference

Example:
```
A 30-year-old woman with shoulder-length auburn hair, green eyes,
wearing a navy blue blazer over a white blouse. She maintains the
same appearance throughout — consistent face, clothing, and hairstyle
without deformation or drift.
```

## Multi-Action & Multi-Shot Techniques

### Chronological Actions

List consecutive actions in order:

```
The dancer raises her arms slowly, spins once on her toes,
then leaps into the air with arms extended.
```

### Multi-Character Actions

Describe each character's action separately:

```
The lead singer holds the microphone and sings passionately on stage,
while the guitarist plays energetically beside her, and the drummer
keeps a steady beat in the background.
```

### Scene Switching

Use "camera switch" or "scene switch" to transition:

```
Scene 1: Close-up of a woman's face as she opens her eyes.
Camera switch to a wide shot of the sunrise over the mountains.
Scene 2: The woman walks through a meadow, camera following from behind.
```

### Time-Segment Format

For precise timing control:

```
0-5s: Close-up of coffee being poured into a ceramic mug, steam rising.
5-10s: Camera pulls back to reveal a cozy café interior, morning light
       streaming through windows.
10-15s: A woman picks up the mug, takes a sip, and smiles contentedly.
```

## Other Parameters

| Parameter | Default | Notes |
|-----------|---------|-------|
| `--resolution` | `720p` | `480p` (faster) or `720p` |
| `--aspect-ratio` | `adaptive` | `16:9`, `4:3`, `1:1`, `3:4`, `9:16`, `21:9`, `adaptive` |
| `--duration` | `5` | 4-15 seconds, or `-1` for auto |
| `--no-audio` | false | Disable audio generation |
| `--camera-fixed` | false | Lock camera position |
| `--watermark` | false | Add watermark |
| `--web-search` | false | Enable web search enhancement |
| `--seed` | random | Fix seed for reproducibility |
| `--poll-interval` | `10` | Polling interval in seconds |
| `--timeout` | `300` | Timeout in seconds, outputs task_id on expiry |
