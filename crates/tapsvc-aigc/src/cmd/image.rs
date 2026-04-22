use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use chrono::Local;
use tapsvc_aigc_openai::OpenAiClient;
use tapsvc_aigc_openai::image::{CreateImageRequest, EditImageRequest};

use crate::cli::ImageCommand;

pub async fn handle(command: ImageCommand) -> anyhow::Result<()> {
    match command {
        ImageCommand::Generate {
            model,
            prompt,
            prompt_file,
            size,
            n,
            quality,
            response_format,
            background,
            output,
        } => {
            let final_prompt = resolve_prompt(prompt.as_deref(), prompt_file.as_deref()).await?;

            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;

            let client = OpenAiClient::new(base_url, api_key);

            let req = CreateImageRequest {
                model,
                prompt: final_prompt,
                n: Some(n),
                size: size_to_param(&size),
                quality: Some(quality),
                response_format: Some("b64_json".to_string()),
                background: Some(background),
                output_format: Some(response_format.clone()),
            };

            let response = client.create_image(&req).await?;

            let ext = format_to_ext(&response_format);
            let output_paths = build_output_paths(output.as_deref(), n, &ext, "image");

            let mut written = 0usize;
            for (i, item) in response.data.iter().enumerate() {
                let Some(path) = output_paths.get(i) else {
                    eprintln!(
                        "warning: API returned more images than requested ({}), ignoring extra",
                        i + 1
                    );
                    break;
                };

                if let Some(revised) = &item.revised_prompt {
                    eprintln!("[image {}] revised prompt: {}", i + 1, revised);
                }

                let bytes = if let Some(data) = &item.b64_json {
                    BASE64
                        .decode(data)
                        .with_context(|| format!("failed to decode base64 for image {}", i + 1))?
                } else if let Some(url) = &item.url {
                    client
                        .download_bytes(url)
                        .await
                        .with_context(|| format!("failed to download image {} from url", i + 1))?
                } else {
                    eprintln!(
                        "warning: image {} has neither b64_json nor url, skipping",
                        i + 1
                    );
                    continue;
                };

                if let Some(parent) = path.parent()
                    && !parent.as_os_str().is_empty()
                {
                    tokio::fs::create_dir_all(parent).await.with_context(|| {
                        format!("failed to create directory {}", parent.display())
                    })?;
                }

                tokio::fs::write(path, &bytes)
                    .await
                    .with_context(|| format!("failed to write {}", path.display()))?;

                println!("{}", path.display());
                written += 1;
            }

            if written == 0 {
                bail!("API returned no valid image data");
            }

            Ok(())
        }
        ImageCommand::Edit {
            model,
            image,
            prompt,
            prompt_file,
            mask,
            size,
            n,
            response_format,
            output,
        } => {
            let final_prompt = resolve_prompt(prompt.as_deref(), prompt_file.as_deref()).await?;

            // Validate and read image file
            validate_image_ext(&image)?;
            let image_bytes = tokio::fs::read(&image)
                .await
                .with_context(|| format!("failed to read image file: {image}"))?;
            validate_file_size(&image, image_bytes.len(), 25 * 1024 * 1024)?;
            let image_filename = Path::new(&image)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| "image.png".to_string());

            // Validate and read mask file (if provided)
            let (mask_bytes, mask_filename) = match &mask {
                Some(mask_path) => {
                    validate_mask_ext(mask_path)?;
                    let bytes = tokio::fs::read(mask_path)
                        .await
                        .with_context(|| format!("failed to read mask file: {mask_path}"))?;
                    validate_file_size(mask_path, bytes.len(), 4 * 1024 * 1024)?;
                    let fname = Path::new(mask_path)
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_else(|| "mask.png".to_string());
                    (Some(bytes), Some(fname))
                }
                None => (None, None),
            };

            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;

            let client = OpenAiClient::new(base_url, api_key);

            let req = EditImageRequest {
                model,
                prompt: final_prompt,
                image_bytes,
                image_filename,
                mask_bytes,
                mask_filename,
                n: Some(n),
                size: size_to_param(&size),
                output_format: Some(response_format.clone()),
            };

            let response = client.edit_image(&req).await?;

            let ext = format_to_ext(&response_format);
            let output_paths = build_output_paths(output.as_deref(), n, &ext, "edited");

            let mut written = 0usize;
            for (i, item) in response.data.iter().enumerate() {
                let Some(path) = output_paths.get(i) else {
                    eprintln!(
                        "warning: API returned more images than requested ({}), ignoring extra",
                        i + 1
                    );
                    break;
                };

                if let Some(revised) = &item.revised_prompt {
                    eprintln!("[image {}] revised prompt: {}", i + 1, revised);
                }

                let bytes = if let Some(data) = &item.b64_json {
                    BASE64
                        .decode(data)
                        .with_context(|| format!("failed to decode base64 for image {}", i + 1))?
                } else if let Some(url) = &item.url {
                    client
                        .download_bytes(url)
                        .await
                        .with_context(|| format!("failed to download image {} from url", i + 1))?
                } else {
                    eprintln!(
                        "warning: image {} has neither b64_json nor url, skipping",
                        i + 1
                    );
                    continue;
                };

                if let Some(parent) = path.parent()
                    && !parent.as_os_str().is_empty()
                {
                    tokio::fs::create_dir_all(parent).await.with_context(|| {
                        format!("failed to create directory {}", parent.display())
                    })?;
                }

                tokio::fs::write(path, &bytes)
                    .await
                    .with_context(|| format!("failed to write {}", path.display()))?;

                println!("{}", path.display());
                written += 1;
            }

            if written == 0 {
                bail!("API returned no valid image data");
            }

            Ok(())
        }
    }
}

fn size_to_param(size: &str) -> Option<String> {
    if size == "auto" {
        None
    } else {
        Some(size.to_string())
    }
}

async fn resolve_prompt(prompt: Option<&str>, prompt_file: Option<&str>) -> anyhow::Result<String> {
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
        (Some(file), Some(p)) => Ok(format!("{}\n{}", file.trim_end(), p)),
        (Some(file), None) => Ok(file),
        (None, Some(p)) => Ok(p.to_string()),
        (None, None) => bail!("either --prompt or --prompt-file must be provided"),
    }
}

fn validate_image_ext(path: &str) -> anyhow::Result<()> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("png" | "jpg" | "jpeg" | "webp") => Ok(()),
        _ => bail!("unsupported image format: {path} (only PNG, JPEG, WebP are supported)"),
    }
}

fn validate_mask_ext(path: &str) -> anyhow::Result<()> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("png") => Ok(()),
        _ => bail!("unsupported mask format: {path} (only PNG is supported)"),
    }
}

fn validate_file_size(path: &str, size: usize, max: usize) -> anyhow::Result<()> {
    if size > max {
        let max_mb = max / (1024 * 1024);
        bail!("file too large: {path} ({} bytes, max {max_mb}MB)", size);
    }
    Ok(())
}

fn format_to_ext(format: &str) -> String {
    match format {
        "jpeg" => "jpg".to_string(),
        other => other.to_string(),
    }
}

fn build_output_paths(output: Option<&str>, n: u32, ext: &str, prefix: &str) -> Vec<PathBuf> {
    match output {
        Some(path) if n == 1 => vec![PathBuf::from(path)],
        Some(path) => {
            let p = Path::new(path);
            let stem = p
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| prefix.to_string());
            let parent = p.parent().unwrap_or(Path::new(""));
            (1..=n)
                .map(|i| parent.join(format!("{stem}_{i}.{ext}")))
                .collect()
        }
        None => {
            let ts = Local::now().format("%Y%m%d_%H%M%S");
            if n == 1 {
                vec![PathBuf::from(format!("{prefix}_{ts}.{ext}"))]
            } else {
                (1..=n)
                    .map(|i| PathBuf::from(format!("{prefix}_{ts}_{i}.{ext}")))
                    .collect()
            }
        }
    }
}
