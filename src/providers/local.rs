use crate::core::{RouterError, UnifiedRequest, UnifiedResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::time::Instant;

pub struct LocalOllamaProvider {
    pub base_url: String,
    pub client: Client,
}

impl LocalOllamaProvider {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            client: Client::builder().timeout(std::time::Duration::from_secs(60)).build().unwrap_or_default(),
        }
    }
}

#[async_trait]
impl super::ModelProvider for LocalOllamaProvider {
    fn id(&self) -> &str {
        "local_ollama"
    }

    fn is_local(&self) -> bool {
        true
    }

    async fn is_healthy(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    async fn complete(&self, req: UnifiedRequest) -> Result<UnifiedResponse, RouterError> {
        let start = Instant::now();
        let payload = json!({
            "model": if req.model == "blockrun/auto" { "llama3" } else { &req.model },
            "messages": req.messages,
            "stream": false
        });

        let res = self
            .client
            .post(format!("{}/api/chat", self.base_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| RouterError::ProviderUnavailable(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RouterError::ProviderUnavailable(format!(
                "Ollama HTTP error {}",
                res.status()
            )));
        }

        let body: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RouterError::Internal(e.to_string()))?;

        let content = body["message"]["content"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let prompt_tokens = body["prompt_eval_count"].as_u64().unwrap_or(0) as u32;
        let completion_tokens = body["eval_count"].as_u64().unwrap_or(0) as u32;

        Ok(UnifiedResponse {
            id: format!("ollama-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
            model: req.model,
            content,
            prompt_tokens,
            completion_tokens,
            tier: "SIMPLE".to_string(),
            execution_backend: "LOCAL_GPU_OLLAMA".to_string(),
            latency_ms: start.elapsed().as_millis(),
        })
    }
}
