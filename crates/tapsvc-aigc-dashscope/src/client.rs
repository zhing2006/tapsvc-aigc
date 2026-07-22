use std::time::Duration;

use reqwest::Client;
use tapsvc_aigc_core::{RetryConfig, retry};

use crate::error::Error;
use crate::video::{CreateVideoTaskRequest, VideoTaskEnvelope, VideoTaskResponse};

pub struct DashScopeClient {
    http: Client,
    base_url: String,
    api_key: String,
    retry_config: RetryConfig,
}

impl DashScopeClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
            retry_config: RetryConfig::default(),
        }
    }

    pub async fn create_video_task(
        &self,
        request: &CreateVideoTaskRequest,
    ) -> Result<VideoTaskResponse, Error> {
        let url = format!(
            "{}/dashscope/api/v1/services/aigc/video-generation/video-synthesis",
            self.base_url.trim_end_matches('/')
        );
        let body = serde_json::to_vec(request).map_err(Error::Deserialize)?;

        retry(&self.retry_config, || async {
            let response = self
                .http
                .post(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .header("X-DashScope-Async", "enable")
                .body(body.clone())
                .send()
                .await?;

            parse_response(response).await
        })
        .await
    }

    pub async fn get_video_task(&self, task_id: &str) -> Result<VideoTaskResponse, Error> {
        let url = format!(
            "{}/dashscope/api/v1/tasks/{}",
            self.base_url.trim_end_matches('/'),
            task_id
        );

        retry(&self.retry_config, || async {
            let response = self
                .http
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?;

            parse_response(response).await
        })
        .await
    }
}

async fn parse_response(response: reqwest::Response) -> Result<VideoTaskResponse, Error> {
    let status = response.status();
    if !status.is_success() {
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
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
    decode_response(&text)
}

fn decode_response(text: &str) -> Result<VideoTaskResponse, Error> {
    let envelope: VideoTaskEnvelope = serde_json::from_str(text).map_err(Error::Deserialize)?;

    if let Some(output) = envelope.output {
        return Ok(VideoTaskResponse {
            output,
            request_id: envelope.request_id,
        });
    }

    if let Some(code) = envelope.code {
        return Err(Error::Service {
            code,
            message: envelope
                .message
                .unwrap_or_else(|| "unknown service error".to_string()),
        });
    }

    Err(Error::InvalidResponse(format!(
        "missing output{}",
        envelope
            .request_id
            .map(|id| format!(" (request_id: {id})"))
            .unwrap_or_default()
    )))
}

#[cfg(test)]
mod tests {
    use super::decode_response;
    use crate::error::Error;

    #[test]
    fn decodes_task_response() {
        let response = decode_response(
            r#"{"output":{"task_id":"task-1","task_status":"PENDING"},"request_id":"req-1"}"#,
        )
        .expect("response should decode");

        assert_eq!(response.output.task_id.as_deref(), Some("task-1"));
        assert_eq!(response.request_id.as_deref(), Some("req-1"));
    }

    #[test]
    fn surfaces_top_level_service_error() {
        let error = decode_response(r#"{"code":"InvalidParameter","message":"bad ratio"}"#)
            .expect_err("service error should fail");

        assert!(matches!(
            error,
            Error::Service { ref code, ref message }
                if code == "InvalidParameter" && message == "bad ratio"
        ));
    }
}
