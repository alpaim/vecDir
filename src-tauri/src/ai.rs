use anyhow::{Context, Ok, Result};
use async_openai::{
    config::OpenAIConfig,
    types::{
        embeddings::{CreateEmbeddingRequestArgs, CreateEmbeddingResponse},
        responses::{CreateResponseArgs, Response},
    },
    Client,
};

pub mod embedding;
pub mod llm;

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

    pub async fn request_llm(&self, input: String, model: String) -> Result<Response> {
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
}
