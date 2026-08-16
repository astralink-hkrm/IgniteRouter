use serde_json::json;

pub fn format_openai_sse_chunk(id: &str, model: &str, content_delta: &str) -> String {
    let payload = json!({
        "id": id,
        "object": "chat.completion.chunk",
        "created": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        "model": model,
        "choices": [
            {
                "index": 0,
                "delta": {
                    "content": content_delta
                },
                "finish_reason": serde_json::Value::Null
            }
        ]
    });

    format!("data: {}\n\n", payload)
}

pub fn format_anthropic_sse_event(event_type: &str, payload: serde_json::Value) -> String {
    format!("event: {}\ndata: {}\n\n", event_type, payload)
}
