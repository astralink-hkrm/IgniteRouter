use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnifiedResponse {
    pub id: String,
    pub model: String,
    pub content: String,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub tier: String,
    pub execution_backend: String,
    pub latency_ms: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityTier {
    Simple,
    Medium,
    Complex,
    Reasoning,
}

impl std::fmt::Display for ComplexityTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexityTier::Simple => write!(f, "SIMPLE"),
            ComplexityTier::Medium => write!(f, "MEDIUM"),
            ComplexityTier::Complex => write!(f, "COMPLEX"),
            ComplexityTier::Reasoning => write!(f, "REASONING"),
        }
    }
}
