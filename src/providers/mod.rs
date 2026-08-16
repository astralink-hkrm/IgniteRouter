pub mod cloud;
pub mod failover;
pub mod local;
pub mod plugins;
pub mod vault;

use crate::core::{RouterError, UnifiedRequest, UnifiedResponse};
use async_trait::async_trait;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn id(&self) -> &str;
    fn is_local(&self) -> bool;
    async fn is_healthy(&self) -> bool;
    async fn complete(&self, req: UnifiedRequest) -> Result<UnifiedResponse, RouterError>;
}

pub use cloud::CloudProvider;
pub use failover::FailoverEngine;
pub use local::LocalOllamaProvider;
pub use plugins::omniroute::OmniRoutePlugin;
pub use vault::ApiKeyVault;
