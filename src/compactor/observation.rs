pub fn compact_observation_logs(text: &str) -> String {
    let mut lines = Vec::new();
    let mut prev_line = "";

    for line in text.lines() {
        let trimmed = line.trim();
        // Deduplicate consecutive identical lines
        if trimmed == prev_line && !trimmed.is_empty() {
            continue;
        }
        prev_line = trimmed;
        lines.push(line);
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_observation_logs() {
        let input = "cargo build\ncargo build\ncargo build\nDone.";
        let expected = "cargo build\nDone.";
        assert_eq!(compact_observation_logs(input), expected);
    }
}
