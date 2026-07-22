# Image Generation & Editing Reference

## Models

| Feature | gpt-image-2 (DEFAULT) | gpt-image-1.5 | gemini-3-pro-image | gemini-3.1-flash-image |
|---------|----------------------|---------------|--------------------|------------------------|
| Count | n=1 only | n=1 only | n=1 only | n=1 only |
| size | `auto` only (router decides; bias via prompt) | `1024x1024`, `1536x1024`, `1024x1536`, `auto` | mapped to aspectRatio | mapped to aspectRatio |
| quality | `auto`, `high`, `medium`, `low` | `auto`, `high`, `medium`, `low` | not supported | not supported |
| background | `auto` / `opaque` only — `transparent` rejected, describe in prompt | `transparent`, `opaque`, `auto` | not supported | not supported |
| mask edit | not supported | supported (PNG, < 4MB) | not supported | not supported |
| text rendering | strongest, multilingual | strong | moderate | moderate |

Stable Gemini model IDs are `gemini-3-pro-image` and
`gemini-3.1-flash-image`. The gateway may retain the corresponding `-preview`
IDs for compatibility; prefer stable IDs for new work. Use Flash for general,
fast generation and Pro for complex, polished assets that need stronger
instruction following.

## Protocol Boundary

This CLI calls the OpenAI-compatible `/v1/images/generations` and
`/v1/images/edits` endpoints. `GET /v1/models` is a flat catalog, so model
presence alone does not prove image-endpoint compatibility.

Gemini's native API supports additional features such as Google Search
grounding, thinking controls, multi-turn image refinement, and richer
multi-reference inputs. They are not exposed by this CLI's OpenAI-shaped image
commands. Use the Gemini-native `/v1beta/models/{model}:generateContent` route
when a task explicitly requires them.

> **Important**: All models support `n=1` only (litellm proxy limitation).
> For multiple images, execute the command multiple times with different prompts or params.

> **Important**: Mask editing is `gpt-image-1.5` only. When mask editing is
> requested, MUST switch to `gpt-image-1.5` regardless of user's model choice.

> **gpt-image-2 size**: only `--size auto` (the new default) is accepted. The
> router chooses output dimensions; bias the aspect ratio in the prompt instead
> (`square composition` / `portrait composition` / `wide landscape 16:9`).
> Passing an explicit `WxH` will be rejected by the proxy.

> **gpt-image-2 background**: `--background transparent` is rejected. To get a
> transparent subject, describe it in the prompt (e.g.
> `transparent background, isolated subject on alpha channel, no backdrop`),
> or fall back to `gpt-image-1.5 --background transparent` for a parameter-level
> guarantee.

## CLI Usage

```bash
# Image generation
tapsvc-aigc image generate -m <model> -p <prompt> [--prompt-file <path>] \
  [--size <auto|WxH>] [--quality <auto|high|medium|low>] \
  [--background <transparent|opaque|auto>] [--response-format <png|jpeg|webp>] \
  [-o <output>]

# Image editing
tapsvc-aigc image edit -m <model> --image <path> -p <prompt> [--prompt-file <path>] \
  [--mask <path>] [--size <auto|WxH>] [--response-format <png|jpeg|webp>] \
  [-o <output>]
```

`--size` defaults to `auto`; when `auto`, the field is omitted from the request
(required for gpt-image-2, accepted by all other models).

### Quick examples

```bash
# gpt-image-2 (default) — bias aspect ratio via prompt, no --size
tapsvc-aigc image generate -m gpt-image-2 \
  -p "A ginger tabby cat in a sunlit window, square composition" \
  -o cat.png

# gpt-image-1.5 — explicit WxH still works
tapsvc-aigc image generate -m gpt-image-1.5 \
  -p "wide cinematic landscape of misty mountains at sunrise" \
  --size 1536x1024 -o mountains.png

# Transparent sticker — must use gpt-image-1.5
tapsvc-aigc image generate -m gpt-image-1.5 \
  -p "cute cartoon robot mascot, flat design, sticker" \
  --background transparent -o robot.png
```

## Prompt Best Practices — Generation

### Structured Prompt Framework

Organize prompts in this order:

1. **Background / Scene** — environment, atmosphere, time of day
2. **Subject** — detailed description of the main object
3. **Key details** — materials, textures, colors
4. **Constraints** — style, exclusions

For Gemini, make the brief concrete and purpose-led: say what the image is for,
describe the composition in layers or steps, use positive constraints, add
camera and lighting language, then refine one change at a time. Prefer
"an empty street" to "no cars" and "place the headline in the upper third" to
vague aesthetic instructions.

Example:

```
Bad:  a cat
Good: A ginger tabby cat lounging on a sunlit windowsill, soft morning light
casting warm golden tones, shallow depth of field, shot with a 50mm lens,
photorealistic style, no text or watermarks.
```

### Style Keywords

| Category | Keywords |
|----------|----------|
| Photorealistic | `photorealistic`, `candid photograph`, `raw photo`, `film grain` |
| Illustration | `digital illustration`, `concept art`, `watercolor`, `ink wash` |
| 3D | `3D render`, `isometric`, `clay render`, `octane render` |
| Flat design | `flat design`, `vector art`, `minimalist`, `clean lines` |
| Painting | `oil painting`, `impressionist`, `baroque`, `art nouveau` |

### Photography Language

- **Lens**: `50mm lens`, `wide-angle`, `macro`, `telephoto`, `fisheye`
- **Aperture**: `shallow depth of field`, `f/1.4`, `bokeh`
- **Lighting**: `golden hour`, `soft diffuse lighting`, `high-key`, `low-key`,
  `rim lighting`, `backlighting`, `studio lighting`
- **Angle**: `eye-level`, `low-angle`, `bird's-eye view`, `Dutch angle`,
  `over-the-shoulder`
- **Composition**: `rule of thirds`, `centered composition`, `negative space`,
  `leading lines`, `symmetrical`

### Text Rendering

gpt-image-1.5 has strong text rendering. For best results:

- Wrap text in **quotes** or **ALL CAPS**: `the sign reads "OPEN 24/7"`
- Specify typography: `bold sans-serif font, white text on dark background, centered`
- Demand verbatim rendering: `render the text verbatim, no extra characters`
- Spell out complex words letter-by-letter: `spelled O-P-E-N`
- Use `quality="high"` for text-heavy layouts

### Negative Prompts

Explicitly exclude unwanted elements:

```
no watermark, no text overlay, no extra limbs, no distorted faces,
no yellow tint, no plastic texture, no oversaturated colors
```

Note: Gemini models respond better to **positive descriptions** instead.
Say "an empty, deserted street" rather than "no cars".

## Prompt Best Practices — Editing

### Change Isolation

Clearly separate what changes from what is protected:

```
Change only the background from indoor to outdoor beach scene.
Keep the person, their pose, clothing, facial expression, and all other
elements exactly the same.
```

### Preserving Invariants

Re-state invariants on every iteration to prevent drift:

- `Do not change the subject's face, skin tone, body shape, or identity`
- `Preserve the original lighting direction and color temperature`
- `Keep the camera angle and perspective unchanged`

### Mask Editing (gpt-image-1.5 only)

- Mask must be PNG, < 4MB
- Transparent areas mark regions to edit
- Pass via `--mask <path>`

```bash
tapsvc-aigc image edit -m gpt-image-1.5 \
  --image photo.png --mask mask.png \
  -p "Replace the masked area with a blooming cherry tree"
```

### Iterative Strategy

1. Start with a single change, don't cram everything in one edit
2. One modification per iteration
3. Use "same style as before" to maintain consistency
4. If details drift, re-state key preservation instructions
