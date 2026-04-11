# Image Generation & Editing Reference

## Models

| Feature | gpt-image-1.5 (DEFAULT) | gemini-3-pro-image-preview | gemini-3.1-flash-image-preview |
|---------|------------------------|---------------------------|-------------------------------|
| Count | n=1 only | n=1 only | n=1 only |
| size | `1024x1024`, `1536x1024`, `1024x1536`, `auto` | mapped to aspectRatio | mapped to aspectRatio |
| quality | `auto`, `high`, `medium`, `low` | not supported | not supported |
| background | `transparent`, `opaque`, `auto` | not supported | not supported |
| mask edit | supported (PNG, < 4MB) | not supported | not supported |
| text rendering | strong | moderate | moderate |

> **Important**: All models support `n=1` only (litellm proxy limitation).
> For multiple images, execute the command multiple times with different prompts or params.

> **Important**: Mask editing is `gpt-image-1.5` only. When mask editing is
> requested, MUST switch to `gpt-image-1.5` regardless of user's model choice.

## CLI Usage

```bash
# Image generation
tapsvc-aigc image generate -m <model> -p <prompt> [--prompt-file <path>] \
  [--size <WxH>] [--quality <auto|high|medium|low>] \
  [--background <transparent|opaque|auto>] [--response-format <png|jpeg|webp>] \
  [-o <output>]

# Image editing
tapsvc-aigc image edit -m <model> --image <path> -p <prompt> [--prompt-file <path>] \
  [--mask <path>] [--size <WxH>] [--response-format <png|jpeg|webp>] \
  [-o <output>]
```

## Prompt Best Practices — Generation

### Structured Prompt Framework

Organize prompts in this order:

1. **Background / Scene** — environment, atmosphere, time of day
2. **Subject** — detailed description of the main object
3. **Key details** — materials, textures, colors
4. **Constraints** — style, exclusions

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
