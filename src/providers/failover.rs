use super::ModelProvider;
use crate::core::{RouterError, UnifiedRequest, UnifiedResponse};
use tracing::{info, warn};

pub struct FailoverEngine {
    pub providers: Vec<Box<dyn ModelProvider>>,
}

impl FailoverEngine {
    pub fn new(providers: Vec<Box<dyn ModelProvider>>) -> Self {
        Self { providers }
    }

    pub async fn execute_with_failover(
        &self,
        req: UnifiedRequest,
    ) -> Result<UnifiedResponse, RouterError> {
        let mut last_error = RouterError::ProviderUnavailable("No providers available".to_string());

        for provider in &self.providers {
            if !provider.is_healthy().await {
                continue;
            }

            info!("[FailoverEngine] Attempting execution on provider: {}", provider.id());

            match provider.complete(req.clone()).await {
                Ok(res) => {
                    info!("[FailoverEngine] Execution succeeded on provider: {}", provider.id());
                    return Ok(res);
                }
                Err(err) => {
                    warn!(
                        "[FailoverEngine] Provider {} failed: {}. Retrying next candidate...",
                        provider.id(),
                        err
                    );
                    last_error = err;
                }
            }
        }

        Err(last_error)
    }
}
