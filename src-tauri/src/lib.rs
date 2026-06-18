//! Regex core library.
//!
//! All business logic lives here so it can be shared by the desktop binary and
//! the mobile entry points, and unit-tested without a running webview.

mod commands;
#[cfg(desktop)]
mod breakdown;
#[cfg(desktop)]
mod breakdown_commands;
mod db;
mod enrich;
mod metadata;
mod transcripts;

use sqlx::SqlitePool;
use tauri::Manager;

#[cfg(desktop)]
use breakdown::CancelMap;
#[cfg(desktop)]
use std::collections::HashMap;
#[cfg(desktop)]
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub pool: SqlitePool,
    #[cfg(desktop)]
    pub breakdown_cancels: CancelMap,
}

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
    let builder = builder
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init());

    builder
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                let path = handle
                    .path()
                    .app_data_dir()
                    .expect("app data directory unavailable")
                    .join("regex.db");

                let pool = db::connect(&path)
                    .await
                    .expect("failed to open the local database");

                #[cfg(desktop)]
                let state = AppState {
                    pool,
                    breakdown_cancels: Arc::new(Mutex::new(HashMap::new())),
                };

                #[cfg(not(desktop))]
                let state = AppState { pool };

                handle.manage(state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::capture_item,
            commands::list_inbox_items,
            commands::list_develop_items,
            commands::get_idea_detail,
            commands::update_develop_note,
            commands::add_item_reference,
            commands::remove_item_reference,
            commands::list_items,
            commands::get_item,
            commands::update_item_note,
            commands::update_item_status,
            commands::delete_item,
            commands::list_tags,
            commands::set_item_tags,
            commands::search_items,
            commands::create_project,
            commands::list_projects,
            commands::list_project_summaries,
            commands::get_project_detail,
            commands::graduate_item,
            commands::detect_platform,
            commands::default_captured_on,
            enrich::enrich_item,
            enrich::add_auto_transcript,
            #[cfg(desktop)]
            breakdown_commands::start_breakdown,
            #[cfg(desktop)]
            breakdown_commands::cancel_breakdown,
            #[cfg(desktop)]
            breakdown_commands::list_project_breakdowns,
            #[cfg(desktop)]
            breakdown_commands::list_project_assets,
            #[cfg(desktop)]
            breakdown_commands::check_media_tools,
        ])
        .run(tauri::generate_context!())
        .expect("error while running the Regex application");
}
