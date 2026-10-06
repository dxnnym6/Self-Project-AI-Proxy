# AI Proxy Project (`sentinel-ai-proxy`)

A lightweight, asynchronous proxy server for local LLM interactions designed to inspect traffic, prevent prompt injections, and block unauthorized model behaviors.

---

## Project Phases & Architecture

### Phase 1: Python Prototype (Completed)
- Successfully established basic connectivity to a local Ollama instance (Llama 3.2).
- Explored prompt structure and basic system-level prompts to guard against identity hallucination.

### Phase 2: Rust High-Performance Engine (`sent-prox`) — *In Progress*
- **Async Runtime:** Powered by `tokio` and `axum` for concurrent, low-latency request handling.
- **Strict Data Contracts:** Strongly typed request models (`ChatReq`) using `serde` for schema validation.
- **Pure Rust Networking:** Uses `reqwest` with `rustls` for cross-platform TLS support with zero external C/OpenSSL dependencies.

---

## Setup & Getting Started

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (1.70+)
- [Ollama](https://ollama.com/) running locally on port `11434`

1. Clone the repo: `git clone https://github.com/dxxnym6/sentinel-ai-proxy.git`
2. Ensure Ollama is running locally.
3. Build project (within sent-prox): `cargo build`
4. Start proxy server: `cargo run`

- Optional -> Install dependencies for Python scripts: `pip install -r requirements.txt`

## Roadmap
- [x] Phase 1: Python baseline & local LLM integration (hello_ai.py)
- [ ] Phase 2: Rust-based core proxy engine (sent-prox)
  - [x] Axum HTTP server baseline & TCP socket binding
  - [x] Strongly-typed ChatReq JSON serialization/deserialization
  - [ ] Outbound HTTP request forwarding to Ollama via reqwest
  - [ ] Response handling & streaming back to client
- [ ] Phase 3: Input/Output Security & Filtering Layer
  - [ ] System prompt injection detection
  - [ ] Keyword sanitization & response policy enforcement
  - [ ] Request logging and metrics middleware






