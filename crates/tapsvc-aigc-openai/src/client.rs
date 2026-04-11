use std::time::Duration;

use reqwest::Client;
use tapsvc_aigc_core::{RetryConfig, retry};

use crate::error::Error;
use crate::image::{CreateImageRequest, ImageResponse};

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
