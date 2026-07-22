use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct MediaItem {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateVideoTaskRequest {
    pub model: String,
    pub input: VideoInput,
    pub parameters: VideoParameters,
}

#[derive(Debug, Clone)]
pub struct VideoTaskResponse {
    pub output: VideoTask,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VideoTaskEnvelope {
    #[serde(default)]
    pub output: Option<VideoTask>,

    #[serde(default)]
    pub request_id: Option<String>,

    #[serde(default)]
    pub code: Option<String>,

    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoTask {
    #[serde(default)]
    pub task_id: Option<String>,
    pub task_status: String,

    #[serde(default)]
    pub video_url: Option<String>,

    #[serde(default)]
    pub orig_prompt: Option<String>,

    #[serde(default)]
    pub code: Option<String>,

    #[serde(default)]
    pub message: Option<String>,
}
