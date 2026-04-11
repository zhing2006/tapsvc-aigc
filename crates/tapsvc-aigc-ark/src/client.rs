use std::time::Duration;

use reqwest::Client;
use tapsvc_aigc_core::{RetryConfig, retry};

use crate::error::Error;
use crate::video::{
    CreateVideoTaskRequest, ListVideoTasksFilter, VideoTask, VideoTaskId, VideoTaskList,
};

pub struct ArkClient {
    http: Client,
    base_url: String,
    api_key: String,
    retry_config: RetryConfig,
}

impl ArkClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.into(),
            api_key: api_key.into(),
            retry_config: RetryConfig::default(),
        }
    }

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub async fn create_video_task(
        &self,
        req: &CreateVideoTaskRequest,
    ) -> Result<VideoTaskId, Error> {
        let url = format!(
            "{}/volcengine/api/v3/contents/generations/tasks",
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
                let retry_after = parse_retry_after(&response);
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
            let task_id: VideoTaskId = serde_json::from_str(&text).map_err(Error::Deserialize)?;
            Ok(task_id)
        })
        .await
    }

    pub async fn get_video_task(&self, task_id: &str) -> Result<VideoTask, Error> {
        let url = format!(
            "{}/volcengine/api/v3/contents/generations/tasks/{}",
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

            let status = response.status();
            if !status.is_success() {
                let retry_after = parse_retry_after(&response);
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
            let task: VideoTask = serde_json::from_str(&text).map_err(Error::Deserialize)?;
            Ok(task)
        })
        .await
    }

    pub async fn list_video_tasks(
        &self,
        filter: &ListVideoTasksFilter,
    ) -> Result<VideoTaskList, Error> {
        let mut url = format!(
            "{}/volcengine/api/v3/contents/generations/tasks",
            self.base_url.trim_end_matches('/')
        );

        let mut params: Vec<String> = Vec::new();
        if let Some(page_num) = filter.page_num {
            params.push(format!("page_num={page_num}"));
        }
        if let Some(page_size) = filter.page_size {
            params.push(format!("page_size={page_size}"));
        }
        if let Some(ref status) = filter.status {
            params.push(format!("filter.status={}", encode_query_value(status)));
        }
        if let Some(ref model) = filter.model {
            params.push(format!("filter.model={}", encode_query_value(model)));
        }
        if let Some(ref task_ids) = filter.task_ids {
            for id in task_ids {
                params.push(format!("filter.task_ids={}", encode_query_value(id)));
            }
        }
        if !params.is_empty() {
            url = format!("{url}?{}", params.join("&"));
        }

        retry(&self.retry_config, || async {
            let response = self
                .http
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?;

            let status = response.status();
            if !status.is_success() {
                let retry_after = parse_retry_after(&response);
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
            let list: VideoTaskList = serde_json::from_str(&text).map_err(Error::Deserialize)?;
            Ok(list)
        })
        .await
    }

    pub async fn delete_video_task(&self, task_id: &str) -> Result<(), Error> {
        let url = format!(
            "{}/volcengine/api/v3/contents/generations/tasks/{}",
            self.base_url.trim_end_matches('/'),
            task_id
        );

        retry(&self.retry_config, || async {
            let response = self
                .http
                .delete(&url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?;

            let status = response.status();
            if !status.is_success() {
                let retry_after = parse_retry_after(&response);
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

            Ok(())
        })
        .await
    }
}

fn parse_retry_after(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
}

fn encode_query_value(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{byte:02X}"));
            }
        }
    }
    encoded
}
