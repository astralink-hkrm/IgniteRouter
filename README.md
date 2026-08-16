<div align="center">

# 🔥 IgniteRouter (`igniterouter-rs`)
### Universal, Ultra-Fast, Hybrid (Local GPU + Cloud API) LLM Router & Gateway in Rust

</div>

---

## ⚡ What is IgniteRouter?

**IgniteRouter** is a general-purpose, microsecond-latency **LLM Router & Proxy Microservice** written in high-performance **Rust (Tokio + Axum)**.

It sits locally between **any CLI coding agent** (Claude Code, Aider, OpenClaw, Cursor, Continue.dev, Goose, Roo Code, Cline) and **any LLM backend** — intelligently routing requests between **Local GPUs** (Ollama, vLLM, llama.cpp at $0 cost & 100% data privacy) and **Cloud APIs** (OpenAI, Anthropic, Google Gemini, DeepSeek, xAI).

---

## 🌟 Key Features

- ⚡ **Microsecond Overhead (<100μs)**: Axum + Tokio async runtime in native Rust with **zero Garbage Collection (GC) pauses** and a micro-thin binary.
- 🔌 **Universal Agent Compatibility**: Native wire format adapters for OpenAI (`/v1/chat/completions`), Anthropic (`/v1/messages` for Claude Code CLI), and Ollama (`/api/chat`).
- 🖥️ **Local GPU Auto-Discovery**: Automatically detects local GPUs and local LLM runtimes (Ollama on `:11434` / vLLM on `:8000`), routing routine/syntax queries locally for **$0 cost and 100% data privacy**.
- 🧠 **15-Dimension SIMD Classifier**: Evaluates code density, token length, math symbols, file paths, and context depth in sub-100 microseconds to select the optimal model.
- 🗜️ **7-Layer Token Compactor**: Scrubs repetitive CLI agent terminal output, file reads, and tool execution logs, saving **50–97% on context tokens**.
- 🧩 **Optional OmniRoute Free-Tier Plugin**: Optionally plug in OmniRoute (`omniroute_plugin.enabled: true`) to leverage ~1.51 billion free tokens/month across 90+ free provider pools.
- 🛡️ **Dynamic Failover Engine**: Automatically retries failed queries across secondary candidate backends in **<5ms** upon HTTP 429 rate limits or 5xx server errors.
- 🔒 **Lock-Free LRU Cache & SHA-256 Dedup**: In-memory `DashMap` cache providing **0ms responses** and $0 cost on repeated prompts.

---

## 🚀 Quick Start

### 1. Build & Install IgniteRouter
```bash
# Build release binary
cargo build --release

# Run IgniteRouter microservice on port 8402
cargo run --release -- start
```

### 2. Connect Your CLI Agents & Tools

| CLI Agent / Application | Configuration Command / Environment Variable |
| :--- | :--- |
| **Claude Code CLI** | `export ANTHROPIC_BASE_URL=http://localhost:8402`<br/>`export ANTHROPIC_API_KEY=unused` |
| **Aider CLI** | `aider --openai-api-base http://localhost:8402/v1 --openai-api-key unused --model blockrun/auto` |
| **OpenClaw Agent** | Set endpoint to `http://localhost:8402/v1/` |
| **Cursor IDE** | Settings → Models → OpenAI-compatible Base URL: `http://localhost:8402/v1/` |
| **Continue.dev** | `~/.continue/config.yaml`: `apiBase: http://localhost:8402/v1/`, `model: blockrun/auto` |

---

## ⚙️ Configuration (`~/.igniterouter/config.yaml`)

```yaml
server:
  port: 8402
  host: "127.0.0.1"

# Routing Policy Strategy: AUTO | ECO | LOCAL_ONLY | CLOUD_PREMIUM
policy: AUTO

# Optional OmniRoute Provider Plugin (Disabled by default)
omniroute_plugin:
  enabled: false
  url: "http://localhost:3000/v1"
```

---

## 🩺 System Commands

```bash
# Start the proxy microservice daemon
igniterouter start

# Check configuration settings
igniterouter config

# Run system diagnostics (API keys & local GPU Ollama detection)
igniterouter doctor
```

---

## 📄 License
MIT License. Free and open source.
