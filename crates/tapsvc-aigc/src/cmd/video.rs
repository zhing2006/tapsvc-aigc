use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use chrono::Local;
use tapsvc_aigc_ark::ArkClient;
use tapsvc_aigc_ark::video::{
    AudioUrlData, ContentItem, CreateVideoTaskRequest, ImageUrlData, ListVideoTasksFilter,
    VideoTask, VideoTaskTool, VideoUrlData,
};
use tapsvc_aigc_dashscope::DashScopeClient;
use tapsvc_aigc_dashscope::video::{
    CreateVideoTaskRequest as DashScopeCreateVideoTaskRequest, MediaItem,
    VideoInput as DashScopeVideoInput, VideoParameters as DashScopeVideoParameters,
    VideoTask as DashScopeVideoTask,
};

use crate::cli::VideoCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HappyHorseMode {
    TextToVideo,
    ImageToVideo,
    ReferenceToVideo,
    VideoEdit,
}

impl HappyHorseMode {
    fn from_model(model: &str) -> Option<Self> {
        match model {
            "happyhorse-1.1-t2v" => Some(Self::TextToVideo),
            "happyhorse-1.1-i2v" => Some(Self::ImageToVideo),
            "happyhorse-1.1-r2v" => Some(Self::ReferenceToVideo),
            "happyhorse-1.0-video-edit" => Some(Self::VideoEdit),
            _ => None,
        }
    }

    fn supports_ratio(self) -> bool {
        matches!(self, Self::TextToVideo | Self::ReferenceToVideo)
    }
}

struct HappyHorseGenerateOptions {
    mode: HappyHorseMode,
    model: String,
    prompt: Option<String>,
    first_frame: Option<String>,
    ref_image: Vec<String>,
    ref_video: Vec<String>,
    resolution: String,
    aspect_ratio: String,
    duration: i32,
    watermark: bool,
    seed: Option<u64>,
    poll_interval: u64,
    timeout: u64,
    output: Option<String>,
}

pub async fn handle(command: VideoCommand) -> anyhow::Result<()> {
    match command {
        VideoCommand::Generate {
            model,
            prompt,
            prompt_file,
            first_frame,
            last_frame,
            ref_image,
            ref_video,
            ref_audio,
            resolution,
            aspect_ratio,
            duration,
            no_audio,
            watermark,
            web_search,
            camera_fixed,
            seed,
            output_format,
            priority,
            expires_after,
            return_last_frame,
            poll_interval,
            timeout,
            output,
        } => {
            // ── Validate inputs ──

            let seedance_2_5 = is_seedance_2_5(&model);

            let has_prompt = prompt.is_some() || prompt_file.is_some();
            let has_first_frame = first_frame.is_some();
            let has_ref_image = !ref_image.is_empty();
            let has_ref_video = !ref_video.is_empty();
            let has_ref_audio = !ref_audio.is_empty();

            // Seedance 2.5 can drive a video from reference audio alone.
            let audio_alone = seedance_2_5 && has_ref_audio;
            if !has_prompt && !has_first_frame && !has_ref_image && !has_ref_video && !audio_alone {
                bail!(
                    "at least one input is required: --prompt, --prompt-file, --first-frame, --ref-image, or --ref-video (Seedance 2.5 also accepts --ref-audio alone)"
                );
            }

            if last_frame.is_some() && !has_first_frame {
                bail!("--last-frame requires --first-frame");
            }

            if has_first_frame && (has_ref_image || has_ref_video) {
                bail!(
                    "--first-frame/--last-frame and --ref-image/--ref-video are mutually exclusive"
                );
            }

            if has_ref_audio && !has_ref_image && !has_ref_video && !seedance_2_5 {
                bail!("--ref-audio requires --ref-image or --ref-video on {model}");
            }

            let (max_ref_images, max_ref_videos, max_ref_audios) = if seedance_2_5 {
                (30, 10, 10)
            } else {
                (9, 3, 3)
            };
            if ref_image.len() > max_ref_images {
                bail!(
                    "--ref-image supports at most {max_ref_images} images on {model}, got {}",
                    ref_image.len()
                );
            }
            if ref_video.len() > max_ref_videos {
                bail!(
                    "--ref-video supports at most {max_ref_videos} videos on {model}, got {}",
                    ref_video.len()
                );
            }
            if ref_audio.len() > max_ref_audios {
                bail!(
                    "--ref-audio supports at most {max_ref_audios} audio files on {model}, got {}",
                    ref_audio.len()
                );
            }

            let happyhorse_mode = HappyHorseMode::from_model(&model);
            if happyhorse_mode.is_none() && model.starts_with("happyhorse-") {
                bail!("unsupported HappyHorse model: {model}");
            }
            validate_camera_fixed(&model, happyhorse_mode, camera_fixed)?;
            validate_seedance_only_options(
                happyhorse_mode,
                output_format.as_deref(),
                priority,
                expires_after,
                return_last_frame,
            )?;
            if happyhorse_mode == Some(HappyHorseMode::VideoEdit) && duration.is_some() {
                bail!("happyhorse-1.0-video-edit does not support --duration");
            }
            let duration = duration.unwrap_or(5);
            validate_duration(&model, happyhorse_mode, duration)?;
            validate_resolution(&model, happyhorse_mode, &resolution)?;
            validate_aspect_ratio(happyhorse_mode, &aspect_ratio)?;
            validate_seedance_2_5_options(
                &model,
                has_first_frame,
                &aspect_ratio,
                output_format.as_deref(),
            )?;

            for url in &ref_video {
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    bail!("--ref-video only supports URLs (http:// or https://), got: {url}");
                }
            }

            let final_prompt = resolve_prompt(prompt.as_deref(), prompt_file.as_deref()).await?;

            if let Some(mode) = happyhorse_mode {
                validate_happyhorse_inputs(
                    mode,
                    final_prompt.as_deref(),
                    first_frame.as_deref(),
                    last_frame.as_deref(),
                    &ref_image,
                    &ref_video,
                    &ref_audio,
                    no_audio,
                    web_search,
                    seed,
                )?;

                return generate_happyhorse(HappyHorseGenerateOptions {
                    mode,
                    model,
                    prompt: final_prompt,
                    first_frame,
                    ref_image,
                    ref_video,
                    resolution,
                    aspect_ratio,
                    duration,
                    watermark,
                    seed,
                    poll_interval,
                    timeout,
                    output,
                })
                .await;
            }

            // ── Build content items ──

            let mut content: Vec<ContentItem> = Vec::new();

            // Prompt
            if let Some(text) = final_prompt {
                content.push(ContentItem::Text { text });
            }

            // First/last frame
            if let Some(ref path) = first_frame {
                let url = resolve_image_url(path).await?;
                content.push(ContentItem::ImageUrl {
                    image_url: ImageUrlData { url },
                    role: "first_frame".to_string(),
                });
            }
            if let Some(ref path) = last_frame {
                let url = resolve_image_url(path).await?;
                content.push(ContentItem::ImageUrl {
                    image_url: ImageUrlData { url },
                    role: "last_frame".to_string(),
                });
            }

            // Reference images
            for path in &ref_image {
                let url = resolve_image_url(path).await?;
                content.push(ContentItem::ImageUrl {
                    image_url: ImageUrlData { url },
                    role: "reference_image".to_string(),
                });
            }

            // Reference videos
            for url in &ref_video {
                content.push(ContentItem::VideoUrl {
                    video_url: VideoUrlData { url: url.clone() },
                    role: "reference_video".to_string(),
                });
            }

            // Reference audio
            for path in &ref_audio {
                let url = resolve_audio_url(path).await?;
                content.push(ContentItem::AudioUrl {
                    audio_url: AudioUrlData { url },
                    role: "reference_audio".to_string(),
                });
            }

            // ── Build request ──

            let tools = if web_search {
                Some(vec![VideoTaskTool {
                    type_: "web_search".to_string(),
                }])
            } else {
                None
            };

            // Kept for the default output file name after `output_format` moves into the request.
            let container = output_format.clone().unwrap_or_else(|| "mp4".to_string());

            let req = CreateVideoTaskRequest {
                model,
                content,
                resolution: Some(resolution),
                ratio: Some(aspect_ratio),
                duration: Some(duration),
                generate_audio: Some(!no_audio),
                watermark: Some(watermark),
                camera_fixed: if camera_fixed { Some(true) } else { None },
                seed,
                output_format,
                priority,
                execution_expires_after: expires_after,
                return_last_frame: if return_last_frame { Some(true) } else { None },
                tools,
            };

            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;
            let client = ArkClient::new(base_url, api_key);

            // ── Submit task ──

            let task_id_resp = client.create_video_task(&req).await?;
            let task_id = task_id_resp.id;
            eprintln!("Task created: {task_id}");

            // ── Poll loop ──

            let deadline = Instant::now() + std::time::Duration::from_secs(timeout);
            loop {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    eprintln!("Timeout after {timeout}s. Task ID: {task_id}");
                    eprintln!("You can check status later with: tapsvc-aigc video get {task_id}");
                    bail!("video generation timed out after {timeout}s");
                }

                let sleep_dur = remaining.min(std::time::Duration::from_secs(poll_interval));
                tokio::time::sleep(sleep_dur).await;

                let elapsed = (timeout as i64
                    - deadline.saturating_duration_since(Instant::now()).as_secs() as i64)
                    .max(0) as u64;

                let task = client.get_video_task(&task_id).await?;

                match task.status.as_str() {
                    "succeeded" => {
                        eprintln!("Task succeeded! ({elapsed}s elapsed)");

                        let video_url = task
                            .content
                            .as_ref()
                            .and_then(|c| c.video_url.as_deref())
                            .context("no video URL in task result")?;

                        let output_path =
                            download_video(video_url, output.as_deref(), &container).await?;
                        println!("{}", output_path.display());

                        // Only present when the task was created with return_last_frame;
                        // useful as the next clip's first frame.
                        if return_last_frame {
                            match task
                                .content
                                .as_ref()
                                .and_then(|c| c.last_frame_url.as_deref())
                            {
                                Some(url) => {
                                    let stem = output_path
                                        .file_stem()
                                        .map(|s| s.to_string_lossy().into_owned())
                                        .unwrap_or_else(|| "video".to_string());
                                    let frame_path = output_path
                                        .with_file_name(format!("{stem}_last_frame.png"));
                                    download_asset(url, &frame_path).await?;
                                    println!("{}", frame_path.display());
                                }
                                None => eprintln!(
                                    "warning: --return-last-frame was requested but the task returned no last frame URL"
                                ),
                            }
                        }
                        break;
                    }
                    "failed" => {
                        let err_msg = task
                            .error
                            .as_ref()
                            .map(|e| format!("{}: {}", e.code, e.message))
                            .unwrap_or_else(|| "unknown error".to_string());
                        bail!("video generation failed: {err_msg}");
                    }
                    "cancelled" => {
                        bail!("video generation was cancelled (task: {task_id})");
                    }
                    "expired" => {
                        bail!("video generation task expired before finishing (task: {task_id})");
                    }
                    other => {
                        eprintln!("Status: {other}, waiting... ({elapsed}s elapsed)");
                    }
                }
            }

            Ok(())
        }

        VideoCommand::Get { task_id, provider } => {
            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;

            if provider == "happyhorse" {
                let client = DashScopeClient::new(base_url, api_key);
                let response = client.get_video_task(&task_id).await?;
                print_happyhorse_task(&response.output);
            } else {
                let client = ArkClient::new(base_url, api_key);
                let task = client.get_video_task(&task_id).await?;
                print_task(&task);
            }

            Ok(())
        }

        VideoCommand::List {
            status,
            model,
            task_ids,
            page,
            page_size,
        } => {
            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;
            let client = ArkClient::new(base_url, api_key);

            let filter = ListVideoTasksFilter {
                page_num: Some(page),
                page_size: Some(page_size),
                status,
                model,
                task_ids,
            };

            let result = client.list_video_tasks(&filter).await?;

            println!("Total tasks: {}", result.total);
            println!("Page {page}, showing {} tasks", result.items.len());
            println!("{}", "-".repeat(80));

            for task in &result.items {
                print_task(task);
                println!("{}", "-".repeat(80));
            }

            Ok(())
        }

        VideoCommand::Delete { task_id } => {
            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;
            let client = ArkClient::new(base_url, api_key);

            client.delete_video_task(&task_id).await?;
            println!("Task deleted: {task_id}");

            Ok(())
        }
    }
}

// ── Helpers ──

fn validate_duration(
    model: &str,
    mode: Option<HappyHorseMode>,
    duration: i32,
) -> anyhow::Result<()> {
    match mode {
        Some(HappyHorseMode::VideoEdit) => {}
        Some(_) => {
            if !(3..=15).contains(&duration) {
                bail!("HappyHorse duration must be between 3 and 15 seconds");
            }
        }
        // Seedance 2.5 holds 30 coherent seconds in one pass; the 2.0 series stops at 15.
        None if is_seedance_2_5(model) => {
            if duration != -1 && !(4..=30).contains(&duration) {
                bail!("Seedance 2.5 duration must be between 4 and 30 seconds, or -1 for auto");
            }
        }
        None if duration != -1 && !(4..=15).contains(&duration) => {
            bail!("Seedance duration must be between 4 and 15 seconds, or -1 for auto");
        }
        None => {}
    }

    Ok(())
}

fn validate_resolution(
    model: &str,
    mode: Option<HappyHorseMode>,
    resolution: &str,
) -> anyhow::Result<()> {
    if mode.is_some() {
        if !matches!(resolution, "720p" | "1080p") {
            bail!("HappyHorse only supports 720p and 1080p");
        }
    } else if (model == "doubao-seedance-2-0-fast-260128" || is_seedance_2_5(model))
        && !matches!(resolution, "480p" | "720p")
    {
        bail!(
            "model {model} only supports 480p and 720p; use doubao-seedance-2-0-260128 for {resolution}"
        );
    }

    Ok(())
}

fn validate_aspect_ratio(mode: Option<HappyHorseMode>, aspect_ratio: &str) -> anyhow::Result<()> {
    const SEEDANCE_RATIOS: &[&str] = &["16:9", "4:3", "1:1", "3:4", "9:16", "21:9", "adaptive"];
    const HAPPYHORSE_RATIOS: &[&str] = &[
        "16:9", "9:16", "1:1", "4:3", "3:4", "4:5", "5:4", "9:21", "21:9", "adaptive",
    ];

    match mode {
        Some(mode) if mode.supports_ratio() => {
            if !HAPPYHORSE_RATIOS.contains(&aspect_ratio) {
                bail!("unsupported HappyHorse aspect ratio: {aspect_ratio}");
            }
        }
        Some(_) if aspect_ratio != "adaptive" => {
            bail!("--aspect-ratio is not supported by this HappyHorse model");
        }
        Some(_) => {}
        None if !SEEDANCE_RATIOS.contains(&aspect_ratio) => {
            bail!("unsupported Seedance aspect ratio: {aspect_ratio}");
        }
        None => {}
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_happyhorse_inputs(
    mode: HappyHorseMode,
    prompt: Option<&str>,
    first_frame: Option<&str>,
    last_frame: Option<&str>,
    ref_image: &[String],
    ref_video: &[String],
    ref_audio: &[String],
    no_audio: bool,
    web_search: bool,
    seed: Option<u64>,
) -> anyhow::Result<()> {
    let has_prompt = prompt.is_some_and(|value| !value.trim().is_empty());

    match mode {
        HappyHorseMode::TextToVideo => {
            if !has_prompt {
                bail!("happyhorse-1.1-t2v requires --prompt or --prompt-file");
            }
            if first_frame.is_some() || !ref_image.is_empty() || !ref_video.is_empty() {
                bail!("happyhorse-1.1-t2v only accepts a text prompt");
            }
        }
        HappyHorseMode::ImageToVideo => {
            if first_frame.is_none() {
                bail!("happyhorse-1.1-i2v requires exactly one --first-frame");
            }
            if !ref_image.is_empty() || !ref_video.is_empty() {
                bail!("happyhorse-1.1-i2v does not accept reference media");
            }
        }
        HappyHorseMode::ReferenceToVideo => {
            if !has_prompt {
                bail!("happyhorse-1.1-r2v requires --prompt or --prompt-file");
            }
            if ref_image.is_empty() {
                bail!("happyhorse-1.1-r2v requires 1 to 9 --ref-image values");
            }
            if first_frame.is_some() || !ref_video.is_empty() {
                bail!("happyhorse-1.1-r2v only accepts reference images");
            }
        }
        HappyHorseMode::VideoEdit => {
            if !has_prompt {
                bail!("happyhorse-1.0-video-edit requires --prompt or --prompt-file");
            }
            if ref_video.len() != 1 {
                bail!("happyhorse-1.0-video-edit requires exactly one --ref-video URL");
            }
            if ref_image.len() > 5 {
                bail!("happyhorse-1.0-video-edit supports at most 5 --ref-image values");
            }
            if first_frame.is_some() {
                bail!("happyhorse-1.0-video-edit does not accept --first-frame");
            }
        }
    }

    if last_frame.is_some() {
        bail!("HappyHorse models do not support --last-frame");
    }
    if !ref_audio.is_empty() {
        bail!("HappyHorse models do not support --ref-audio");
    }
    if no_audio {
        bail!("HappyHorse models do not support --no-audio");
    }
    if web_search {
        bail!("HappyHorse models do not support --web-search");
    }
    if seed.is_some_and(|value| value > 2_147_483_647) {
        bail!("HappyHorse seed must be between 0 and 2147483647");
    }

    Ok(())
}

fn validate_camera_fixed(
    model: &str,
    happyhorse_mode: Option<HappyHorseMode>,
    camera_fixed: bool,
) -> anyhow::Result<()> {
    if !camera_fixed {
        return Ok(());
    }
    if happyhorse_mode.is_some() {
        bail!("HappyHorse models do not support --camera-fixed");
    }
    if is_seedance_2_5(model) {
        bail!(
            "Seedance 2.5 does not support --camera-fixed; describe a static or locked-off shot in the prompt instead"
        );
    }
    if model.starts_with("doubao-seedance-2-0") || model.starts_with("doubao-seedance-2.0") {
        bail!(
            "Seedance 2.0 models do not support --camera-fixed; describe a static or locked-off shot in the prompt instead"
        );
    }

    Ok(())
}

/// Seedance 2.5 is exposed as `bytedance/seedance-2.5` through the gateway and as
/// `doubao-seedance-2-5-*` on ARK directly; match both spellings.
fn is_seedance_2_5(model: &str) -> bool {
    let model = model.to_ascii_lowercase();
    model.contains("seedance-2.5") || model.contains("seedance-2-5")
}

/// Request fields the ARK API accepts only from Seedance 2.5, plus the ratio
/// restriction it enforces on first-frame tasks.
fn validate_seedance_2_5_options(
    model: &str,
    has_first_frame: bool,
    aspect_ratio: &str,
    output_format: Option<&str>,
) -> anyhow::Result<()> {
    if is_seedance_2_5(model) {
        // The output ratio always follows the first frame, so an explicit one is rejected.
        if has_first_frame && aspect_ratio != "adaptive" {
            bail!(
                "{model} only supports --aspect-ratio adaptive with --first-frame; the output follows the first frame"
            );
        }
    } else if output_format.is_some() {
        bail!("--output-format is only supported by Seedance 2.5, not {model}");
    }

    Ok(())
}

/// ARK-only request fields; the DashScope API behind HappyHorse has no equivalent.
fn validate_seedance_only_options(
    happyhorse_mode: Option<HappyHorseMode>,
    output_format: Option<&str>,
    priority: Option<u8>,
    expires_after: Option<u32>,
    return_last_frame: bool,
) -> anyhow::Result<()> {
    if happyhorse_mode.is_none() {
        return Ok(());
    }
    if output_format.is_some() {
        bail!("HappyHorse models do not support --output-format");
    }
    if priority.is_some() {
        bail!("HappyHorse models do not support --priority");
    }
    if expires_after.is_some() {
        bail!("HappyHorse models do not support --expires-after");
    }
    if return_last_frame {
        bail!("HappyHorse models do not support --return-last-frame");
    }

    Ok(())
}

async fn generate_happyhorse(options: HappyHorseGenerateOptions) -> anyhow::Result<()> {
    let HappyHorseGenerateOptions {
        mode,
        model,
        prompt,
        first_frame,
        ref_image,
        ref_video,
        resolution,
        aspect_ratio,
        duration,
        watermark,
        seed,
        poll_interval,
        timeout,
        output,
    } = options;

    let mut media = Vec::new();
    match mode {
        HappyHorseMode::TextToVideo => {}
        HappyHorseMode::ImageToVideo => {
            let frame = first_frame.context("missing first frame after validation")?;
            media.push(MediaItem {
                type_: "first_frame".to_string(),
                url: resolve_image_url(&frame).await?,
            });
        }
        HappyHorseMode::ReferenceToVideo => {
            for image in ref_image {
                media.push(MediaItem {
                    type_: "reference_image".to_string(),
                    url: resolve_image_url(&image).await?,
                });
            }
        }
        HappyHorseMode::VideoEdit => {
            let video = ref_video
                .into_iter()
                .next()
                .context("missing reference video after validation")?;
            media.push(MediaItem {
                type_: "video".to_string(),
                url: video,
            });
            for image in ref_image {
                media.push(MediaItem {
                    type_: "reference_image".to_string(),
                    url: resolve_image_url(&image).await?,
                });
            }
        }
    }

    let ratio = mode.supports_ratio().then(|| {
        if aspect_ratio == "adaptive" {
            "16:9".to_string()
        } else {
            aspect_ratio
        }
    });
    let request = DashScopeCreateVideoTaskRequest {
        model,
        input: DashScopeVideoInput { prompt, media },
        parameters: DashScopeVideoParameters {
            resolution: Some(resolution.to_ascii_uppercase()),
            ratio,
            duration: (mode != HappyHorseMode::VideoEdit).then_some(duration),
            watermark: Some(watermark),
            seed,
        },
    };

    let base_url = std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
    let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;
    let client = DashScopeClient::new(base_url, api_key);

    let created = client.create_video_task(&request).await?;
    let task_id = created
        .output
        .task_id
        .context("no task ID in HappyHorse create response")?;
    eprintln!("Task created: {task_id}");

    let deadline = Instant::now() + std::time::Duration::from_secs(timeout);
    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            eprintln!("Timeout after {timeout}s. Task ID: {task_id}");
            eprintln!(
                "You can check status later with: tapsvc-aigc video get {task_id} --provider happyhorse"
            );
            bail!("video generation timed out after {timeout}s");
        }

        let sleep_duration = remaining.min(std::time::Duration::from_secs(poll_interval.max(1)));
        tokio::time::sleep(sleep_duration).await;

        let elapsed = (timeout as i64
            - deadline.saturating_duration_since(Instant::now()).as_secs() as i64)
            .max(0) as u64;
        let response = client.get_video_task(&task_id).await?;
        let task = response.output;

        match task.task_status.to_ascii_uppercase().as_str() {
            "SUCCEEDED" => {
                eprintln!("Task succeeded! ({elapsed}s elapsed)");
                let video_url = task
                    .video_url
                    .as_deref()
                    .context("no video URL in HappyHorse task result")?;
                let output_path = download_video(video_url, output.as_deref(), "mp4").await?;
                println!("{}", output_path.display());
                return Ok(());
            }
            "FAILED" => {
                let message = match (task.code, task.message) {
                    (Some(code), Some(message)) => format!("{code}: {message}"),
                    (Some(code), None) => code,
                    (None, Some(message)) => message,
                    (None, None) => "unknown error".to_string(),
                };
                bail!("video generation failed: {message}");
            }
            "CANCELED" | "CANCELLED" => {
                bail!("video generation was cancelled (task: {task_id})");
            }
            status => {
                if status == "UNKNOWN" {
                    bail!("HappyHorse task is unknown or expired: {task_id}");
                }
                eprintln!("Status: {status}, waiting... ({elapsed}s elapsed)");
            }
        }
    }
}

async fn download_video(
    url: &str,
    output: Option<&str>,
    container: &str,
) -> anyhow::Result<PathBuf> {
    let output_path = output.map(PathBuf::from).unwrap_or_else(|| {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(format!("video_{timestamp}.{container}"))
    });
    download_asset(url, &output_path).await?;

    Ok(output_path)
}

async fn download_asset(url: &str, output_path: &Path) -> anyhow::Result<()> {
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .context("failed to download generated asset")?
        .error_for_status()
        .context("asset download returned an error status")?;
    let bytes = response
        .bytes()
        .await
        .context("failed to read generated asset")?;

    if let Some(parent) = output_path.parent()
        && !parent.as_os_str().is_empty()
    {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }
    tokio::fs::write(output_path, bytes)
        .await
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    Ok(())
}

async fn resolve_prompt(
    prompt: Option<&str>,
    prompt_file: Option<&str>,
) -> anyhow::Result<Option<String>> {
    let file_content = match prompt_file {
        Some(path) => {
            let content = tokio::fs::read_to_string(path)
                .await
                .with_context(|| format!("failed to read prompt file: {path}"))?;
            Some(content)
        }
        None => None,
    };

    match (file_content, prompt) {
        (Some(file), Some(p)) => Ok(Some(format!("{}\n{}", file.trim_end(), p))),
        (Some(file), None) => Ok(Some(file)),
        (None, Some(p)) => Ok(Some(p.to_string())),
        (None, None) => Ok(None),
    }
}

fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://") || s.starts_with("data:")
}

async fn resolve_image_url(path_or_url: &str) -> anyhow::Result<String> {
    if is_url(path_or_url) {
        return Ok(path_or_url.to_string());
    }
    encode_file_as_data_uri(path_or_url, "image").await
}

async fn resolve_audio_url(path_or_url: &str) -> anyhow::Result<String> {
    if is_url(path_or_url) {
        return Ok(path_or_url.to_string());
    }
    encode_file_as_data_uri(path_or_url, "audio").await
}

async fn encode_file_as_data_uri(path: &str, media_type: &str) -> anyhow::Result<String> {
    let bytes = tokio::fs::read(path)
        .await
        .with_context(|| format!("failed to read file: {path}"))?;

    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();

    let fmt = match ext.as_str() {
        "jpg" | "jpeg" => "jpeg",
        "png" => "png",
        "webp" => "webp",
        "gif" => "gif",
        "bmp" => "bmp",
        "wav" => "wav",
        "mp3" => "mpeg",
        "ogg" => "ogg",
        "flac" => "flac",
        "aac" => "aac",
        other => other,
    };

    let encoded = BASE64.encode(&bytes);
    Ok(format!("data:{media_type}/{fmt};base64,{encoded}"))
}

fn format_timestamp(ts: Option<u64>) -> String {
    match ts {
        Some(ts) => {
            let dt = chrono::DateTime::from_timestamp(ts as i64, 0);
            match dt {
                Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                None => "-".to_string(),
            }
        }
        None => "-".to_string(),
    }
}

fn print_task(task: &VideoTask) {
    println!("  ID:         {}", task.id);
    println!("  Model:      {}", task.model);
    println!("  Status:     {}", task.status);
    if let Some(d) = task.duration {
        println!("  Duration:   {d}s");
    }
    if let Some(ref r) = task.ratio {
        println!("  Ratio:      {r}");
    }
    if let Some(ref r) = task.resolution {
        println!("  Resolution: {r}");
    }
    println!("  Created:    {}", format_timestamp(task.created_at));
    println!("  Updated:    {}", format_timestamp(task.updated_at));
    if let Some(ref content) = task.content
        && let Some(ref url) = content.video_url
    {
        println!("  Video URL:  {url}");
    }
    if let Some(ref err) = task.error {
        println!("  Error:      {} - {}", err.code, err.message);
    }
}

fn print_happyhorse_task(task: &DashScopeVideoTask) {
    println!("  ID:         {}", task.task_id.as_deref().unwrap_or("-"));
    println!("  Status:     {}", task.task_status);
    if let Some(ref prompt) = task.orig_prompt {
        println!("  Prompt:     {prompt}");
    }
    if let Some(ref url) = task.video_url {
        println!("  Video URL:  {url}");
    }
    if task.code.is_some() || task.message.is_some() {
        println!(
            "  Error:      {}{}{}",
            task.code.as_deref().unwrap_or("unknown"),
            if task.code.is_some() && task.message.is_some() {
                " - "
            } else {
                ""
            },
            task.message.as_deref().unwrap_or("")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HappyHorseMode, validate_camera_fixed, validate_duration, validate_happyhorse_inputs,
        validate_resolution, validate_seedance_2_5_options, validate_seedance_only_options,
    };

    #[test]
    fn fast_model_rejects_high_resolutions() {
        assert!(validate_resolution("doubao-seedance-2-0-fast-260128", None, "1080p").is_err());
        assert!(validate_resolution("doubao-seedance-2-0-fast-260128", None, "4k").is_err());
    }

    #[test]
    fn full_model_accepts_high_resolutions() {
        assert!(validate_resolution("doubao-seedance-2-0-260128", None, "1080p").is_ok());
        assert!(validate_resolution("doubao-seedance-2-0-260128", None, "4k").is_ok());
    }

    #[test]
    fn happyhorse_has_distinct_resolution_and_duration_ranges() {
        let mode = Some(HappyHorseMode::TextToVideo);
        assert!(validate_resolution("happyhorse-1.1-t2v", mode, "480p").is_err());
        assert!(validate_resolution("happyhorse-1.1-t2v", mode, "1080p").is_ok());
        assert!(validate_duration("happyhorse-1.1-t2v", mode, 3).is_ok());
        assert!(validate_duration("happyhorse-1.1-t2v", mode, 16).is_err());
    }

    #[test]
    fn seedance_2_5_extends_duration_and_caps_resolution() {
        assert!(validate_duration("bytedance/seedance-2.5", None, 30).is_ok());
        assert!(validate_duration("bytedance/seedance-2.5", None, 31).is_err());
        assert!(validate_duration("doubao-seedance-2-0-260128", None, 30).is_err());
        assert!(validate_resolution("bytedance/seedance-2.5", None, "720p").is_ok());
        assert!(validate_resolution("bytedance/seedance-2.5", None, "1080p").is_err());
    }

    #[test]
    fn seedance_2_5_rejects_camera_fixed_and_owns_output_format() {
        assert!(validate_camera_fixed("bytedance/seedance-2.5", None, true).is_err());
        assert!(
            validate_seedance_2_5_options("bytedance/seedance-2.5", false, "adaptive", Some("mov"))
                .is_ok()
        );
        // The output ratio follows the first frame, so an explicit ratio is rejected.
        assert!(
            validate_seedance_2_5_options("bytedance/seedance-2.5", true, "16:9", None).is_err()
        );
        assert!(
            validate_seedance_2_5_options("bytedance/seedance-2.5", true, "adaptive", None).is_ok()
        );
        // mov is exclusive to 2.5.
        assert!(
            validate_seedance_2_5_options("doubao-seedance-2-0-260128", false, "16:9", Some("mov"))
                .is_err()
        );
    }

    #[test]
    fn happyhorse_rejects_seedance_only_options() {
        let mode = Some(HappyHorseMode::TextToVideo);
        assert!(validate_seedance_only_options(mode, Some("mov"), None, None, false).is_err());
        assert!(validate_seedance_only_options(mode, None, Some(5), None, false).is_err());
        assert!(validate_seedance_only_options(mode, None, None, Some(3600), false).is_err());
        assert!(validate_seedance_only_options(mode, None, None, None, true).is_err());
        assert!(
            validate_seedance_only_options(None, Some("mov"), Some(5), Some(3600), true).is_ok()
        );
    }

    #[test]
    fn happyhorse_reference_mode_requires_prompt_and_images() {
        let no_media = Vec::new();
        assert!(
            validate_happyhorse_inputs(
                HappyHorseMode::ReferenceToVideo,
                Some("Use [Image 1] as the character"),
                None,
                None,
                &no_media,
                &no_media,
                &no_media,
                false,
                false,
                None,
            )
            .is_err()
        );

        let images = vec!["reference.png".to_string()];
        assert!(
            validate_happyhorse_inputs(
                HappyHorseMode::ReferenceToVideo,
                Some("Use [Image 1] as the character"),
                None,
                None,
                &images,
                &no_media,
                &no_media,
                false,
                false,
                Some(42),
            )
            .is_ok()
        );
    }

    #[test]
    fn camera_fixed_rejects_current_video_models() {
        assert!(validate_camera_fixed("doubao-seedance-2-0-260128", None, true).is_err());
        assert!(validate_camera_fixed("doubao-seedance-2.0", None, true).is_err());
        assert!(
            validate_camera_fixed(
                "happyhorse-1.1-t2v",
                Some(HappyHorseMode::TextToVideo),
                true,
            )
            .is_err()
        );
        assert!(validate_camera_fixed("doubao-seedance-1-0-pro-250528", None, true).is_ok());
    }
}
