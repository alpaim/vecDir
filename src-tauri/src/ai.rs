use std::path::Path;

use anyhow::{bail, Context, Ok, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        chat::{
            ChatCompletionRequestMessage, ChatCompletionRequestMessageContentPartImage,
            ChatCompletionRequestMessageContentPartText, ChatCompletionRequestSystemMessage,
            ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
            CreateChatCompletionRequestArgs, ImageDetail, ImageUrl,
        },
        embeddings::{CreateEmbeddingRequestArgs, CreateEmbeddingResponse},
        responses::{CreateResponseArgs, InputParam, Response},
    },
    Client,
};
use base64::Engine;
use serde_json::json;

use crate::ai::{self};

pub mod embedding;
pub mod llm;

#[derive(Clone)]
pub struct AI {
    pub client: Client<OpenAIConfig>,
}

impl AI {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        let config = OpenAIConfig::new()
            .with_api_base(base_url)
            .with_api_key(api_key);

        let client = Client::with_config(config);

        Ok(AI { client: client })
    }

    pub async fn create_embedding(
        &self,
        input: String,
        model: String,
    ) -> Result<CreateEmbeddingResponse> {
        let args = CreateEmbeddingRequestArgs::default()
            .input(input)
            .model(model)
            .build()
            .context("failed to build embedding request args")?;

        let response: CreateEmbeddingResponse = self
            .client
            .embeddings()
            .create(args)
            .await
            .context("failed to generate embedding")?;

        Ok(response)
    }

    pub async fn create_embeddings_batch(
        &self,
        input: Vec<String>,
        model: String,
    ) -> Result<CreateEmbeddingResponse> {
        let args = CreateEmbeddingRequestArgs::default()
            .input(input)
            .model(model)
            .build()
            .context("failed to build embedding request args")?;

        let response = self
            .client
            .embeddings()
            .create(args)
            .await
            .context("failed to generate embeddings batch")?;

        Ok(response)
    }

    // the expected structure of qwen3vl embedding multimodal request in llamacpp
    // accroding to this PR: https://github.com/ggml-org/llama.cpp/pull/18665
    // and exact this commit: https://github.com/ggml-org/llama.cpp/pull/18665/commits/56a0d87bd022ab017484523beac26cc3a946c5a4

    // struct ai::embedding::multimodal_llamacpp::MultimodalEmbeddingInput is currently typed in this manner to prevent accidentally usage

    pub async fn create_embedding_qwen3vl_llamacpp(
        &self,
        input: ai::embedding::multimodal_llamacpp::MultimodalEmbeddingInput,
        model: String,
    ) -> Result<CreateEmbeddingResponse> {
        let mut content = Vec::new();

        if let Some(text) = input.text {
            content.push(json!({
                "type": "text",
                "text": text,
            }));
        }

        if let Some(url) = input.image_url {
            content.push(json!({
                "type": "image_url",
                "image_url": { "url": url },
            }));
        }

        let request = ai::embedding::multimodal_llamacpp::MultimodalEmbeddingRequest {
            model: model,
            input: json!(content),
            encoding_format: None,
        };

        let response: CreateEmbeddingResponse = self
            .client
            .embeddings()
            .create_byot(&request)
            .await
            .context("failed to generate multimodal embedding using qwen3vl and llamacpp")
            .unwrap();

        Ok(response)
    }

    pub async fn create_embeddings_batch_qwen3vl_llamacpp(
        &self,
        inputs: Vec<ai::embedding::multimodal_llamacpp::MultimodalEmbeddingInput>,
        model: String,
    ) -> Result<CreateEmbeddingResponse> {
        let content_array: Vec<serde_json::Value> = inputs
            .into_iter()
            .map(|input| {
                let mut content = Vec::new();

                if let Some(text) = input.text {
                    content.push(json!({
                        "type": "text",
                        "text": text,
                    }));
                }

                if let Some(url) = input.image_url {
                    content.push(json!({
                        "type": "image_url",
                        "image_url": { "url": url },
                    }));
                }

                json!(content)
            })
            .collect();

        let request = ai::embedding::multimodal_llamacpp::MultimodalEmbeddingRequest {
            model: model,
            input: json!(content_array),
            encoding_format: None,
        };

        let response: CreateEmbeddingResponse = self
            .client
            .embeddings()
            .create_byot(&request)
            .await
            .context("failed to generate batch of multimodal embedding using qwen3vl and llamacpp")
            .unwrap();

        Ok(response)
    }

    pub async fn request_llm(&self, input: InputParam, model: String) -> Result<Response> {
        let args = CreateResponseArgs::default()
            .input(input)
            .model(model)
            .build()
            .context("failed to build llm request args")?;

        let response = self
            .client
            .responses()
            .create(args)
            .await
            .context("failed to receive result of llm request")?;

        Ok(response)
    }

    pub async fn image_to_base64(&self, file_path: &str) -> Result<String> {
        let image_path = Path::new(file_path);

        let image_data = tokio::fs::read(image_path)
            .await
            .context("failed to read image to parse it to base64")?;

        let mime_type = match image_path.extension().and_then(|ext| ext.to_str()) {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("webp") => "image/webp",
            Some("gif") => "image/gif",
            _ => "image/jpeg",
        };

        let b64 = base64::engine::general_purpose::STANDARD.encode(&image_data);

        let data_url = format!("data:{};base64,{}", mime_type, b64);

        Ok(data_url)
    }

    pub async fn describe_image_from_file(
        &self,
        file_path: &str,
        system_prompt: &str,
        user_prompt: &str,
        model: &str,
    ) -> Result<String> {
        let image_url = self
            .image_to_base64(file_path)
            .await
            .context("failed to get image base64 url")?;

        let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();

        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()
            .context("failed to build system message in describe_image_from_file")?
            .into();

        messages.push(system_message);

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(vec![
                ChatCompletionRequestMessageContentPartText::from(user_prompt).into(),
                ChatCompletionRequestMessageContentPartImage::from(ImageUrl {
                    url: image_url,
                    detail: Some(ImageDetail::Auto),
                })
                .into(),
            ])
            .build()
            .context("failed to build user message content")?
            .into();

        messages.push(user_message);

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .build()
            .context("failed to build llm request for image description")?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .context("failed to receive result of llm image description")?;

        let llm_response_message = response
            .choices
            .last()
            .context("failed to get latest llm message")?
            .clone();

        let llm_response_text = llm_response_message
            .message
            .content
            .context("failed to get content of llm message")?;

        Ok(llm_response_text)
    }

    pub fn prepare_matroshka(&self, mut embedding: Vec<f32>, n: usize) -> Result<Vec<f32>> {
        if n == 0 {
            bail!("Requested dimension n must be greater than 0");
        }

        let original_len = embedding.len();
        if original_len < n {
            bail!(
                "Input embedding is too short: length is {}, but requested n is {}",
                original_len,
                n
            );
        }

        embedding.truncate(n);

        let sum_squares: f32 = embedding.iter().map(|&x| x * x).sum();
        let norm = sum_squares.sqrt();

        if norm < f32::EPSILON {
            bail!("The truncated vector has a zero norm (it consists only of zeros)");
        }

        // normalizing
        for val in embedding.iter_mut() {
            *val /= norm;
        }

        Ok(embedding)
    }
}
