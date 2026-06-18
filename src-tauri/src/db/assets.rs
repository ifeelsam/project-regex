use sqlx::SqlitePool;
use uuid::Uuid;

use super::breakdowns::{Asset, AssetType};
use super::error::DbResult;

fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    breakdown_id: &str,
    project_id: Option<&str>,
    item_id: Option<&str>,
    asset_type: AssetType,
    path: &str,
    start_ms: Option<i64>,
    end_ms: Option<i64>,
    label: &str,
    meta: &str,
) -> DbResult<Asset> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_iso();

    sqlx::query(
        r#"
        INSERT INTO assets (
            id, breakdown_id, project_id, item_id, type, path,
            start_ms, end_ms, label, meta, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(breakdown_id)
    .bind(project_id)
    .bind(item_id)
    .bind(asset_type)
    .bind(path)
    .bind(start_ms)
    .bind(end_ms)
    .bind(label)
    .bind(meta)
    .bind(&created_at)
    .execute(pool)
    .await?;

    sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
}

pub async fn list_for_project(pool: &SqlitePool, project_id: &str) -> DbResult<Vec<Asset>> {
    let rows = sqlx::query_as::<_, Asset>(
        "SELECT * FROM assets WHERE project_id = ? ORDER BY created_at DESC",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(dead_code)]
pub async fn list_for_breakdown(pool: &SqlitePool, breakdown_id: &str) -> DbResult<Vec<Asset>> {
    let rows = sqlx::query_as::<_, Asset>(
        "SELECT * FROM assets WHERE breakdown_id = ? ORDER BY created_at ASC",
    )
    .bind(breakdown_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
