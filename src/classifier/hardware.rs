use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct LocalHardwareMonitor {
    pub ollama_available: Arc<AtomicBool>,
    pub vllm_available: Arc<AtomicBool>,
}

impl LocalHardwareMonitor {
    pub fn new() -> Self {
        let monitor = Self {
            ollama_available: Arc::new(AtomicBool::new(false)),
            vllm_available: Arc::new(AtomicBool::new(false)),
        };

        let monitor_clone = monitor.clone();
        tokio::spawn(async move {
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(2))
                .build()
                .unwrap_or_default();

            loop {
                // Poll local Ollama endpoint
                let ollama_online = client
                    .get("http://localhost:11434/api/tags")
                    .send()
                    .await
                    .map(|r| r.status().is_success())
                    .unwrap_or(false);

                // Poll local vLLM endpoint
                let vllm_online = client
                    .get("http://localhost:8000/v1/models")
                    .send()
                    .await
                    .map(|r| r.status().is_success())
                    .unwrap_or(false);

                monitor_clone
                    .ollama_available
                    .store(ollama_online, Ordering::Relaxed);
                monitor_clone
                    .vllm_available
                    .store(vllm_online, Ordering::Relaxed);

                sleep(Duration::from_secs(15)).await;
            }
        });

        monitor
    }

    pub fn is_local_gpu_ready(&self) -> bool {
        self.ollama_available.load(Ordering::Relaxed) || self.vllm_available.load(Ordering::Relaxed)
    }
}
