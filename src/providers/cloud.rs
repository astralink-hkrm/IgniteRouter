use crate::core::{RouterError, UnifiedRequest, UnifiedResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::time::Instant;

pub struct CloudProvider {
    pub id_name: String,
    pub api_base: String,
    pub api_key: String,
    pub client: Client,
}

impl CloudProvider {
    pub fn new(id_name: &str, api_base: &str, api_key: &str) -> Self {
        Self {
            id_name: id_name.to_string(),
            api_base: api_base.to_string(),
            api_key: api_key.to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }
}

#[async_trait]
impl super::ModelProvider for CloudProvider {
    fn id(&self) -> &str {
        &self.id_name
    }

    fn is_local(&self) -> bool {
        false
    }

    async fn is_healthy(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn complete(&self, req: UnifiedRequest) -> Result<UnifiedResponse, RouterError> {
        let start = Instant::now();

        let payload = json!({
            "model": req.model,
            "messages": req.messages,
            "temperature": req.temperature,
            "max_tokens": req.max_tokens,
            "stream": false
        });

        let mut request_builder = self
            .client
            .post(format!("{}/chat/completions", self.api_base))
            .json(&payload);

        if !self.api_key.is_empty() {
            request_builder = request_builder.bearer_auth(&self.api_key);
        }

        let res = request_builder
            .send()
            .await
            .map_err(|e| RouterError::ProviderUnavailable(e.to_string()))?;

        let status = res.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(RouterError::RateLimited("HTTP 429 Rate limit exceeded".to_string()));
        }

        if !status.is_success() {
            return Err(RouterError::ProviderUnavailable(format!(
                "Cloud API HTTP error {}",
                status
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RouterError::Internal(e.to_string()))?;

        let content = body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let prompt_tokens = body["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32;
        let completion_tokens = body["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32;

        Ok(UnifiedResponse {
            id: body["id"].as_str().unwrap_or("cloud-res").to_string(),
            model: req.model,
            content,
            prompt_tokens,
            completion_tokens,
            tier: "AUTO".to_string(),
            execution_backend: self.id_name.clone(),
            latency_ms: start.elapsed().as_millis(),
        })
    }
}
