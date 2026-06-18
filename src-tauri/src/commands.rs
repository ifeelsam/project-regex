use tauri::State;

use crate::db;
use crate::db::error::DbError;
use crate::db::models::{
    CaptureItemInput, CaptureItemResult, CapturedOn, Item, ItemStatus, Platform, Project,
    ProjectFormat, SearchHit, Tag,
};
use crate::AppState;

#[tauri::command]
pub async fn ping() -> &'static str {
    "pong"
}

#[tauri::command]
pub async fn capture_item(
    state: State<'_, AppState>,
    input: CaptureItemInput,
) -> Result<CaptureItemResult, DbError> {
    db::items::capture(&state.pool, input).await
}

#[tauri::command]
pub async fn list_inbox_items(
    state: State<'_, AppState>,
    status: Option<ItemStatus>,
) -> Result<Vec<db::items::ItemWithTags>, DbError> {
    db::items::list_with_tags(&state.pool, status).await
}

#[tauri::command]
pub async fn list_develop_items(
    state: State<'_, AppState>,
) -> Result<Vec<db::items::ItemWithTags>, DbError> {
    db::ideas::list_develop(&state.pool).await
}

#[tauri::command]
pub async fn get_idea_detail(
    state: State<'_, AppState>,
    id: String,
) -> Result<db::ideas::IdeaDetail, DbError> {
    db::ideas::get_detail(&state.pool, &id).await
}

#[tauri::command]
pub async fn update_develop_note(
    state: State<'_, AppState>,
    id: String,
    develop_note: String,
) -> Result<Item, DbError> {
    db::ideas::update_develop_note(&state.pool, &id, &develop_note).await
}

#[tauri::command]
pub async fn add_item_reference(
    state: State<'_, AppState>,
    idea_id: String,
    reference_id: String,
) -> Result<(), DbError> {
    db::references::add(&state.pool, &idea_id, &reference_id).await
}

#[tauri::command]
pub async fn remove_item_reference(
    state: State<'_, AppState>,
    idea_id: String,
    reference_id: String,
) -> Result<(), DbError> {
    db::references::remove(&state.pool, &idea_id, &reference_id).await
}

#[tauri::command]
pub async fn list_items(
    state: State<'_, AppState>,
    status: Option<ItemStatus>,
) -> Result<Vec<Item>, DbError> {
    db::items::list(&state.pool, status).await
}

#[tauri::command]
pub async fn get_item(state: State<'_, AppState>, id: String) -> Result<Option<Item>, DbError> {
    db::items::get(&state.pool, &id).await
}

#[tauri::command]
pub async fn update_item_note(
    state: State<'_, AppState>,
    id: String,
    note: String,
) -> Result<Item, DbError> {
    db::items::update_note(&state.pool, &id, &note).await
}

#[tauri::command]
pub async fn update_item_status(
    state: State<'_, AppState>,
    id: String,
    status: ItemStatus,
) -> Result<Item, DbError> {
    db::items::set_status(&state.pool, &id, status).await
}

#[tauri::command]
pub async fn delete_item(state: State<'_, AppState>, id: String) -> Result<(), DbError> {
    db::items::delete_item(&state.pool, &id).await
}

#[tauri::command]
pub async fn list_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, DbError> {
    db::tags::list(&state.pool).await
}

#[tauri::command]
pub async fn set_item_tags(
    state: State<'_, AppState>,
    item_id: String,
    tags: Vec<String>,
) -> Result<Vec<Tag>, DbError> {
    db::tags::set_item_tags(&state.pool, &item_id, &tags).await
}

#[tauri::command]
pub async fn search_items(
    state: State<'_, AppState>,
    query: String,
    limit: Option<i64>,
) -> Result<Vec<SearchHit>, DbError> {
    db::search::search(&state.pool, &query, limit.unwrap_or(50)).await
}

#[tauri::command]
pub async fn list_project_summaries(
    state: State<'_, AppState>,
) -> Result<Vec<db::projects::ProjectSummary>, DbError> {
    db::projects::list_summaries(&state.pool).await
}

#[tauri::command]
pub async fn get_project_detail(
    state: State<'_, AppState>,
    id: String,
) -> Result<db::projects::ProjectDetail, DbError> {
    db::projects::get_detail(&state.pool, &id).await
}

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    title: String,
    brief: String,
    format: ProjectFormat,
) -> Result<Project, DbError> {
    db::projects::create(&state.pool, &title, &brief, format, None).await
}

#[tauri::command]
pub async fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, DbError> {
    db::projects::list(&state.pool).await
}

#[tauri::command]
pub async fn graduate_item(
    state: State<'_, AppState>,
    item_id: String,
    title: String,
    brief: String,
    format: ProjectFormat,
) -> Result<Project, DbError> {
    db::projects::graduate_item(&state.pool, &item_id, &title, &brief, format).await
}

#[tauri::command]
pub fn detect_platform(url: String) -> Platform {
    db::items::detect_platform(&url)
}

#[tauri::command]
pub fn default_captured_on() -> CapturedOn {
  #[cfg(any(target_os = "android", target_os = "ios"))]
  {
    CapturedOn::Mobile
  }
  #[cfg(not(any(target_os = "android", target_os = "ios")))]
  {
    CapturedOn::Desktop
  }
}
