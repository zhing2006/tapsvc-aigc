---
name: tapsvc-aigc
description: >
  Generate and edit images, synthesize speech, and create or edit videos through
  the TapSvc AI Gateway with the tapsvc-aigc CLI. Use when a user requests AIGC
  media, prompt optimization, model selection, or generation parameter guidance.
---

# tapsvc-aigc AIGC Skill

Generate images, audio, and videos through the `tapsvc-aigc` CLI.
Follow the workflow below for every request. Read the matching reference
file — it contains models, CLI parameters, constraints, and prompt best
practices.

## Workflow

Execute every step in order.

### Step 0 — Preflight

1. Check that `tapsvc-aigc` is available:
   - If your platform provides a skill directory variable (e.g. `${CLAUDE_SKILL_DIR}`),
     try `<skill-dir>/scripts/tapsvc-aigc --help` (on Windows: `tapsvc-aigc.exe`).
   - If that fails or no skill directory variable exists, fall back to PATH.
   - Only fail if both checks find nothing.
2. If no `.env` file exists in the working directory, check that `TAPSVC_BASE_URL`
   and `TAPSVC_API_KEY` are set. If `.env` exists, skip this check — the CLI
   loads it automatically at startup.
3. When checking current model availability, call `GET /v1/models`. Treat the
   result as a flat cross-protocol catalog: a listed model is not evidence that
   it works on every endpoint. Match the requested modality to the protocol in
   the corresponding reference.

If the binary is not found, or env vars are missing without a `.env`, **stop**
and tell the user what is missing.

**Important**: Run the CLI from the current working directory — do NOT cd into
the skill directory. The CLI loads `.env` from CWD and writes output files
to CWD.

### Step 1 — Optimize Prompt

Read the reference for the requested modality and optimize the user's input:

- **Image** → [references/image-generation.md](references/image-generation.md)
- **Audio** → [references/audio-generation.md](references/audio-generation.md)
- **Video** → [references/video-generation.md](references/video-generation.md)

Show the optimized prompt to the user before proceeding.

### Step 2 — Execute

Build and run the command per the reference file read in Step 1.

### Step 3 — Deliver Result

- If a delivery channel is available (file transfer, message push, etc.),
  send the result through it.
- Otherwise, report the output file path and relevant details.
