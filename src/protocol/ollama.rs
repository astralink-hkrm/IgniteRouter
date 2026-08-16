use crate::core::{ChatMessage, RouterError, UnifiedRequest, UnifiedResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaOptions {
    pub temperature: Option<f32>,
    pub num_predict: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequestPayload {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OllamaOptions>,
    #[serde(default)]
    pub stream: bool,
}

pub fn parse_ollama_request(body: Value) -> Result<UnifiedRequest, RouterError> {
    let payload: OllamaRequestPayload = serde_json::from_value(body)
        .map_err(|e| RouterError::BadRequest(format!("Invalid Ollama payload: {}", e)))?;

    let (temp, max_tok) = match payload.options {
        Some(opts) => (opts.temperature, opts.num_predict),
        None => (None, None),
    };

    Ok(UnifiedRequest {
        model: payload.model,
        messages: payload.messages,
        temperature: temp,
        max_tokens: max_tok,
        stream: payload.stream,
        tools: None,
    })
}

pub fn format_ollama_response(res: &UnifiedResponse) -> Value {
    json!({
        "model": res.model,
        "created_at": chrono_like_now(),
        "message": {
            "role": "assistant",
            "content": res.content
        },
        "done": true,
        "prompt_eval_count": res.prompt_tokens,
        "eval_count": res.completion_tokens
    })
}

fn chrono_like_now() -> String {
    "2026-08-16T11:40:00Z".to_string()
}
