use regex::Regex;

pub struct GuardrailEngine {
    aws_key_re: Regex,
    email_re: Regex,
}

impl GuardrailEngine {
    pub fn new() -> Self {
        Self {
            aws_key_re: Regex::new(r"(?i)(AKIA[0-9A-Z]{16})").unwrap(),
            email_re: Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
        }
    }

    pub fn sanitize_prompt(&self, prompt: &str) -> String {
        let scrubbed_aws = self.aws_key_re.replace_all(prompt, "[REDACTED_AWS_KEY]");
        let scrubbed_email = self.email_re.replace_all(&scrubbed_aws, "[REDACTED_EMAIL]");
        scrubbed_email.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guardrail_sanitization() {
        let engine = GuardrailEngine::new();
        let prompt = "My key is AKIAIOSFODNN7EXAMPLE and email is test@example.com";
        let clean = engine.sanitize_prompt(prompt);
        assert!(clean.contains("[REDACTED_AWS_KEY]"));
        assert!(clean.contains("[REDACTED_EMAIL]"));
    }
}
