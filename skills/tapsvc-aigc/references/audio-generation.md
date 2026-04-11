# Audio Generation Reference

## Models

| Feature | elevenlabs/eleven_v3 (DEFAULT) | elevenlabs/eleven_multilingual_v2 |
|---------|-------------------------------|----------------------------------|
| Max input | 3000 chars | 10000 chars |
| Expressiveness | High (Audio Tags supported) | Moderate |
| Multilingual | Yes | Yes (broader) |
| Audio Tags | Full support | Not supported |
| Best for | Short text, emotional delivery | Long text, plain narration |

## Text Length Auto-Switch Rules

Agent MUST check input text length before building the command:

| Text length | Action |
|-------------|--------|
| ≤ 3000 chars | Use `elevenlabs/eleven_v3` (DEFAULT) |
| 3001–10000 chars | Auto-switch to `elevenlabs/eleven_multilingual_v2`, inform user |
| > 10000 chars | Stop, ask user to split text |

If user explicitly chose a model but text exceeds its limit,
MUST NOT execute — ask user to switch model or split text.

## CLI Usage

```bash
tapsvc-aigc audio speech -m <model> --voice <voice> -i <text> [--input-file <path>] \
  [--format <mp3|opus|aac|flac|wav|pcm>] [--speed <0.5-2.0>] \
  [--stability <0.0-1.0>] [--similarity <0.0-1.0>] \
  [-o <output>]
```

## Default Voices

10 preset voices, use via `--voice <name>`:

| CLI Voice | ElevenLabs Voice | Voice ID | Character |
|-----------|-----------------|----------|-----------|
| `alloy` | Rachel | `21m00Tcm4TlvDq8ikWAM` | Female, clear and warm |
| `amber` | Paul | `5Q0t7uMcjvnagumLfvZi` | Male, steady newscaster |
| `ash` | Domi | `AZnzlk1XvdvUeBnXmlld` | Female, composed and confident |
| `august` | Fin | `D38z5RcWu1voky8WS1ja` | Neutral, Irish accent |
| `blue` | Clyde | `2EiwWnXFnvU5JabPnv8n` | Male, deep and textured |
| `coral` | Aria | `9BWtsMINqrJLrRacOk9x` | Female, bright and lively |
| `lily` | Sarah | `EXAVITQu4vr4xnSDxMaL` | Female, soft and educational |
| `onyx` | Drew | `29vD33N1CtxCmqQRPOHJ` | Male, deep and magnetic |
| `sage` | Roger | `CwhRBWXzGAHq8TQ4Fs17` | Male, mature British accent |
| `verse` | Dave | `CYw3kZ02Hs0563khs1Fj` | Male, young and energetic |

### Voice Recommendation Guide

| User description | Recommended voice |
|------------------|-------------------|
| Warm female / narration | `alloy` (Rachel) or `lily` (Sarah) |
| Lively female / ads | `coral` (Aria) |
| Steady male / news | `amber` (Paul) or `onyx` (Drew) |
| Deep and textured | `blue` (Clyde) |
| British accent | `sage` (Roger) |
| Young and energetic | `verse` (Dave) |

### Custom Voice ID

Users can pass an ElevenLabs Voice ID directly for voices not in the
default list:

```bash
tapsvc-aigc audio speech -m elevenlabs/eleven_v3 \
  --voice pNInz6obpgDQGcFmaJgB -i "Hello world"
```

## Text Formatting Best Practices

Audio prompt optimization is **text formatting**, not content rewriting.

### Number & Symbol Normalization

Convert numbers, abbreviations, and symbols into speakable text:

| Original | Normalized (English) | Normalized (Chinese) |
|----------|---------------------|---------------------|
| `$42.50` | `forty-two dollars and fifty cents` | 四十二美元五十美分 |
| `3.14` | `three point one four` | 三点一四 |
| `555-1234` | `five five five, one two three four` | 五五五，一二三四 |
| `2024-01-15` | `January fifteenth, twenty twenty-four` | 二零二四年一月十五日 |
| `Dr.` | `Doctor` | 博士 / 医生 |
| `vs.` | `versus` | 对 / 与 |
| `10am` | `ten A M` | 上午十点 |
| `50%` | `fifty percent` | 百分之五十 |
| `Ctrl+Z` | `control Z` | control Z |

### Punctuation for Pacing

- **Comma `,`** — short pause
- **Period `.`** — medium pause
- **Ellipsis `...`** — longer pause, adds hesitation
- **Em dash `—`** — tone shift
- **Exclamation `!`** — emphasis
- **Question `?`** — rising intonation

### Minimum Text Length

eleven_v3 output may be unstable with text < 250 characters.
If the original text is short, preserve it as-is — optimize only through
punctuation, Audio Tags, and pacing. Do NOT add extra content.
Warn the user that short text may produce inconsistent results.

## Audio Tags Guide

Audio Tags are an eleven_v3 feature — bracketed cues that control delivery.

> **Note**: Audio Tags are `elevenlabs/eleven_v3` only.
> Do NOT insert Audio Tags when using `elevenlabs/eleven_multilingual_v2`.

### Emotion Tags

| Tag | Effect | Use case |
|-----|--------|----------|
| `[excited]` | Excited, enthusiastic | Good news, surprises |
| `[nervous]` | Anxious, uneasy | Worry, tension |
| `[frustrated]` | Annoyed, irritated | Complaints |
| `[sorrowful]` | Sad, somber | Bad news, farewells |
| `[calm]` | Peaceful, serene | Meditation, narration |
| `[curious]` | Inquisitive | Questions, exploration |
| `[mischievously]` | Playful, sly | Jokes, hints |

### Reaction Tags

| Tag | Effect |
|-----|--------|
| `[laughs]` | Laughter |
| `[sighs]` | Sigh |
| `[gasps]` | Sharp inhale |
| `[whispers]` | Whisper |
| `[crying]` | Crying |
| `[gulps]` | Swallow |

### Tone Tags

| Tag | Effect |
|-----|--------|
| `[cheerfully]` | Cheerful tone |
| `[flatly]` | Flat, emotionless |
| `[deadpan]` | Deadpan delivery |
| `[playfully]` | Playful, bouncy |
| `[sarcastic]` | Sarcastic |

### Pacing Tags

| Tag | Effect |
|-----|--------|
| `[pauses]` | Pause |
| `[hesitates]` | Hesitation |
| `[stammers]` | Stutter |
| `[rushed]` | Speed up |
| `[drawn out]` | Elongate |

### Usage Example

```
Original:
I don't know what to say. This is shocking. Are you sure this is real?

Optimized:
[gasps] I don't know what to say... [pauses] This is shocking.
[curious] Are you sure... this is real?
```

### Audio Tags Principles

1. **Match voice character** — don't use `[giggles]` on a deep male voice.
   Tag effectiveness depends on the chosen voice.
2. **Use sparingly** — 2-3 tags per segment. Overuse sounds unnatural.
3. **Combine tags** — mix for complex emotions: `[laughs][sarcastic] Oh really?`
4. **Inform the user** — tell the user which tags were added and why.

## Stability Settings

Control voice variation via `--stability` and `--similarity`:

| Parameter | Range | Effect |
|-----------|-------|--------|
| `--stability` | 0.0-1.0 | Low = expressive, varied; High = stable, monotone |
| `--similarity` | 0.0-1.0 | High = closer to original voice; Low = more variation |

### Recommended Presets

| Scenario | stability | similarity | Notes |
|----------|-----------|------------|-------|
| Drama / dialogue | 0.2-0.4 | 0.5-0.7 | Max expressiveness, Audio Tags work best |
| General narration | 0.4-0.6 | 0.7-0.8 | Balanced (default when omitted) |
| News / technical | 0.7-0.9 | 0.8-1.0 | Highly stable, but Audio Tags barely respond |

```bash
# Dramatic reading — low stability lets Audio Tags shine
tapsvc-aigc audio speech -m elevenlabs/eleven_v3 \
  --voice alloy --stability 0.3 --similarity 0.7 \
  -i "[excited] This is incredible!" -o dramatic.mp3

# News broadcast — high stability for consistency
tapsvc-aigc audio speech -m elevenlabs/eleven_v3 \
  --voice amber --stability 0.8 --similarity 0.9 \
  -i "Today's market update..." -o news.mp3
```

> **Tip**: Omit `--stability` and `--similarity` to use API defaults.
> Only set these when the user explicitly wants more expressiveness or stability.

## Pronunciation Control

eleven_v3 does **not** support SSML `<break>` tags or `<phoneme>` tags.

Pacing and pronunciation rely entirely on:

### Pacing

- **Audio Tags**: `[pauses]`, `[hesitates]`, `[rushed]`, `[drawn out]`, etc.
- **Punctuation**: comma = short, period = medium, ellipsis = long, dash = shift
- **Capitalization**: ALL CAPS adds emphasis — `THIS is important`
- **Text structure**: line breaks and paragraphs affect breathing and rhythm

### Pronunciation Correction

When the model mispronounces a word, use in order:

1. **Phonetic respelling** — substitute with a similar-sounding spelling:
   - `NI-keh` (Nike), `An-THRO-pic` (Anthropic)
   - Use hyphens to split syllables, CAPS to mark stress
2. **Alias** — replace the word with the desired pronunciation directly

> **Warning**: Do NOT use `<phoneme>` tags with v3 — they are ignored or produce glitchy output.
