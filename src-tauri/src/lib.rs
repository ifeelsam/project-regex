//! Regex core library.
//!
//! All business logic lives here so it can be shared by the desktop binary and
//! the mobile entry points, and unit-tested without a running webview.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "regex_lib=info,warn".into()),
        )
        .init();

    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init());

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_global_shortcut::Builder::new().build());

    builder
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .expect("error while running the Regex application");
}

/// Trivial liveness command used while wiring up the shell.
#[tauri::command]
fn ping() -> &'static str {
    "pong"
}
