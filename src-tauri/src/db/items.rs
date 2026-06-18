use sqlx::SqlitePool;
use uuid::Uuid;

use super::error::DbResult;
use super::models::{CaptureItemInput, CaptureItemResult, Item, ItemStatus, Platform, Tag, now_iso};
use super::status::assert_transition;
use super::tags::{get_item_tags, set_item_tags};

#[derive(serde::Serialize)]
pub struct ItemWithTags {
    pub item: Item,
    pub tags: Vec<Tag>,
}

pub async fn find_by_url(pool: &SqlitePool, url: &str) -> DbResult<Option<Item>> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE url = ?")
        .bind(url)
        .fetch_optional(pool)
        .await?;
    Ok(item)
}

pub async fn get(pool: &SqlitePool, id: &str) -> DbResult<Option<Item>> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(item)
}

pub async fn list_with_tags(
    pool: &SqlitePool,
    status: Option<ItemStatus>,
) -> DbResult<Vec<ItemWithTags>> {
    let items = list(pool, status).await?;
    let mut rows = Vec::with_capacity(items.len());
    for item in items {
        let tags = get_item_tags(pool, &item.id).await?;
        rows.push(ItemWithTags { item, tags });
    }
    Ok(rows)
}

pub async fn list(pool: &SqlitePool, status: Option<ItemStatus>) -> DbResult<Vec<Item>> {
    let items = if let Some(status) = status {
        sqlx::query_as::<_, Item>(
            "SELECT * FROM items WHERE status = ? ORDER BY captured_at DESC",
        )
        .bind(status)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, Item>("SELECT * FROM items ORDER BY captured_at DESC")
            .fetch_all(pool)
            .await?
    };
    Ok(items)
}

pub async fn capture(pool: &SqlitePool, input: CaptureItemInput) -> DbResult<CaptureItemResult> {
    if let Some(ref url) = input.url {
        if !url.is_empty() {
            if let Some(existing) = find_by_url(pool, url).await? {
                let tags = get_item_tags(pool, &existing.id).await?;
                return Ok(CaptureItemResult {
                    item: existing,
                    created: false,
                    tags,
                });
            }
        }
    }

    let id = Uuid::new_v4().to_string();
    let captured_at = now_iso();

    sqlx::query(
        r#"
        INSERT INTO items (id, url, platform, title, author, note, develop_note, status, captured_at, captured_on)
        VALUES (?, ?, ?, ?, ?, ?, '', 'inbox', ?, ?)
        "#,
    )
    .bind(&id)
    .bind(&input.url)
    .bind(input.platform)
    .bind(input.title.unwrap_or_default())
    .bind(input.author.unwrap_or_default())
    .bind(&input.note)
    .bind(&captured_at)
    .bind(input.captured_on)
    .execute(pool)
    .await?;

    let tags = set_item_tags(pool, &id, &input.tags).await?;
    let item = get(pool, &id).await?.expect("item just inserted");

    Ok(CaptureItemResult {
        item,
        created: true,
        tags,
    })
}

pub async fn update_note(pool: &SqlitePool, id: &str, note: &str) -> DbResult<Item> {
    sqlx::query("UPDATE items SET note = ? WHERE id = ?")
        .bind(note)
        .bind(id)
        .execute(pool)
        .await?;

    get(pool, id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("item not found".into()))
}

pub async fn update_metadata(
    pool: &SqlitePool,
    id: &str,
    title: Option<&str>,
    author: Option<&str>,
    thumbnail_path: Option<&str>,
) -> DbResult<Item> {
    if let Some(title) = title {
        sqlx::query("UPDATE items SET title = ? WHERE id = ?")
            .bind(title)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(author) = author {
        sqlx::query("UPDATE items SET author = ? WHERE id = ?")
            .bind(author)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(thumbnail_path) = thumbnail_path {
        sqlx::query("UPDATE items SET thumbnail_path = ? WHERE id = ?")
            .bind(thumbnail_path)
            .bind(id)
            .execute(pool)
            .await?;
    }

    get(pool, id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("item not found".into()))
}

pub async fn set_status(pool: &SqlitePool, id: &str, status: ItemStatus) -> DbResult<Item> {
    let current = get(pool, id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("item not found".into()))?;

    assert_transition(current.status, status)?;

    sqlx::query("UPDATE items SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;

    get(pool, id)
        .await?
        .ok_or_else(|| super::error::DbError::Message("item not found".into()))
}

pub async fn delete_item(pool: &SqlitePool, id: &str) -> DbResult<()> {
    sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub fn detect_platform(url: &str) -> Platform {
    let lower = url.to_lowercase();
    if lower.contains("instagram.com") {
        Platform::Instagram
    } else if lower.contains("tiktok.com") {
        Platform::Tiktok
    } else if lower.contains("youtube.com") || lower.contains("youtu.be") {
        Platform::Youtube
    } else if lower.contains("twitter.com") || lower.contains("x.com") {
        Platform::X
    } else {
        Platform::Web
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::models::CapturedOn;
    use crate::db::pool::connect_memory;

    #[tokio::test]
    async fn dedupes_on_url() {
        let pool = connect_memory().await.unwrap();
        let input = CaptureItemInput {
            url: Some("https://example.com/post".into()),
            platform: Platform::Web,
            title: None,
            author: None,
            note: "first save".into(),
            tags: vec![],
            captured_on: CapturedOn::Desktop,
        };

        let first = capture(&pool, input.clone()).await.unwrap();
        assert!(first.created);

        let second = capture(&pool, input).await.unwrap();
        assert!(!second.created);
        assert_eq!(first.item.id, second.item.id);
    }

    #[tokio::test]
    async fn status_transition_rejects_invalid() {
        let pool = connect_memory().await.unwrap();
        let result = capture(
            &pool,
            CaptureItemInput {
                url: None,
                platform: Platform::Manual,
                title: None,
                author: None,
                note: "thought".into(),
                tags: vec![],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        let err = set_status(&pool, &result.item.id, ItemStatus::Producing)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("invalid status transition"));
    }
}
