use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use chrono::Local;
use tapsvc_aigc_openai::OpenAiClient;
use tapsvc_aigc_openai::image::CreateImageRequest;

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
                size: Some(size),
                quality: Some(quality),
                response_format: Some(response_format.clone()),
                background: Some(background),
            };

            let response = client.create_image(&req).await?;

            let ext = format_to_ext(&response_format);
            let output_paths = build_output_paths(output.as_deref(), n, &ext);

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

                match &item.b64_json {
                    Some(data) => {
                        let bytes = BASE64.decode(data).with_context(|| {
                            format!("failed to decode base64 for image {}", i + 1)
                        })?;

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
                    None => {
                        eprintln!("warning: image {} has no data, skipping", i + 1);
                    }
                }
            }

            if written == 0 {
                bail!("API returned no valid image data");
            }

            Ok(())
        }
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

fn format_to_ext(format: &str) -> String {
    match format {
        "jpeg" => "jpg".to_string(),
        other => other.to_string(),
    }
}

fn build_output_paths(output: Option<&str>, n: u32, ext: &str) -> Vec<PathBuf> {
    match output {
        Some(path) if n == 1 => vec![PathBuf::from(path)],
        Some(path) => {
            let p = Path::new(path);
            let stem = p
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "image".to_string());
            let parent = p.parent().unwrap_or(Path::new(""));
            (1..=n)
                .map(|i| parent.join(format!("{stem}_{i}.{ext}")))
                .collect()
        }
        None => {
            let ts = Local::now().format("%Y%m%d_%H%M%S");
            if n == 1 {
                vec![PathBuf::from(format!("image_{ts}.{ext}"))]
            } else {
                (1..=n)
                    .map(|i| PathBuf::from(format!("image_{ts}_{i}.{ext}")))
                    .collect()
            }
        }
    }
}
