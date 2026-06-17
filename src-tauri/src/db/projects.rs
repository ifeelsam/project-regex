use sqlx::SqlitePool;
use uuid::Uuid;

use super::error::DbResult;
use super::models::{ItemStatus, Project, ProjectFormat, now_iso};
use super::items::set_status;

pub async fn create(
    pool: &SqlitePool,
    title: &str,
    brief: &str,
    format: ProjectFormat,
) -> DbResult<Project> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_iso();

    sqlx::query(
        r#"
        INSERT INTO projects (id, title, brief, format, status, created_at)
        VALUES (?, ?, ?, ?, 'active', ?)
        "#,
    )
    .bind(&id)
    .bind(title)
    .bind(brief)
    .bind(format)
    .bind(&created_at)
    .execute(pool)
    .await?;

    get(pool, &id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("project not found".into()))
}

pub async fn get(pool: &SqlitePool, id: &str) -> DbResult<Option<Project>> {
    let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(project)
}

pub async fn list(pool: &SqlitePool) -> DbResult<Vec<Project>> {
    let projects = sqlx::query_as::<_, Project>(
        "SELECT * FROM projects ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(projects)
}

pub async fn graduate_item(
    pool: &SqlitePool,
    item_id: &str,
    title: &str,
    brief: &str,
    format: ProjectFormat,
) -> DbResult<Project> {
    let project = create(pool, title, brief, format).await?;

    sqlx::query("UPDATE items SET project_id = ? WHERE id = ?")
        .bind(&project.id)
        .bind(item_id)
        .execute(pool)
        .await?;

    set_status(pool, item_id, ItemStatus::Producing).await?;

    Ok(project)
}
