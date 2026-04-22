use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateImageRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

#[derive(Debug)]
pub struct EditImageRequest {
    pub model: String,
    pub prompt: String,
    pub image_bytes: Vec<u8>,
    pub image_filename: String,
    pub mask_bytes: Option<Vec<u8>>,
    pub mask_filename: Option<String>,
    pub n: Option<u32>,
    pub size: Option<String>,
    pub output_format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageResponse {
    pub created: u64,
    pub data: Vec<ImageData>,
}

#[derive(Debug, Deserialize)]
pub struct ImageData {
    pub b64_json: Option<String>,
    pub url: Option<String>,
    pub revised_prompt: Option<String>,
}
