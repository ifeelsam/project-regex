use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::error::DbResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum BreakdownStatus {
    Queued,
    Running,
    Done,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Frame,
    Clip,
    Audio,
    Transcript,
    Structure,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Breakdown {
    pub id: String,
    pub item_id: String,
    pub project_id: Option<String>,
    pub status: BreakdownStatus,
    pub error: Option<String>,
    pub created_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: String,
    pub breakdown_id: Option<String>,
    pub project_id: Option<String>,
    pub item_id: Option<String>,
    pub r#type: AssetType,
    pub path: String,
    pub start_ms: Option<i64>,
    pub end_ms: Option<i64>,
    pub label: String,
    pub meta: String,
    pub created_at: String,
}

fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

pub async fn create(
    pool: &sqlx::SqlitePool,
    item_id: &str,
    project_id: Option<&str>,
) -> DbResult<Breakdown> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_iso();

    sqlx::query(
        r#"
        INSERT INTO breakdowns (id, item_id, project_id, status, created_at)
        VALUES (?, ?, ?, 'queued', ?)
        "#,
    )
    .bind(&id)
    .bind(item_id)
    .bind(project_id)
    .bind(&created_at)
    .execute(pool)
    .await?;

    get(pool, &id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("breakdown not found".into()))
}

pub async fn get(pool: &sqlx::SqlitePool, id: &str) -> DbResult<Option<Breakdown>> {
    let row = sqlx::query_as::<_, Breakdown>("SELECT * FROM breakdowns WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

pub async fn list_for_project(pool: &sqlx::SqlitePool, project_id: &str) -> DbResult<Vec<Breakdown>> {
    let rows = sqlx::query_as::<_, Breakdown>(
        "SELECT * FROM breakdowns WHERE project_id = ? ORDER BY created_at DESC",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(dead_code)]
pub async fn latest_for_item(pool: &sqlx::SqlitePool, item_id: &str) -> DbResult<Option<Breakdown>> {
    let row = sqlx::query_as::<_, Breakdown>(
        "SELECT * FROM breakdowns WHERE item_id = ? ORDER BY created_at DESC LIMIT 1",
    )
    .bind(item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn set_status(
    pool: &sqlx::SqlitePool,
    id: &str,
    status: BreakdownStatus,
    error: Option<&str>,
) -> DbResult<Breakdown> {
    let finished_at = if matches!(status, BreakdownStatus::Done | BreakdownStatus::Failed) {
        Some(now_iso())
    } else {
        None
    };

    sqlx::query(
        r#"
        UPDATE breakdowns
        SET status = ?, error = ?, finished_at = COALESCE(?, finished_at)
        WHERE id = ?
        "#,
    )
    .bind(status)
    .bind(error)
    .bind(finished_at)
    .bind(id)
    .execute(pool)
    .await?;

    get(pool, id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("breakdown not found".into()))
}
