use sha2::{Digest, Sha256};

pub fn compute_prompt_hash(payload: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(payload.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_prompt_hash() {
        let h1 = compute_prompt_hash("hello world");
        let h2 = compute_prompt_hash("hello world");
        assert_eq!(h1, h2);
    }
}
