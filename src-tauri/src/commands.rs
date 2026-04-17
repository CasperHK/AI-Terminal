use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Unified error type for all Tauri command handlers.
#[derive(Debug, Error, Serialize)]
pub enum CommandError {
    #[error("LLM request failed: {0}")]
    LlmError(String),
    #[error("Shell execution failed: {0}")]
    ShellError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("MCP connection error: {0}")]
    McpError(String),
}

/// The response from an LLM for natural language → shell command translation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NlpCommandResult {
    /// The original natural-language input.
    pub natural_input: String,
    /// The generated shell command ready for sandboxed preview.
    pub shell_command: String,
    /// Human-readable explanation of what the command does.
    pub explanation: String,
    /// Confidence score in [0.0, 1.0].
    pub confidence: f32,
}

/// Request payload for the MCP context bridge.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct McpContext {
    /// The MCP server URL (e.g. `http://localhost:3000`).
    pub server_url: String,
    /// The context key to retrieve (e.g. `slack.latest_message`).
    pub context_key: String,
}

/// Translate a natural-language string into a shell command using the
/// configured LLM backend (Ollama local or cloud API).
///
/// # Errors
/// Returns [`CommandError::LlmError`] if the LLM backend is unavailable.
#[tauri::command]
pub async fn process_command(input: String) -> Result<NlpCommandResult, String> {
    tracing::info!(input = %input, "Processing natural language command");

    // --- LLM routing ---
    // 1. Try the local Ollama endpoint first (latency ~50 ms, no cost).
    // 2. Fall back to a configured cloud LLM (Claude / GPT-4).
    // Both paths are stubbed here; wire up `reqwest` calls in production.
    let shell_command = stub_translate(&input);

    Ok(NlpCommandResult {
        natural_input: input.clone(),
        shell_command: shell_command.clone(),
        explanation: format!(
            "Translates '{}' using the system shell",
            input.trim()
        ),
        confidence: 0.95,
    })
}

/// Execute a pre-approved shell command after user confirmation in the sandbox.
///
/// # Safety
/// This function must only be called after the user has explicitly approved
/// the command shown in the `CommandPreviewBanner`.
#[tauri::command]
pub async fn execute_approved_command(
    shell_command: String,
) -> Result<String, String> {
    tracing::info!(cmd = %shell_command, "Executing approved command");

    // NOTE: In production, use `tauri-plugin-shell` to spawn a child process
    // with a restricted environment.  The stub below returns a placeholder.
    Ok(format!(
        "[sandbox] Would execute: `{}`\nReplace this stub with tauri_plugin_shell::open()",
        shell_command
    ))
}

/// Fetch context from an MCP-compatible server.
///
/// Used to give the LLM awareness of external data (Slack, Notion, etc.).
#[tauri::command]
pub async fn fetch_mcp_context(
    server_url: String,
    context_key: String,
) -> Result<String, String> {
    tracing::info!(url = %server_url, key = %context_key, "Fetching MCP context");

    // TODO: Replace with a real HTTP call to the MCP server using `reqwest`.
    // Example:
    //   let client = reqwest::Client::new();
    //   let resp = client.get(&format!("{}/context/{}", server_url, context_key))
    //       .send().await.map_err(|e| e.to_string())?;
    //   resp.text().await.map_err(|e| e.to_string())

    Ok(format!(
        "{{\"key\":\"{}\",\"value\":\"<mcp-placeholder>\"}}",
        context_key
    ))
}

/// Simple keyword-based NLP stub.
/// Replace with a real Ollama or cloud LLM request in production.
fn stub_translate(input: &str) -> String {
    let lower = input.to_lowercase();
    if lower.contains("disk") || lower.contains("storage") {
        "df -h".to_string()
    } else if lower.contains("process") || lower.contains("running") {
        "ps aux".to_string()
    } else if lower.contains("memory") || lower.contains("ram") {
        "free -h".to_string()
    } else if lower.contains("list") || lower.contains("files") {
        "ls -la".to_string()
    } else if lower.contains("network") || lower.contains("ip") {
        "ip addr show".to_string()
    } else {
        format!("echo '{}'", input.replace('\'', "\\'"))
    }
}
