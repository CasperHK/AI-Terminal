// Prevent a console window from opening on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{execute_approved_command, fetch_mcp_context, process_command};

fn main() {
    // Initialise structured logging (respects the RUST_LOG env var).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_terminal_backend=info,tauri=warn".parse().unwrap()),
        )
        .init();

    tauri::Builder::default()
        // Register native plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        // Register IPC command handlers exposed to the Dioxus frontend
        .invoke_handler(tauri::generate_handler![
            process_command,
            execute_approved_command,
            fetch_mcp_context,
        ])
        .run(tauri::generate_context!())
        .expect("error while running AI Terminal");
}
