use crate::core::{RouterError, UnifiedRequest, UnifiedResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::time::Instant;

pub struct OmniRoutePlugin {
    pub enabled: bool,
    pub base_url: String,
    pub client: Client,
}

impl OmniRoutePlugin {
    pub fn new(enabled: bool, base_url: Option<String>) -> Self {
        Self {
            enabled,
            base_url: base_url.unwrap_or_else(|| "http://localhost:3000/v1".to_string()),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }
}

use crate::providers::ModelProvider;

#[async_trait]
impl ModelProvider for OmniRoutePlugin {
    fn id(&self) -> &str {
        "omniroute_plugin"
    }

    fn is_local(&self) -> bool {
        false
    }

    async fn is_healthy(&self) -> bool {
        if !self.enabled {
            return false;
        }

        self.client
            .get(format!("{}/models", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    async fn complete(&self, req: UnifiedRequest) -> Result<UnifiedResponse, RouterError> {
        if !self.enabled {
            return Err(RouterError::ProviderUnavailable(
                "OmniRoute plugin disabled".to_string(),
            ));
        }

        let start = Instant::now();
        let payload = json!({
            "model": req.model,
            "messages": req.messages,
            "stream": false
        });

        let res = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .json(&payload)
            .send()
            .await
            .map_err(|e| RouterError::ProviderUnavailable(e.to_string()))?;

        if !res.status().is_success() {
            return Err(RouterError::ProviderUnavailable(format!(
                "OmniRoute plugin HTTP error {}",
                res.status()
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

        Ok(UnifiedResponse {
            id: body["id"].as_str().unwrap_or("omniroute-res").to_string(),
            model: req.model,
            content,
            prompt_tokens: 0,
            completion_tokens: 0,
            tier: "FREE_TIER_POOL".to_string(),
            execution_backend: "OMNIROUTE_FREE_TIERS".to_string(),
            latency_ms: start.elapsed().as_millis(),
        })
    }
}
