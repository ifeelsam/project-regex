use tauri::State;
use tauri_plugin_shell::ShellExt;

use crate::breakdown;
use crate::db;
use crate::db::breakdowns::Breakdown;
use crate::db::error::DbError;
use crate::AppState;

#[tauri::command]
pub async fn start_breakdown(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    item_id: String,
    project_id: Option<String>,
    personal_use_confirmed: bool,
) -> Result<Breakdown, DbError> {
    let breakdown_row =
        db::breakdowns::create(&state.pool, &item_id, project_id.as_deref()).await?;

    let pool = state.pool.clone();
    let cancels = state.breakdown_cancels.clone();
    let breakdown_id = breakdown_row.id.clone();
    let project_id_clone = project_id.clone();

    tauri::async_runtime::spawn(async move {
        breakdown::run(
            app,
            pool,
            cancels,
            breakdown_id,
            item_id,
            project_id_clone,
            personal_use_confirmed,
        )
        .await;
    });

    Ok(breakdown_row)
}

#[tauri::command]
pub async fn cancel_breakdown(
    state: State<'_, AppState>,
    breakdown_id: String,
) -> Result<bool, DbError> {
    Ok(breakdown::cancel_job(&state.breakdown_cancels, &breakdown_id))
}

#[tauri::command]
pub async fn list_project_breakdowns(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<Vec<Breakdown>, DbError> {
    db::breakdowns::list_for_project(&state.pool, &project_id).await
}

#[tauri::command]
pub async fn list_project_assets(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<Vec<db::breakdowns::Asset>, DbError> {
    db::assets::list_for_project(&state.pool, &project_id).await
}

#[derive(serde::Serialize)]
pub struct MediaToolsStatus {
    pub yt_dlp: bool,
    pub ffmpeg: bool,
    pub whisper: bool,
}

#[tauri::command]
pub async fn check_media_tools(app: tauri::AppHandle) -> Result<MediaToolsStatus, String> {
    Ok(MediaToolsStatus {
        yt_dlp: app.shell().sidecar("yt-dlp").is_ok(),
        ffmpeg: app.shell().sidecar("ffmpeg").is_ok(),
        whisper: app.shell().sidecar("whisper").is_ok(),
    })
}
