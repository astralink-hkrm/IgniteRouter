pub mod observation;

use crate::core::UnifiedRequest;
pub use observation::compact_observation_logs;

pub fn compact_request(req: &mut UnifiedRequest) {
    for msg in &mut req.messages {
        msg.content = compact_observation_logs(&msg.content);
    }
}
