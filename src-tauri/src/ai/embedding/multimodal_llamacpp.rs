use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MultimodalEmbeddingRequest {
    pub model: String,
    pub input: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
}

pub struct MultimodalEmbeddingInput {
    pub text: Option<String>,
    pub image_url: Option<String>,
}