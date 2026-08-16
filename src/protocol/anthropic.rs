use crate::core::{ChatMessage, Role, RouterError, UnifiedRequest, UnifiedResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnthropicRequestPayload {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub stream: bool,
}

pub fn parse_anthropic_request(body: Value) -> Result<UnifiedRequest, RouterError> {
    let payload: AnthropicRequestPayload = serde_json::from_value(body)
        .map_err(|e| RouterError::BadRequest(format!("Invalid Anthropic payload: {}", e)))?;

    let mut unified_messages = Vec::new();

    // Map top-level system parameter into messages[0] if present
    if let Some(sys_val) = payload.system {
        let sys_content = if let Some(s) = sys_val.as_str() {
            s.to_string()
        } else {
            sys_val.to_string()
        };
        unified_messages.push(ChatMessage {
            role: Role::System,
            content: sys_content,
            name: None,
            tool_call_id: None,
        });
    }

    unified_messages.extend(payload.messages);

    Ok(UnifiedRequest {
        model: payload.model,
        messages: unified_messages,
        temperature: payload.temperature,
        max_tokens: payload.max_tokens,
        stream: payload.stream,
        tools: None,
    })
}

pub fn format_anthropic_response(res: &UnifiedResponse) -> Value {
    json!({
        "id": res.id,
        "type": "message",
        "role": "assistant",
        "model": res.model,
        "content": [
            {
                "type": "text",
                "text": res.content
            }
        ],
        "stop_reason": "end_turn",
        "usage": {
            "input_tokens": res.prompt_tokens,
            "output_tokens": res.completion_tokens
        }
    })
}
