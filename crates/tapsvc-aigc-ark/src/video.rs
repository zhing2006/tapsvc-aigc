use serde::{Deserialize, Serialize};

// ── Content item types (for create request) ──

#[derive(Debug, Clone, Serialize)]
pub struct ImageUrlData {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoUrlData {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioUrlData {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[serde(rename = "text")]
    Text { text: String },

    #[serde(rename = "image_url")]
    ImageUrl {
        image_url: ImageUrlData,
        role: String,
    },

    #[serde(rename = "video_url")]
    VideoUrl {
        video_url: VideoUrlData,
        role: String,
    },

    #[serde(rename = "audio_url")]
    AudioUrl {
        audio_url: AudioUrlData,
        role: String,
    },
}

// ── Tool type ──

#[derive(Debug, Clone, Serialize)]
pub struct VideoTaskTool {
    #[serde(rename = "type")]
    pub type_: String,
}

// ── Create request ──

#[derive(Debug, Clone, Serialize)]
pub struct CreateVideoTaskRequest {
    pub model: String,
    pub content: Vec<ContentItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_audio: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_fixed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<VideoTaskTool>>,
}

// ── Response types ──

#[derive(Debug, Clone, Deserialize)]
pub struct VideoTaskId {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoContent {
    pub video_url: Option<String>,
    pub last_frame_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoTaskError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoTask {
    pub id: String,
    pub model: String,
    pub status: String,

    #[serde(default)]
    pub content: Option<VideoContent>,

    #[serde(default)]
    pub error: Option<VideoTaskError>,

    #[serde(default)]
    pub created_at: Option<u64>,

    #[serde(default)]
    pub updated_at: Option<u64>,

    #[serde(default)]
    pub duration: Option<i32>,

    #[serde(default)]
    pub ratio: Option<String>,

    #[serde(default)]
    pub resolution: Option<String>,

    #[serde(default)]
    pub seed: Option<u64>,

    #[serde(default)]
    pub revised_prompt: Option<String>,

    #[serde(default)]
    pub generate_audio: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoTaskList {
    pub total: u32,
    pub items: Vec<VideoTask>,
}

// ── List filter ──

#[derive(Debug, Clone, Default)]
pub struct ListVideoTasksFilter {
    pub page_num: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
    pub model: Option<String>,
    pub task_ids: Option<Vec<String>>,
}
