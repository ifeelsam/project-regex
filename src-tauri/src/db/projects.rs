use sqlx::SqlitePool;
use uuid::Uuid;

use super::error::{DbError, DbResult};
use super::items::{get as get_item, set_status, ItemWithTags};
use super::models::{ItemStatus, Project, ProjectFormat, now_iso};
use super::references as item_references;
use super::tags::get_item_tags;

#[derive(serde::Serialize)]
pub struct ProjectSummary {
    pub project: Project,
    pub primary_item: ItemWithTags,
    pub reference_count: i64,
}

#[derive(serde::Serialize)]
pub struct ProjectDetail {
    pub project: Project,
    pub primary_item: ItemWithTags,
    pub references: Vec<ItemWithTags>,
}

pub async fn create(
    pool: &SqlitePool,
    title: &str,
    brief: &str,
    format: ProjectFormat,
    primary_item_id: Option<&str>,
) -> DbResult<Project> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_iso();

    sqlx::query(
        r#"
        INSERT INTO projects (id, title, brief, format, status, created_at, primary_item_id)
        VALUES (?, ?, ?, ?, 'active', ?, ?)
        "#,
    )
    .bind(&id)
    .bind(title)
    .bind(brief)
    .bind(format)
    .bind(&created_at)
    .bind(primary_item_id)
    .execute(pool)
    .await?;

    get(pool, &id)
        .await?
        .ok_or_else(|| DbError::Message("project not found".into()))
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

async fn link_item(pool: &SqlitePool, item_id: &str, project_id: &str) -> DbResult<()> {
    sqlx::query("UPDATE items SET project_id = ? WHERE id = ?")
        .bind(project_id)
        .bind(item_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn item_with_tags(pool: &SqlitePool, item_id: &str) -> DbResult<ItemWithTags> {
    let item = get_item(pool, item_id)
        .await?
        .ok_or_else(|| DbError::Message("item not found".into()))?;
    let tags = get_item_tags(pool, item_id).await?;
    Ok(ItemWithTags { item, tags })
}

pub async fn graduate_item(
    pool: &SqlitePool,
    item_id: &str,
    title: &str,
    brief: &str,
    format: ProjectFormat,
) -> DbResult<Project> {
    let item = get_item(pool, item_id)
        .await?
        .ok_or_else(|| DbError::Message("idea not found".into()))?;

    if item.status != ItemStatus::Ready {
        return Err(DbError::Message(
            "only ready ideas can be turned into projects".into(),
        ));
    }

    let references = item_references::list_for_idea(pool, item_id).await?;
    let project = create(pool, title, brief, format, Some(item_id)).await?;

    link_item(pool, item_id, &project.id).await?;
    set_status(pool, item_id, ItemStatus::Producing).await?;

    for reference in &references {
        link_item(pool, &reference.item.id, &project.id).await?;
    }

    Ok(project)
}

pub async fn list_summaries(pool: &SqlitePool) -> DbResult<Vec<ProjectSummary>> {
    let projects = list(pool).await?;
    let mut summaries = Vec::with_capacity(projects.len());

    for project in projects {
        let Some(primary_id) = project.primary_item_id.as_deref() else {
            continue;
        };

        let primary_item = item_with_tags(pool, primary_id).await?;
        let reference_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM items
            WHERE project_id = ? AND id != ?
            "#,
        )
        .bind(&project.id)
        .bind(primary_id)
        .fetch_one(pool)
        .await?;

        summaries.push(ProjectSummary {
            project,
            primary_item,
            reference_count,
        });
    }

    Ok(summaries)
}

pub async fn get_detail(pool: &SqlitePool, id: &str) -> DbResult<ProjectDetail> {
    let project = get(pool, id)
        .await?
        .ok_or_else(|| DbError::Message("project not found".into()))?;

    let primary_id = project
        .primary_item_id
        .as_deref()
        .ok_or_else(|| DbError::Message("project has no linked idea".into()))?;

    let primary_item = item_with_tags(pool, primary_id).await?;

    let reference_rows = sqlx::query_as::<_, super::models::Item>(
        r#"
        SELECT * FROM items
        WHERE project_id = ? AND id != ?
        ORDER BY captured_at DESC
        "#,
    )
    .bind(&project.id)
    .bind(primary_id)
    .fetch_all(pool)
    .await?;

    let mut references = Vec::with_capacity(reference_rows.len());
    for item in reference_rows {
        let tags = get_item_tags(pool, &item.id).await?;
        references.push(ItemWithTags { item, tags });
    }

    Ok(ProjectDetail {
        project,
        primary_item,
        references,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::items::capture;
    use crate::db::models::{CaptureItemInput, CapturedOn, Platform};
    use crate::db::pool::connect_memory;
    use crate::db::references::add;

    #[tokio::test]
    async fn graduates_ready_idea_with_references() {
        let pool = connect_memory().await.unwrap();

        let idea = capture(
            &pool,
            CaptureItemInput {
                url: None,
                platform: Platform::Manual,
                title: Some("Video essay".into()),
                author: None,
                note: "main spark".into(),
                tags: vec![],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        set_status(&pool, &idea.item.id, ItemStatus::Brewing)
            .await
            .unwrap();
        set_status(&pool, &idea.item.id, ItemStatus::Ready)
            .await
            .unwrap();

        let reference = capture(
            &pool,
            CaptureItemInput {
                url: Some("https://example.com/ref".into()),
                platform: Platform::Web,
                title: Some("Reference".into()),
                author: None,
                note: "supporting".into(),
                tags: vec![],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        add(&pool, &idea.item.id, &reference.item.id)
            .await
            .unwrap();

        let project = graduate_item(
            &pool,
            &idea.item.id,
            "My video essay",
            "A piece about creative process",
            ProjectFormat::Video,
        )
        .await
        .unwrap();

        assert_eq!(project.primary_item_id.as_deref(), Some(idea.item.id.as_str()));

        let detail = get_detail(&pool, &project.id).await.unwrap();
        assert_eq!(detail.primary_item.item.status, ItemStatus::Producing);
        assert_eq!(detail.references.len(), 1);
        assert_eq!(detail.references[0].item.project_id.as_deref(), Some(project.id.as_str()));
    }

    #[tokio::test]
    async fn rejects_graduation_when_not_ready() {
        let pool = connect_memory().await.unwrap();
        let idea = capture(
            &pool,
            CaptureItemInput {
                url: None,
                platform: Platform::Manual,
                title: None,
                author: None,
                note: "still brewing".into(),
                tags: vec![],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        set_status(&pool, &idea.item.id, ItemStatus::Brewing)
            .await
            .unwrap();

        let err = graduate_item(
            &pool,
            &idea.item.id,
            "Nope",
            "",
            ProjectFormat::Other,
        )
        .await
        .unwrap_err();

        assert!(err.to_string().contains("only ready ideas"));
    }
}
