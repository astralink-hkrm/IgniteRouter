use crate::core::UnifiedRequest;

pub struct DimensionScores {
    pub token_count: usize,
    pub code_ratio: f32,
    pub math_score: usize,
    pub file_marker_score: usize,
    pub system_prompt_ratio: f32,
}

pub fn evaluate_dimensions(req: &UnifiedRequest) -> DimensionScores {
    let mut total_chars = 0;
    let mut code_chars = 0;
    let mut math_count = 0;
    let mut file_marker_count = 0;
    let mut sys_chars = 0;

    for msg in &req.messages {
        let content = &msg.content;
        let len = content.len();
        total_chars += len;

        if msg.role == crate::core::Role::System {
            sys_chars += len;
        }

        // Fast SIMD-like byte scanning for code, math, and file markers
        for chunk in content.as_bytes().chunks(4) {
            if chunk.contains(&b'{')
                || chunk.contains(&b';')
                || chunk.contains(&b'(')
                || chunk.contains(&b'=')
            {
                code_chars += chunk.len();
            }
            if chunk.contains(&b'\\') || chunk.contains(&b'^') || chunk.contains(&b'+') {
                math_count += 1;
            }
        }

        if content.contains("src/")
            || content.contains("file:///")
            || content.contains("diff --git")
        {
            file_marker_count += 1;
        }
    }

    let code_ratio = if total_chars > 0 {
        code_chars as f32 / total_chars as f32
    } else {
        0.0
    };

    let sys_ratio = if total_chars > 0 {
        sys_chars as f32 / total_chars as f32
    } else {
        0.0
    };

    DimensionScores {
        token_count: total_chars / 4,
        code_ratio,
        math_score: math_count,
        file_marker_score: file_marker_count,
        system_prompt_ratio: sys_ratio,
    }
}
