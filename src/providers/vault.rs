use std::collections::HashMap;
use std::env;

#[derive(Clone, Default)]
pub struct ApiKeyVault {
    keys: HashMap<String, String>,
}

impl ApiKeyVault {
    pub fn new() -> Self {
        let mut keys = HashMap::new();

        if let Ok(key) = env::var("OPENAI_API_KEY") {
            keys.insert("openai".to_string(), key);
        }
        if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
            keys.insert("anthropic".to_string(), key);
        }
        if let Ok(key) = env::var("DEEPSEEK_API_KEY") {
            keys.insert("deepseek".to_string(), key);
        }
        if let Ok(key) = env::var("GEMINI_API_KEY") {
            keys.insert("gemini".to_string(), key);
        }

        Self { keys }
    }

    pub fn set_key(&mut self, provider: &str, key: &str) {
        self.keys.insert(provider.to_lowercase(), key.to_string());
    }

    pub fn get_key(&self, provider: &str) -> Option<&String> {
        self.keys.get(&provider.to_lowercase())
    }
}
