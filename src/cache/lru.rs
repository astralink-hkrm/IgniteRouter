use crate::core::UnifiedResponse;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct LruCache {
    store: Arc<DashMap<String, UnifiedResponse>>,
}

impl LruCache {
    pub fn new() -> Self {
        Self {
            store: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, hash: &str) -> Option<UnifiedResponse> {
        self.store.get(hash).map(|r| r.value().clone())
    }

    pub fn put(&self, hash: String, response: UnifiedResponse) {
        self.store.insert(hash, response);
    }
}
