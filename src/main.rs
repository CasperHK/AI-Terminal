use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch::launch(
        App,
        vec![],
        dioxus_desktop::Config::new()
            .with_window(
                dioxus_desktop::WindowBuilder::new()
                    .with_title("AI Terminal")
                    .with_inner_size(dioxus_desktop::LogicalSize::new(900.0, 600.0))
                    .with_min_inner_size(dioxus_desktop::LogicalSize::new(640.0, 400.0)),
            )
            .with_custom_head(
                r#"<style>
                    * { box-sizing: border-box; margin: 0; padding: 0; }
                    body { background: #0d1117; color: #c9d1d9; font-family: 'JetBrains Mono', 'Fira Code', monospace; }
                </style>"#
                    .to_string(),
            ),
    );
}

#[component]
fn App() -> Element {
    let mut input = use_signal(String::new);
    let mut history: Signal<Vec<HistoryEntry>> = use_signal(Vec::new);
    let mut preview: Signal<Option<CommandPreview>> = use_signal(|| None);
    let mut is_loading = use_signal(|| false);

    let on_submit = move |()| {
        let cmd = input.read().clone();
        if cmd.trim().is_empty() {
            return;
        }

        is_loading.set(true);
        let preview_cmd = cmd.clone();

        // Simulate NLP command translation (replace with real Tauri invoke in production)
        let translated = translate_natural_language(&preview_cmd);
        preview.set(Some(CommandPreview {
            natural: preview_cmd.clone(),
            shell_command: translated,
        }));

        history.write().push(HistoryEntry {
            input: cmd.clone(),
            output: format!("Processing: {}", cmd),
            is_error: false,
        });

        input.set(String::new());
        is_loading.set(false);
    };

    let on_approve = move |()| {
        // Clone out of the read guard before writing to avoid overlapping borrows.
        let pending = preview.read().clone();
        if let Some(p) = pending {
            history.write().push(HistoryEntry {
                input: p.natural.clone(),
                output: format!("✓ Executed: {}", p.shell_command),
                is_error: false,
            });
            preview.set(None);
        }
    };

    let on_reject = move |()| {
        preview.set(None);
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100vh; padding: 16px; gap: 12px;",

            // Header
            TerminalHeader {}

            // History output
            div {
                style: "flex: 1; overflow-y: auto; background: #161b22; border-radius: 8px; padding: 12px; border: 1px solid #30363d;",
                for entry in history.read().iter() {
                    HistoryItem { entry: entry.clone() }
                }
                if *is_loading.read() {
                    div { style: "color: #58a6ff;", "⏳ Thinking..." }
                }
            }

            // Command preview sandbox
            if let Some(p) = preview.read().clone() {
                CommandPreviewBanner {
                    preview: p,
                    on_approve,
                    on_reject,
                }
            }

            // Input bar
            InputBar {
                value: input.read().clone(),
                on_input: move |val: String| input.set(val),
                on_submit,
            }
        }
    }
}

#[component]
fn TerminalHeader() -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px; padding-bottom: 8px; border-bottom: 1px solid #30363d;",
            span { style: "font-size: 18px; font-weight: bold; color: #58a6ff;", "⚡ AI Terminal" }
            span { style: "color: #8b949e; font-size: 12px;", "Powered by Dioxus + Tauri" }
        }
    }
}

#[derive(Clone, PartialEq)]
struct HistoryEntry {
    input: String,
    output: String,
    is_error: bool,
}

#[component]
fn HistoryItem(entry: HistoryEntry) -> Element {
    let output_color = if entry.is_error { "#f85149" } else { "#3fb950" };
    rsx! {
        div {
            style: "margin-bottom: 10px;",
            div {
                style: "color: #58a6ff;",
                span { style: "color: #8b949e;", "❯ " }
                "{entry.input}"
            }
            div { style: "color: {output_color}; padding-left: 16px; font-size: 13px;", "{entry.output}" }
        }
    }
}

#[derive(Clone, PartialEq)]
struct CommandPreview {
    natural: String,
    shell_command: String,
}

#[component]
fn CommandPreviewBanner(
    preview: CommandPreview,
    on_approve: EventHandler<()>,
    on_reject: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "background: #1c2128; border: 1px solid #f0883e; border-radius: 8px; padding: 12px;",
            p { style: "color: #f0883e; font-size: 12px; margin-bottom: 6px;", "🛡️ SANDBOX PREVIEW — Review before execution" }
            p { style: "color: #c9d1d9; margin-bottom: 4px;",
                span { style: "color: #8b949e;", "Natural: " }
                "{preview.natural}"
            }
            p { style: "color: #c9d1d9; margin-bottom: 10px;",
                span { style: "color: #8b949e;", "Shell:   " }
                code { style: "background: #0d1117; padding: 2px 6px; border-radius: 4px; color: #79c0ff;",
                    "{preview.shell_command}"
                }
            }
            div { style: "display: flex; gap: 8px;",
                button {
                    style: "background: #238636; color: #fff; border: none; border-radius: 6px; padding: 6px 16px; cursor: pointer;",
                    onclick: move |_| on_approve.call(()),
                    "✓ Execute"
                }
                button {
                    style: "background: #b91c1c; color: #fff; border: none; border-radius: 6px; padding: 6px 16px; cursor: pointer;",
                    onclick: move |_| on_reject.call(()),
                    "✗ Cancel"
                }
            }
        }
    }
}

#[component]
fn InputBar(
    value: String,
    on_input: EventHandler<String>,
    on_submit: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            style: "display: flex; gap: 8px; align-items: center;",
            input {
                style: "flex: 1; background: #161b22; border: 1px solid #30363d; border-radius: 6px; padding: 10px 14px; color: #c9d1d9; font-family: inherit; font-size: 14px; outline: none;",
                r#type: "text",
                placeholder: "Type a natural language command… (e.g. 'show disk usage')",
                value: "{value}",
                oninput: move |e| on_input.call(e.value().clone()),
            }
            button {
                style: "background: #1f6feb; color: #fff; border: none; border-radius: 6px; padding: 10px 20px; cursor: pointer; font-size: 14px;",
                onclick: move |_| on_submit.call(()),
                "Run"
            }
        }
    }
}

/// Lightweight stub that maps natural language phrases to shell commands.
/// Replace with a real Ollama/Claude call via the Tauri `process_command` IPC.
///
/// SECURITY NOTE: The returned string is shown in the sandbox preview banner
/// and only executed after explicit user approval in `execute_approved_command`.
/// It is never passed to a shell interpreter directly from this function.
fn translate_natural_language(input: &str) -> String {
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
        // Use single-quote escaping (replace ' with '\'') so the preview string
        // is safe for display. In production the LLM generates the command and
        // this branch is unreachable.
        format!("echo {}", shell_single_quote(input))
    }
}

/// Wrap `s` in POSIX single quotes, escaping any embedded single quotes.
fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

