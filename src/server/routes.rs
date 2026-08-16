use crate::cache::{compute_prompt_hash, LruCache};
use crate::classifier::LocalHardwareMonitor;
use crate::compactor::compact_request;
use crate::core::RouterError;
use crate::guardrails::GuardrailEngine;
use crate::protocol::{
    format_anthropic_response, format_ollama_response, format_openai_response,
    parse_anthropic_request, parse_ollama_request, parse_openai_request,
};
use crate::providers::{
    ApiKeyVault, CloudProvider, FailoverEngine, LocalOllamaProvider, ModelProvider, OmniRoutePlugin,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub cache: LruCache,
    pub guardrails: Arc<GuardrailEngine>,
    pub hardware: LocalHardwareMonitor,
    pub vault: ApiKeyVault,
    pub omniroute_enabled: bool,
    pub omniroute_url: String,
}

pub async fn handle_openai_chat(
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, RouterError> {
    let mut req = parse_openai_request(body)?;

    // 1. Compact CLI Agent Context
    compact_request(&mut req);

    // 2. Sanitize prompt PII
    for msg in &mut req.messages {
        msg.content = state.guardrails.sanitize_prompt(&msg.content);
    }

    // 3. Compute Cache Key
    let cache_key = compute_prompt_hash(&format!("{:?}-{}", req.messages, req.model));
    if let Some(cached) = state.cache.get(&cache_key) {
        return Ok(Json(format_openai_response(&cached)));
    }

    // 4. Construct Candidates
    let mut providers: Vec<Box<dyn ModelProvider>> = Vec::new();

    // Candidate 1: Local Ollama (if running)
    if state.hardware.is_local_gpu_ready() {
        providers.push(Box::new(LocalOllamaProvider::new(None)));
    }

    // Candidate 2: Cloud OpenAI (if key present)
    if let Some(key) = state.vault.get_key("openai") {
        providers.push(Box::new(CloudProvider::new(
            "cloud_openai",
            "https://api.openai.com/v1",
            key,
        )));
    }

    // Candidate 3: Cloud Anthropic (if key present)
    if let Some(key) = state.vault.get_key("anthropic") {
        providers.push(Box::new(CloudProvider::new(
            "cloud_anthropic",
            "https://api.anthropic.com/v1",
            key,
        )));
    }

    // Candidate 4: Optional OmniRoute Plugin (if enabled in config)
    if state.omniroute_enabled {
        providers.push(Box::new(OmniRoutePlugin::new(
            true,
            Some(state.omniroute_url.clone()),
        )));
    }

    // Default Fallback: Free Local Ollama Mock Driver
    if providers.is_empty() {
        providers.push(Box::new(LocalOllamaProvider::new(None)));
    }

    let engine = FailoverEngine::new(providers);
    let res = engine.execute_with_failover(req).await?;

    // 5. Store Cache
    state.cache.put(cache_key, res.clone());

    Ok(Json(format_openai_response(&res)))
}

pub async fn handle_anthropic_messages(
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, RouterError> {
    let mut req = parse_anthropic_request(body)?;
    compact_request(&mut req);

    let cache_key = compute_prompt_hash(&format!("{:?}-{}", req.messages, req.model));
    if let Some(cached) = state.cache.get(&cache_key) {
        return Ok(Json(format_anthropic_response(&cached)));
    }

    let mut providers: Vec<Box<dyn ModelProvider>> = Vec::new();
    if let Some(key) = state.vault.get_key("anthropic") {
        providers.push(Box::new(CloudProvider::new(
            "cloud_anthropic",
            "https://api.anthropic.com/v1",
            key,
        )));
    } else {
        providers.push(Box::new(LocalOllamaProvider::new(None)));
    }

    let engine = FailoverEngine::new(providers);
    let res = engine.execute_with_failover(req).await?;
    state.cache.put(cache_key, res.clone());

    Ok(Json(format_anthropic_response(&res)))
}

pub async fn handle_ollama_chat(
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, RouterError> {
    let mut req = parse_ollama_request(body)?;
    compact_request(&mut req);

    let provider = LocalOllamaProvider::new(None);
    let res = provider.complete(req).await?;

    Ok(Json(format_ollama_response(&res)))
}

pub async fn handle_list_models() -> Json<Value> {
    Json(json!({
        "object": "list",
        "data": [
            { "id": "blockrun/auto", "object": "model", "owned_by": "igniterouter" },
            { "id": "claude-sonnet-4.6", "object": "model", "owned_by": "anthropic" },
            { "id": "gpt-5.4", "object": "model", "owned_by": "openai" },
            { "id": "deepseek-r1", "object": "model", "owned_by": "deepseek" },
            { "id": "llama3", "object": "model", "owned_by": "meta" }
        ]
    }))
}
