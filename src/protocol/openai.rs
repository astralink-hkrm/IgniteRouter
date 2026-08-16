use crate::core::{ChatMessage, RouterError, UnifiedRequest, UnifiedResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequestPayload {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Value>,
}

pub fn parse_openai_request(body: Value) -> Result<UnifiedRequest, RouterError> {
    serde_json::from_value::<OpenAIRequestPayload>(body)
        .map(|payload| UnifiedRequest {
            model: payload.model,
            messages: payload.messages,
            temperature: payload.temperature,
            max_tokens: payload.max_tokens,
            stream: payload.stream,
            tools: payload.tools,
        })
        .map_err(|e| RouterError::BadRequest(format!("Invalid OpenAI payload: {}", e)))
}

pub fn format_openai_response(res: &UnifiedResponse) -> Value {
    json!({
        "id": res.id,
        "object": "chat.completion",
        "created": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        "model": res.model,
        "choices": [
            {
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": res.content
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": res.prompt_tokens,
            "completion_tokens": res.completion_tokens,
            "total_tokens": res.prompt_tokens + res.completion_tokens
        },
        "_igniterouter": {
            "tier": res.tier,
            "execution_backend": res.execution_backend,
            "latency_ms": res.latency_ms
        }
    })
}
