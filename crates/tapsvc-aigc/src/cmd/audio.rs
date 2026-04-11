use anyhow::{Context, bail};
use chrono::Local;
use tapsvc_aigc_openai::OpenAiClient;
use tapsvc_aigc_openai::audio::{SpeechRequest, VoiceSettings};

use crate::cli::AudioCommand;

pub async fn handle(command: AudioCommand) -> anyhow::Result<()> {
    match command {
        AudioCommand::Speech {
            model,
            voice,
            input,
            input_file,
            format,
            speed,
            stability,
            similarity,
            output,
        } => {
            let text = resolve_input(input.as_deref(), input_file.as_deref()).await?;

            let base_url =
                std::env::var("TAPSVC_BASE_URL").context("TAPSVC_BASE_URL is not set")?;
            let api_key = std::env::var("TAPSVC_API_KEY").context("TAPSVC_API_KEY is not set")?;

            let client = OpenAiClient::new(base_url, api_key);

            let voice_settings = if stability.is_some() || similarity.is_some() {
                Some(VoiceSettings {
                    stability,
                    similarity_boost: similarity,
                })
            } else {
                None
            };

            let req = SpeechRequest {
                model,
                input: text,
                voice,
                response_format: Some(format.clone()),
                speed: Some(speed),
                voice_settings,
            };

            let audio_bytes = client.speech(&req).await?;

            let output_path = match output {
                Some(p) => std::path::PathBuf::from(p),
                None => {
                    let ts = Local::now().format("%Y%m%d_%H%M%S");
                    std::path::PathBuf::from(format!("speech_{ts}.{format}"))
                }
            };

            if let Some(parent) = output_path.parent()
                && !parent.as_os_str().is_empty()
            {
                tokio::fs::create_dir_all(parent)
                    .await
                    .with_context(|| format!("failed to create directory {}", parent.display()))?;
            }

            tokio::fs::write(&output_path, &audio_bytes)
                .await
                .with_context(|| format!("failed to write {}", output_path.display()))?;

            println!("{}", output_path.display());

            Ok(())
        }
    }
}

async fn resolve_input(input: Option<&str>, input_file: Option<&str>) -> anyhow::Result<String> {
    match (input, input_file) {
        (Some(_), Some(_)) => bail!("--input and --input-file are mutually exclusive"),
        (Some(text), None) => Ok(text.to_string()),
        (None, Some(path)) => {
            let content = tokio::fs::read_to_string(path)
                .await
                .with_context(|| format!("failed to read input file: {path}"))?;
            Ok(content)
        }
        (None, None) => bail!("either --input or --input-file must be provided"),
    }
}
