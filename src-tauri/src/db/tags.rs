use sqlx::SqlitePool;
use uuid::Uuid;

use super::error::DbResult;
use super::models::Tag;

pub async fn get_or_create(pool: &SqlitePool, name: &str) -> DbResult<Tag> {
    let normalized = name.trim().to_lowercase();
    if normalized.is_empty() {
        return Err(super::error::DbError::Message("tag name cannot be empty".into()));
    }

    if let Some(tag) = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ?")
        .bind(&normalized)
        .fetch_optional(pool)
        .await?
    {
        return Ok(tag);
    }

    let id = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO tags (id, name) VALUES (?, ?)")
        .bind(&id)
        .bind(&normalized)
        .execute(pool)
        .await?;

    Ok(Tag {
        id,
        name: normalized,
    })
}

pub async fn list(pool: &SqlitePool) -> DbResult<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name ASC")
        .fetch_all(pool)
        .await?;
    Ok(tags)
}

pub async fn get_item_tags(pool: &SqlitePool, item_id: &str) -> DbResult<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(
        r#"
        SELECT t.*
        FROM tags t
        INNER JOIN item_tags it ON it.tag_id = t.id
        WHERE it.item_id = ?
        ORDER BY t.name ASC
        "#,
    )
    .bind(item_id)
    .fetch_all(pool)
    .await?;
    Ok(tags)
}

pub async fn set_item_tags(
    pool: &SqlitePool,
    item_id: &str,
    names: &[String],
) -> DbResult<Vec<Tag>> {
    sqlx::query("DELETE FROM item_tags WHERE item_id = ?")
        .bind(item_id)
        .execute(pool)
        .await?;

    let mut tags = Vec::new();
    for name in names {
        let tag = get_or_create(pool, name).await?;
        sqlx::query("INSERT OR IGNORE INTO item_tags (item_id, tag_id) VALUES (?, ?)")
            .bind(item_id)
            .bind(&tag.id)
            .execute(pool)
            .await?;
        tags.push(tag);
    }

    Ok(tags)
}
