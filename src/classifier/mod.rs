pub mod dimensions;
pub mod hardware;
pub mod policy;

pub use dimensions::evaluate_dimensions;
pub use hardware::LocalHardwareMonitor;
pub use policy::RoutingPolicy;

use crate::core::{ComplexityTier, UnifiedRequest};

pub fn classify_request(req: &UnifiedRequest) -> ComplexityTier {
    let scores = evaluate_dimensions(req);

    if scores.token_count > 16000 || scores.file_marker_score > 3 {
        ComplexityTier::Complex
    } else if scores.math_score > 5 {
        ComplexityTier::Reasoning
    } else if scores.code_ratio > 0.45 || scores.token_count > 4000 {
        ComplexityTier::Medium
    } else {
        ComplexityTier::Simple
    }
}
