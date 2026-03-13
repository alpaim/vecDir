use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmbeddingContentPart {
    Text { text: String },
    ImageUrl { image_url: EmbeddingImageUrl },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingImageUrl {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct VecBoxEmbeddingRequest {
    pub model: Option<String>,
    pub input: Vec<EmbeddingContentPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VecBoxEmbeddingInput {
    pub text: Option<String>,
    pub instruction: Option<String>,
    pub image_url: Option<String>,
}

impl VecBoxEmbeddingInput {
    pub fn to_content_parts(&self) -> Vec<EmbeddingContentPart> {
        let mut parts = Vec::new();

        if let Some(ref text) = self.text {
            parts.push(EmbeddingContentPart::Text { text: text.clone() });
        }

        if let Some(ref url) = self.image_url {
            parts.push(EmbeddingContentPart::ImageUrl {
                image_url: EmbeddingImageUrl { url: url.clone() },
            });
        }

        parts
    }
}
