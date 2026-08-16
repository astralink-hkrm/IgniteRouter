/// Intercepts and cleans reasoning `<think>...</think>` tags to prevent bleeding into code outputs
pub fn filter_reasoning_tags(content: &str) -> String {
    if !content.contains("<think>") {
        return content.to_string();
    }

    let re = regex::Regex::new(r"(?s)<think>.*?</think>").unwrap();
    re.replace_all(content, "").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_reasoning_tags() {
        let input = "<think>\nLet me calculate 2+2=4.\n</think>\n\nHere is the code:\nfn main() {}";
        let expected = "Here is the code:\nfn main() {}";
        assert_eq!(filter_reasoning_tags(input), expected);
    }
}
