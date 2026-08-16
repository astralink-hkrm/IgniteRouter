pub mod routes;

use crate::cache::LruCache;
use crate::classifier::LocalHardwareMonitor;
use crate::config::IgniteConfig;
use crate::guardrails::GuardrailEngine;
use crate::providers::ApiKeyVault;
use axum::{
    routing::{get, post},
    Router,
};
use routes::{handle_anthropic_messages, handle_list_models, handle_ollama_chat, handle_openai_chat, AppState};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub async fn run_server(config: IgniteConfig) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState {
        cache: LruCache::new(),
        guardrails: Arc::new(GuardrailEngine::new()),
        hardware: LocalHardwareMonitor::new(),
        vault: ApiKeyVault::new(),
        omniroute_enabled: config.omniroute_plugin.enabled,
        omniroute_url: config.omniroute_plugin.url,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/v1/chat/completions", post(handle_openai_chat))
        .route("/v1/messages", post(handle_anthropic_messages))
        .route("/api/chat", post(handle_ollama_chat))
        .route("/v1/models", get(handle_list_models))
        .layer(cors)
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", config.server.host, config.server.port).parse()?;
    info!("🔥 IgniteRouter Rust Microservice listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
