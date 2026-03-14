use anyhow::{bail, Context, Result};
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct VecboxClient {
    client: ReqwestClient,
    base_url: String,
    model: String,
}

#[derive(Serialize)]
struct VecboxEmbeddingRequest {
    model: Option<String>,
    input: serde_json::Value,
    #[serde(rename = "encoding_format")]
    encoding_format: String,
    instruction: Option<String>,
}

#[derive(Deserialize)]
struct VecboxEmbeddingResponse {
    object: String,
    data: Vec<VecboxEmbeddingData>,
    model: String,
    usage: VecboxUsage,
}

#[derive(Deserialize)]
struct VecboxEmbeddingData {
    object: String,
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Deserialize)]
struct VecboxUsage {
    #[serde(rename = "prompt_tokens")]
    prompt_tokens: usize,
    #[serde(rename = "total_tokens")]
    total_tokens: usize,
}

impl VecboxClient {
    pub fn new(base_url: &str, model: &str) -> Result<Self> {
        let client = ReqwestClient::new();
        let base_url = base_url.trim_end_matches('/').to_string();

        Ok(VecboxClient {
            client,
            base_url,
            model: model.to_string(),
        })
    }

    pub async fn create_text_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let request = VecboxEmbeddingRequest {
            model: Some(self.model.clone()),
            input: serde_json::json!(text),
            encoding_format: "float".to_string(),
            instruction: None,
        };

        let response = self
            .client
            .post(format!("{}/v1/embeddings", self.base_url))
            .json(&request)
            .send()
            .await
            .context("failed to send vecbox embedding request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            bail!("vecbox API error ({}): {}", status, body);
        }

        let result: VecboxEmbeddingResponse = response
            .json()
            .await
            .context("failed to parse vecbox embedding response")?;

        result
            .data
            .into_iter()
            .next()
            .map(|d| d.embedding)
            .context("no embedding in response")
    }

    pub async fn create_image_embedding(&self, image_data_url: &str) -> Result<Vec<f32>> {
        let request = VecboxEmbeddingRequest {
            model: Some(self.model.clone()),
            input: serde_json::json!([
                {
                    "type": "image_url",
                    "image_url": {
                        "url": image_data_url
                    }
                }
            ]),
            encoding_format: "float".to_string(),
            instruction: None,
        };

        let response = self
            .client
            .post(format!("{}/v1/embeddings", self.base_url))
            .json(&request)
            .send()
            .await
            .context("failed to send vecbox image embedding request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            bail!("vecbox API error ({}): {}", status, body);
        }

        let result: VecboxEmbeddingResponse = response
            .json()
            .await
            .context("failed to parse vecbox embedding response")?;

        result
            .data
            .into_iter()
            .next()
            .map(|d| d.embedding)
            .context("no embedding in response")
    }

    pub fn prepare_matroshka(&self, embedding: Vec<f32>, target_dim: usize) -> Result<Vec<f32>> {
        if target_dim == 0 {
            bail!("Target dimension must be greater than 0");
        }

        let original_len = embedding.len();
        if original_len < target_dim {
            bail!(
                "Input embedding too short: length is {}, target is {}",
                original_len,
                target_dim
            );
        }

        let mut truncated = embedding;
        truncated.truncate(target_dim);

        let sum_squares: f32 = truncated.iter().map(|&x| x * x).sum();
        let norm = sum_squares.sqrt();

        if norm < f32::EPSILON {
            bail!("Vector has zero norm");
        }

        for val in truncated.iter_mut() {
            *val /= norm;
        }

        Ok(truncated)
    }
}
