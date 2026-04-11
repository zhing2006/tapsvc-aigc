use std::path::Path;
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

use crate::cli::VideoCommand;

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
            poll_interval,
            timeout,
            output,
        } => {
            // ── Validate inputs ──

            let has_prompt = prompt.is_some() || prompt_file.is_some();
            let has_first_frame = first_frame.is_some();
            let has_ref_image = !ref_image.is_empty();
            let has_ref_video = !ref_video.is_empty();
            let has_ref_audio = !ref_audio.is_empty();

            if !has_prompt && !has_first_frame && !has_ref_image && !has_ref_video {
                bail!(
                    "at least one input is required: --prompt, --prompt-file, --first-frame, --ref-image, or --ref-video"
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

            if has_ref_audio && !has_ref_image && !has_ref_video {
                bail!("--ref-audio requires --ref-image or --ref-video");
            }

            if ref_image.len() > 9 {
                bail!(
                    "--ref-image supports at most 9 images, got {}",
                    ref_image.len()
                );
            }
            if ref_video.len() > 3 {
                bail!(
                    "--ref-video supports at most 3 videos, got {}",
                    ref_video.len()
                );
            }
            if ref_audio.len() > 3 {
                bail!(
                    "--ref-audio supports at most 3 audio files, got {}",
                    ref_audio.len()
                );
            }

            if duration != -1 && !(4..=15).contains(&duration) {
                bail!("--duration must be 4-15 seconds, or -1 for auto");
            }

            for url in &ref_video {
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    bail!("--ref-video only supports URLs (http:// or https://), got: {url}");
                }
            }

            // ── Build content items ──

            let mut content: Vec<ContentItem> = Vec::new();

            // Prompt
            let final_prompt = resolve_prompt(prompt.as_deref(), prompt_file.as_deref()).await?;
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

                        // Download video
                        let output_path = match output {
                            Some(p) => std::path::PathBuf::from(p),
                            None => {
                                let ts = Local::now().format("%Y%m%d_%H%M%S");
                                std::path::PathBuf::from(format!("video_{ts}.mp4"))
                            }
                        };

                        eprintln!("Downloading video...");
                        let resp = reqwest::get(video_url)
                            .await
                            .context("failed to download video")?;

                        if !resp.status().is_success() {
                            bail!("failed to download video: HTTP {}", resp.status());
                        }

                        let bytes = resp.bytes().await.context("failed to read video data")?;

                        if let Some(parent) = output_path.parent()
                            && !parent.as_os_str().is_empty()
                        {
                            tokio::fs::create_dir_all(parent).await.with_context(|| {
                                format!("failed to create directory {}", parent.display())
                            })?;
                        }

                        tokio::fs::write(&output_path, &bytes)
                            .await
                            .with_context(|| {
                                format!("failed to write {}", output_path.display())
                            })?;

                        println!("{}", output_path.display());
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
                    other => {
                        eprintln!("Status: {other}, waiting... ({elapsed}s elapsed)");
                    }
                }
            }

            Ok(())
        }

        VideoCommand::Get { task_id } => {
            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;
            let client = ArkClient::new(base_url, api_key);

            let task = client.get_video_task(&task_id).await?;
            print_task(&task);

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
