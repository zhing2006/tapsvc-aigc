use std::path::Path;
use std::time::Duration;

use reqwest::Client;
use tapsvc_aigc_core::{RetryConfig, retry};

use crate::audio::SpeechRequest;
use crate::error::Error;
use crate::image::{CreateImageRequest, EditImageRequest, ImageResponse};

pub struct OpenAiClient {
    http: Client,
    base_url: String,
    api_key: String,
    retry_config: RetryConfig,
}

impl OpenAiClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
            retry_config: RetryConfig::default(),
        }
    }

    pub async fn edit_image(&self, req: &EditImageRequest) -> Result<ImageResponse, Error> {
        let url = format!("{}/v1/images/edits", self.base_url.trim_end_matches('/'));

        let model = req.model.clone();
        let prompt = req.prompt.clone();
        let image_bytes = req.image_bytes.clone();
        let image_filename = req.image_filename.clone();
        let mask_bytes = req.mask_bytes.clone();
        let mask_filename = req.mask_filename.clone();
        let n = req.n;
        let size = req.size.clone();
        let output_format = req.output_format.clone();

        retry(&self.retry_config, || {
            let url = url.clone();
            let model = model.clone();
            let prompt = prompt.clone();
            let image_bytes = image_bytes.clone();
            let image_filename = image_filename.clone();
            let mask_bytes = mask_bytes.clone();
            let mask_filename = mask_filename.clone();
            let size = size.clone();
            let output_format = output_format.clone();

            async move {
                let mime = mime_from_filename(&image_filename);
                let image_part = reqwest::multipart::Part::bytes(image_bytes)
                    .file_name(image_filename)
                    .mime_str(&mime)
                    .map_err(Error::Request)?;

                let mut form = reqwest::multipart::Form::new()
                    .text("model", model)
                    .text("prompt", prompt)
                    .part("image", image_part);

                if let Some(bytes) = mask_bytes {
                    let fname = mask_filename.unwrap_or_else(|| "mask.png".to_string());
                    let mask_part = reqwest::multipart::Part::bytes(bytes)
                        .file_name(fname)
                        .mime_str("image/png")
                        .map_err(Error::Request)?;
                    form = form.part("mask", mask_part);
                }

                if let Some(n) = n {
                    form = form.text("n", n.to_string());
                }
                if let Some(size) = size {
                    form = form.text("size", size);
                }
                if let Some(fmt) = output_format {
                    form = form.text("output_format", fmt);
                }

                let response = self
                    .http
                    .post(&url)
                    .header("Authorization", format!("Bearer {}", self.api_key))
                    .multipart(form)
                    .send()
                    .await?;

                let status = response.status();
                if !status.is_success() {
                    let retry_after = response
                        .headers()
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                        .map(Duration::from_secs);

                    let message = response
                        .text()
                        .await
                        .unwrap_or_else(|_| String::from("unknown error"));

                    return Err(Error::Api {
                        status: status.as_u16(),
                        message,
                        retry_after,
                    });
                }

                let text = response.text().await?;
                let image_response: ImageResponse =
                    serde_json::from_str(&text).map_err(Error::Deserialize)?;
                Ok(image_response)
            }
        })
        .await
    }

    pub async fn speech(&self, req: &SpeechRequest) -> Result<Vec<u8>, Error> {
        let url = format!("{}/v1/audio/speech", self.base_url.trim_end_matches('/'));
        let body = serde_json::to_vec(req).map_err(Error::Deserialize)?;

        retry(&self.retry_config, || async {
            let response = self
                .http
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .body(body.clone())
                .send()
                .await?;

            let status = response.status();
            if !status.is_success() {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(Duration::from_secs);

                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| String::from("unknown error"));

                return Err(Error::Api {
                    status: status.as_u16(),
                    message,
                    retry_after,
                });
            }

            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        })
        .await
    }

    pub async fn create_image(&self, req: &CreateImageRequest) -> Result<ImageResponse, Error> {
        let url = format!(
            "{}/v1/images/generations",
            self.base_url.trim_end_matches('/')
        );
        let body = serde_json::to_vec(req).map_err(Error::Deserialize)?;

        retry(&self.retry_config, || async {
            let response = self
                .http
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .body(body.clone())
                .send()
                .await?;

            let status = response.status();
            if !status.is_success() {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(Duration::from_secs);

                let message = response
                    .text()
                    .await
                    .unwrap_or_else(|_| String::from("unknown error"));

                return Err(Error::Api {
                    status: status.as_u16(),
                    message,
                    retry_after,
                });
            }

            let text = response.text().await?;
            let image_response: ImageResponse =
                serde_json::from_str(&text).map_err(Error::Deserialize)?;
            Ok(image_response)
        })
        .await
    }
}

fn mime_from_filename(filename: &str) -> String {
    match Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => "image/png".to_string(),
        Some("jpg" | "jpeg") => "image/jpeg".to_string(),
        Some("webp") => "image/webp".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}
