use anyhow::{bail, Context, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};

const DEFAULT_MEDIA_MARKER: &str = "<__media__>";
const TARGET_DIMENSION: usize = 768;

#[derive(Clone)]
pub struct LlamaCppClient {
    client: reqwest::Client,
    base_url: String,
    model: String,
    media_marker: String,
}

#[derive(Serialize)]
struct LlamaCppEmbeddingRequest {
    model: String,
    input: Vec<LlamaCppInput>,
}

#[derive(Serialize)]
struct LlamaCppInput {
    prompt_string: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    multimodal_data: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct LlamaCppEmbeddingResponse {
    data: Vec<LlamaCppEmbeddingData>,
}

#[derive(Deserialize)]
struct LlamaCppEmbeddingData {
    embedding: Vec<f32>,
    index: usize,
}

#[derive(Deserialize)]
struct LlamaCppPropsResponse {
    media_marker: String,
}

impl LlamaCppClient {
    pub fn new(base_url: &str, model: &str, media_marker: Option<String>) -> Self {
        let base_url = base_url.trim_end_matches('/').to_string();
        LlamaCppClient {
            client: reqwest::Client::new(),
            base_url,
            model: model.to_string(),
            media_marker: media_marker.unwrap_or_else(|| DEFAULT_MEDIA_MARKER.to_string()),
        }
    }

    pub async fn fetch_media_marker(base_url: &str) -> Result<String> {
        let base_url = base_url.trim_end_matches('/');
        let url = format!("{}/props", base_url);

        let response = reqwest::Client::new()
            .get(&url)
            .send()
            .await
            .context("failed to fetch /props from llama.cpp server")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            bail!("llama.cpp /props error ({}): {}", status, body);
        }

        let props: LlamaCppPropsResponse = response
            .json()
            .await
            .context("failed to parse /props response")?;

        Ok(props.media_marker)
    }

    pub fn media_marker(&self) -> &str {
        &self.media_marker
    }

    pub async fn create_text_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let request = LlamaCppEmbeddingRequest {
            model: self.model.clone(),
            input: vec![LlamaCppInput {
                prompt_string: text.to_string(),
                multimodal_data: None,
            }],
        };

        let embedding = self.send_request(request).await?;

        self.prepare_matroshka(embedding, TARGET_DIMENSION)
    }

    pub async fn create_image_embedding(
        &self,
        image_b64: &str,
        user_prompt: &str,
    ) -> Result<Vec<f32>> {
        let prompt_string = format!("{} {}", user_prompt, self.media_marker);

        let request = LlamaCppEmbeddingRequest {
            model: self.model.clone(),
            input: vec![LlamaCppInput {
                prompt_string,
                multimodal_data: Some(vec![image_b64.to_string()]),
            }],
        };

        let embedding = self.send_request(request).await?;

        self.prepare_matroshka(embedding, TARGET_DIMENSION)
    }

    pub async fn create_text_embeddings_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let input: Vec<LlamaCppInput> = texts
            .iter()
            .map(|text| LlamaCppInput {
                prompt_string: text.clone(),
                multimodal_data: None,
            })
            .collect();

        let request = LlamaCppEmbeddingRequest {
            model: self.model.clone(),
            input,
        };

        let raw_embeddings = self.send_request_batch(request).await?;

        let mut results = Vec::with_capacity(raw_embeddings.len());
        for embedding in raw_embeddings {
            let truncated = self
                .prepare_matroshka(embedding, TARGET_DIMENSION)
                .context("failed to prepare matroshka for batch embedding")?;
            results.push(truncated);
        }

        Ok(results)
    }

    pub async fn image_to_base64(file_path: &str) -> Result<String> {
        let image_path = std::path::Path::new(file_path);
        let image_data = tokio::fs::read(image_path)
            .await
            .context("failed to read image file for base64 encoding")?;

        let b64 = base64::engine::general_purpose::STANDARD.encode(&image_data);

        Ok(b64)
    }

    pub fn prepare_matroshka(
        &self,
        mut embedding: Vec<f32>,
        target_dim: usize,
    ) -> Result<Vec<f32>> {
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

        embedding.truncate(target_dim);

        let sum_squares: f32 = embedding.iter().map(|&x| x * x).sum();
        let norm = sum_squares.sqrt();

        if norm < f32::EPSILON {
            bail!("Vector has zero norm");
        }

        for val in embedding.iter_mut() {
            *val /= norm;
        }

        Ok(embedding)
    }

    async fn send_request(&self, request: LlamaCppEmbeddingRequest) -> Result<Vec<f32>> {
        let url = format!("{}/v1/embeddings", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("failed to send llama.cpp embedding request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            bail!("llama.cpp API error ({}): {}", status, body);
        }

        let result: LlamaCppEmbeddingResponse = response
            .json()
            .await
            .context("failed to parse llama.cpp embedding response")?;

        result
            .data
            .into_iter()
            .next()
            .map(|d| d.embedding)
            .context("no embedding in response")
    }

    async fn send_request_batch(&self, request: LlamaCppEmbeddingRequest) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/v1/embeddings", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("failed to send llama.cpp batch embedding request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            bail!("llama.cpp API error ({}): {}", status, body);
        }

        let result: LlamaCppEmbeddingResponse = response
            .json()
            .await
            .context("failed to parse llama.cpp embedding response")?;

        let mut embeddings: Vec<(usize, Vec<f32>)> = result
            .data
            .into_iter()
            .map(|d| (d.index, d.embedding))
            .collect();

        embeddings.sort_by_key(|(index, _)| *index);

        Ok(embeddings.into_iter().map(|(_, emb)| emb).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_json, header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn make_dummy_embedding(dim: usize) -> Vec<f32> {
        (0..dim).map(|i| (i as f32) * 0.001).collect()
    }

    #[test]
    fn test_text_request_serialization() {
        let request = LlamaCppEmbeddingRequest {
            model: "test-model".to_string(),
            input: vec![LlamaCppInput {
                prompt_string: "hello world".to_string(),
                multimodal_data: None,
            }],
        };

        let json_val = serde_json::to_value(&request).unwrap();

        assert_eq!(json_val["model"], "test-model");
        assert_eq!(json_val["input"][0]["prompt_string"], "hello world");
        assert!(json_val["input"][0].get("multimodal_data").is_none());
    }

    #[test]
    fn test_image_request_serialization() {
        let request = LlamaCppEmbeddingRequest {
            model: "test-model".to_string(),
            input: vec![LlamaCppInput {
                prompt_string: "Describe this image. <__media__>".to_string(),
                multimodal_data: Some(vec!["base64data".to_string()]),
            }],
        };

        let json_val = serde_json::to_value(&request).unwrap();

        assert_eq!(json_val["model"], "test-model");
        assert_eq!(
            json_val["input"][0]["prompt_string"],
            "Describe this image. <__media__>"
        );
        assert_eq!(json_val["input"][0]["multimodal_data"][0], "base64data");
    }

    #[test]
    fn test_batch_request_serialization() {
        let request = LlamaCppEmbeddingRequest {
            model: "test-model".to_string(),
            input: vec![
                LlamaCppInput {
                    prompt_string: "first chunk".to_string(),
                    multimodal_data: None,
                },
                LlamaCppInput {
                    prompt_string: "second chunk".to_string(),
                    multimodal_data: None,
                },
            ],
        };

        let json_val = serde_json::to_value(&request).unwrap();

        assert_eq!(json_val["input"].as_array().unwrap().len(), 2);
        assert_eq!(json_val["input"][0]["prompt_string"], "first chunk");
        assert_eq!(json_val["input"][1]["prompt_string"], "second chunk");
    }

    #[test]
    fn test_response_deserialization() {
        let response_json = json!({
            "model": "test-model",
            "object": "list",
            "usage": {
                "prompt_tokens": 3,
                "total_tokens": 3
            },
            "data": [
                {
                    "object": "embedding",
                    "index": 0,
                    "embedding": [0.1, 0.2, 0.3, 0.4]
                }
            ]
        });

        let response: LlamaCppEmbeddingResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].index, 0);
        assert_eq!(response.data[0].embedding, vec![0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn test_props_response_deserialization() {
        let props_json = json!({
            "media_marker": "<__media_abc123__>",
            "model_alias": "test.gguf"
        });

        let props: LlamaCppPropsResponse = serde_json::from_value(props_json).unwrap();

        assert_eq!(props.media_marker, "<__media_abc123__>");
    }

    #[test]
    fn test_client_uses_default_marker() {
        let client = LlamaCppClient::new("http://localhost:8080", "model", None);
        assert_eq!(client.media_marker(), "<__media__>");
    }

    #[test]
    fn test_client_uses_custom_marker() {
        let client = LlamaCppClient::new(
            "http://localhost:8080",
            "model",
            Some("<__media_custom__>".to_string()),
        );
        assert_eq!(client.media_marker(), "<__media_custom__>");
    }

    #[test]
    fn test_client_trims_trailing_slash() {
        let client = LlamaCppClient::new("http://localhost:8080/", "model", None);
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_prepare_matroshka_truncates() {
        let client = LlamaCppClient::new("http://localhost:8080", "model", None);
        let embedding = make_dummy_embedding(2048);

        let result = client.prepare_matroshka(embedding, 768).unwrap();

        assert_eq!(result.len(), 768);
    }

    #[test]
    fn test_prepare_matroshka_normalizes() {
        let client = LlamaCppClient::new("http://localhost:8080", "model", None);
        let embedding = vec![3.0, 4.0];

        let result = client.prepare_matroshka(embedding, 2).unwrap();

        let norm: f32 = result.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
        assert!((result[0] - 0.6).abs() < 1e-5);
        assert!((result[1] - 0.8).abs() < 1e-5);
    }

    #[test]
    fn test_prepare_matroshka_rejects_zero_norm() {
        let client = LlamaCppClient::new("http://localhost:8080", "model", None);
        let embedding = vec![0.0, 0.0, 0.0, 0.0];

        let result = client.prepare_matroshka(embedding, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_matroshka_rejects_too_short() {
        let client = LlamaCppClient::new("http://localhost:8080", "model", None);
        let embedding = vec![1.0, 2.0];

        let result = client.prepare_matroshka(embedding, 768);
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_matroshka_rejects_zero_dim() {
        let client = LlamaCppClient::new("http://localhost:8080", "model", None);
        let embedding = vec![1.0, 2.0];

        let result = client.prepare_matroshka(embedding, 0);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_text_embedding_http() {
        let mock_server = MockServer::start().await;

        let dummy_embedding = make_dummy_embedding(2048);
        let response_body = json!({
            "model": "test-model",
            "object": "list",
            "usage": {"prompt_tokens": 3, "total_tokens": 3},
            "data": [{
                "object": "embedding",
                "index": 0,
                "embedding": dummy_embedding
            }]
        });

        let expected_request = json!({
            "model": "test-model",
            "input": [{
                "prompt_string": "hello world"
            }]
        });

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .and(body_json(expected_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(&mock_server.uri(), "test-model", None);
        let result = client.create_text_embedding("hello world").await.unwrap();

        assert_eq!(result.len(), 768);
    }

    #[tokio::test]
    async fn test_create_image_embedding_http() {
        let mock_server = MockServer::start().await;

        let dummy_embedding = make_dummy_embedding(2048);
        let response_body = json!({
            "model": "test-model",
            "object": "list",
            "usage": {"prompt_tokens": 17, "total_tokens": 17},
            "data": [{
                "object": "embedding",
                "index": 0,
                "embedding": dummy_embedding
            }]
        });

        let expected_request = json!({
            "model": "test-model",
            "input": [{
                "prompt_string": "Describe this image. <__media__>",
                "multimodal_data": ["base64data"]
            }]
        });

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .and(body_json(expected_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(&mock_server.uri(), "test-model", None);
        let result = client
            .create_image_embedding("base64data", "Describe this image.")
            .await
            .unwrap();

        assert_eq!(result.len(), 768);
    }

    #[tokio::test]
    async fn test_create_image_embedding_custom_marker() {
        let mock_server = MockServer::start().await;

        let dummy_embedding = make_dummy_embedding(2048);
        let response_body = json!({
            "model": "test-model",
            "object": "list",
            "usage": {"prompt_tokens": 17, "total_tokens": 17},
            "data": [{
                "object": "embedding",
                "index": 0,
                "embedding": dummy_embedding
            }]
        });

        let expected_request = json!({
            "model": "test-model",
            "input": [{
                "prompt_string": "Describe this image. <__media_custom__>",
                "multimodal_data": ["base64data"]
            }]
        });

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .and(body_json(expected_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(
            &mock_server.uri(),
            "test-model",
            Some("<__media_custom__>".to_string()),
        );
        let result = client
            .create_image_embedding("base64data", "Describe this image.")
            .await
            .unwrap();

        assert_eq!(result.len(), 768);
    }

    #[tokio::test]
    async fn test_fetch_media_marker_http() {
        let mock_server = MockServer::start().await;

        let props_body = json!({
            "media_marker": "<__media_test123__>",
            "model_alias": "test.gguf",
            "modalities": {"vision": true}
        });

        Mock::given(method("GET"))
            .and(path("/props"))
            .respond_with(ResponseTemplate::new(200).set_body_json(props_body))
            .mount(&mock_server)
            .await;

        let marker = LlamaCppClient::fetch_media_marker(&mock_server.uri())
            .await
            .unwrap();

        assert_eq!(marker, "<__media_test123__>");
    }

    #[tokio::test]
    async fn test_fetch_media_marker_http_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/props"))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&mock_server)
            .await;

        let result = LlamaCppClient::fetch_media_marker(&mock_server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_text_embedding_api_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Failed to tokenize prompt"))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(&mock_server.uri(), "test-model", None);
        let result = client.create_text_embedding("hello").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_text_embedding_empty_response() {
        let mock_server = MockServer::start().await;

        let response_body = json!({
            "model": "test-model",
            "object": "list",
            "usage": {"prompt_tokens": 0, "total_tokens": 0},
            "data": []
        });

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(&mock_server.uri(), "test-model", None);
        let result = client.create_text_embedding("hello").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_text_embeddings_batch_http() {
        let mock_server = MockServer::start().await;

        let emb1 = make_dummy_embedding(2048);
        let emb2 = make_dummy_embedding(2048);
        let response_body = json!({
            "model": "test-model",
            "object": "list",
            "usage": {"prompt_tokens": 6, "total_tokens": 6},
            "data": [
                {"object": "embedding", "index": 0, "embedding": emb1},
                {"object": "embedding", "index": 1, "embedding": emb2}
            ]
        });

        let expected_request = json!({
            "model": "test-model",
            "input": [
                {"prompt_string": "first"},
                {"prompt_string": "second"}
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .and(body_json(expected_request))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(&mock_server.uri(), "test-model", None);
        let result = client
            .create_text_embeddings_batch(&["first".to_string(), "second".to_string()])
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 768);
        assert_eq!(result[1].len(), 768);
    }

    #[tokio::test]
    async fn test_request_has_content_type_json() {
        let mock_server = MockServer::start().await;

        let dummy_embedding = make_dummy_embedding(2048);
        let response_body = json!({
            "model": "test-model",
            "object": "list",
            "usage": {"prompt_tokens": 3, "total_tokens": 3},
            "data": [{
                "object": "embedding",
                "index": 0,
                "embedding": dummy_embedding
            }]
        });

        Mock::given(method("POST"))
            .and(path("/v1/embeddings"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = LlamaCppClient::new(&mock_server.uri(), "test-model", None);
        let result = client.create_text_embedding("hello").await;
        assert!(result.is_ok());
    }
}
