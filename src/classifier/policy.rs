use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoutingPolicy {
    Auto,
    Eco,
    LocalOnly,
    CloudPremium,
}

impl Default for RoutingPolicy {
    fn default() -> Self {
        RoutingPolicy::Auto
    }
}
