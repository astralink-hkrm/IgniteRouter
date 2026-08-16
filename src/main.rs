pub mod cache;
pub mod classifier;
pub mod compactor;
pub mod config;
pub mod core;
pub mod guardrails;
pub mod protocol;
pub mod providers;
pub mod server;

use clap::{Parser, Subcommand};
use config::IgniteConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "igniterouter")]
#[command(author = "Saksham Agarwal <sakshamagarwalm2@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "🔥 Universal, Ultra-Fast, Hybrid (Local GPU + Cloud API) LLM Router & Gateway in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the IgniteRouter Rust microservice server
    Start,
    /// View current configuration settings
    Config,
    /// Check local hardware, Ollama status, and environment API keys
    Doctor,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "igniterouter_rs=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    let config = IgniteConfig::load();

    match cli.command {
        Some(Commands::Start) | None => {
            println!("🔥 Starting IgniteRouter Rust Microservice...");
            server::run_server(config).await?;
        }
        Some(Commands::Config) => {
            println!("📋 IgniteRouter Configuration:");
            println!("   Host: {}", config.server.host);
            println!("   Port: {}", config.server.port);
            println!("   OmniRoute Plugin Enabled: {}", config.omniroute_plugin.enabled);
            println!("   OmniRoute Plugin URL: {}", config.omniroute_plugin.url);
        }
        Some(Commands::Doctor) => {
            println!("🩺 IgniteRouter System Diagnostics:");
            let vault = providers::ApiKeyVault::new();
            println!("   OpenAI Key: {}", if vault.get_key("openai").is_some() { "✅ Present" } else { "❌ Missing" });
            println!("   Anthropic Key: {}", if vault.get_key("anthropic").is_some() { "✅ Present" } else { "❌ Missing" });
            println!("   DeepSeek Key: {}", if vault.get_key("deepseek").is_some() { "✅ Present" } else { "❌ Missing" });
            println!("   Gemini Key: {}", if vault.get_key("gemini").is_some() { "✅ Present" } else { "❌ Missing" });
            
            let monitor = classifier::LocalHardwareMonitor::new();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("   Local Ollama GPU: {}", if monitor.ollama_available.load(std::sync::atomic::Ordering::Relaxed) { "✅ Running" } else { "⚠️ Offline / Not detected" });
        }
    }

    Ok(())
}
