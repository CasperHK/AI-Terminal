# 💻 AI Terminal (Dioxus + Tauri Edition)

AI Terminal is a high-performance desktop command-line interface built entirely in Rust. It combines the reactive UI power of Dioxus with the native system capabilities of Tauri, creating a "human-first" terminal that shares context across your apps via the Model Context Protocol (MCP).

------------------------------
## ✨ Core Features

* 🗣️ **Human-Language Commands:** Use natural language to manage your OS. Dioxus handles the high-speed UI updates while the Rust backend executes your intent.
* 🔗 **Context Bridge (MCP):** Seamlessly connect to external apps. Your terminal "knows" your Slack messages or Notion notes thanks to [MCP integration](https://kiro.dev/docs/cli/).
* 🦀 **100% Rust Logic:** No JavaScript overhead. Both your UI (via Dioxus) and system logic (via Tauri) run on the memory-safe Rust engine.
* 🛡️ **Agentic Sandboxing:** AI-generated commands are previewed in the TUI for your approval before hitting the system shell.
* ⚡ **Ultra-Lightweight:** Built with Tauri 2.0, resulting in sub-10MB binaries and minimal memory usage. 

------------------------------
## 🛠️ Technical Stack

* Frontend: Dioxus (using the RSX macro for React-like declarative UI in Rust).
* Backend/Runtime: Tauri for window management and native API access (File System, Shell, HTTP).
* Protocol: Model Context Protocol (MCP) for cross-app data sharing.
* AI Agent: Integrates with Ollama for local LLMs or Claude/GPT-4 via Tauri’s HTTP client. 

------------------------------
## 🚀 Quick Start (Development)## Prerequisites
Ensure you have the Rust toolchain and Tauri CLI installed.

### 1. Clone & Install
```bash
git clone https://github.com
cd ai-terminal
cargo install tauri-cli --version "^2.0.0"
```

### 2. Run in Development Mode
```bash
cargo tauri dev
```

------------------------------
## 📖 Why Dioxus + Tauri?

* **ype Safety:** Shared types between your UI and system backend mean fewer runtime bugs.
* **Performance:** [Dioxus](https://dioxuslabs.com/blog/release-030/) is faster and less resource-intensive than traditional web frontends.
* **Ease of Use:** You get a React-like developer experience without leaving the Rust ecosystem. [9, 10] 

------------------------------
## 🤝 Contributing
Join us in making the terminal more human. Check out our Contributing Guide to get started with Dioxus components or Tauri commands.

------------------------------
## 📄 License
Licensed under the MIT License.
