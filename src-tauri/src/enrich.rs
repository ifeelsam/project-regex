use std::path::PathBuf;

use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

use crate::db;
use crate::db::error::DbError;
use crate::db::models::Item;
use crate::metadata;
use crate::transcripts;
use crate::AppState;

#[derive(Clone, serde::Serialize)]
pub struct ItemEnrichedEvent {
    pub item: Item,
}

#[tauri::command]
pub async fn enrich_item(
    app: AppHandle,
    state: State<'_, AppState>,
    item_id: String,
) -> Result<(), DbError> {
    let pool = state.pool.clone();
    let item = db::items::get(&pool, &item_id)
        .await?
        .ok_or_else(|| DbError::Message("item not found".into()))?;

    let Some(url) = item.url.clone() else {
        return Ok(());
    };

    tauri::async_runtime::spawn(async move {
        if let Err(error) = run_enrichment(&app, &pool, &item_id, &url, item.platform).await {
            tracing::warn!("item enrichment failed for {item_id}: {error}");
        }
    });

    Ok(())
}

async fn run_enrichment(
    app: &AppHandle,
    pool: &sqlx::SqlitePool,
    item_id: &str,
    url: &str,
    platform: db::models::Platform,
) -> Result<(), DbError> {
    let meta = metadata::fetch(url, platform).await;

    let thumbnail_path = if let Some(thumb_url) = meta.thumbnail_url.as_deref() {
        download_thumbnail(app, item_id, thumb_url).await
    } else {
        None
    };

    let item = db::items::update_metadata(
        pool,
        item_id,
        meta.title.as_deref(),
        meta.author.as_deref(),
        thumbnail_path.as_deref(),
    )
    .await?;

    let _ = app.emit("item-enriched", ItemEnrichedEvent { item: item.clone() });

    transcripts::fetch_auto_transcript(pool, item_id, url, platform).await?;

    if let Some(updated) = db::items::get(pool, item_id).await? {
        let _ = app.emit("item-enriched", ItemEnrichedEvent { item: updated });
    }

    Ok(())
}

async fn download_thumbnail(app: &AppHandle, item_id: &str, url: &str) -> Option<String> {
    let bytes = reqwest::get(url).await.ok()?.bytes().await.ok()?;
    let dir = app
        .path()
        .app_data_dir()
        .ok()?
        .join("thumbnails");
    std::fs::create_dir_all(&dir).ok()?;
    let path: PathBuf = dir.join(format!("{item_id}.jpg"));
    std::fs::write(&path, bytes).ok()?;
    Some(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn add_auto_transcript(
    state: State<'_, AppState>,
    item_id: String,
    text: String,
    lang: Option<String>,
) -> Result<(), DbError> {
    let id = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().to_rfc3339();
    let lang = lang.unwrap_or_else(|| "en".into());

    sqlx::query(
        r#"
        INSERT INTO transcripts (id, item_id, lang, text, source, created_at)
        VALUES (?, ?, ?, ?, 'auto', ?)
        "#,
    )
    .bind(id)
    .bind(item_id)
    .bind(lang)
    .bind(text)
    .bind(created_at)
    .execute(&state.pool)
    .await?;

    Ok(())
}
