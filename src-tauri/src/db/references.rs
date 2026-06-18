use sqlx::SqlitePool;

use super::error::{DbError, DbResult};
use super::items::{get, ItemWithTags};
use super::tags::get_item_tags;

pub async fn list_for_idea(pool: &SqlitePool, idea_id: &str) -> DbResult<Vec<ItemWithTags>> {
    let rows = sqlx::query_as::<_, super::models::Item>(
        r#"
        SELECT i.*
        FROM items i
        INNER JOIN item_references ir ON ir.reference_id = i.id
        WHERE ir.idea_id = ?
        ORDER BY i.captured_at DESC
        "#,
    )
    .bind(idea_id)
    .fetch_all(pool)
    .await?;

    let mut references = Vec::with_capacity(rows.len());
    for item in rows {
        let tags = get_item_tags(pool, &item.id).await?;
        references.push(ItemWithTags { item, tags });
    }

    Ok(references)
}

pub async fn add(pool: &SqlitePool, idea_id: &str, reference_id: &str) -> DbResult<()> {
    if idea_id == reference_id {
        return Err(DbError::Message(
            "an idea cannot reference itself".into(),
        ));
    }

    get(pool, idea_id)
        .await?
        .ok_or_else(|| DbError::Message("idea not found".into()))?;

    get(pool, reference_id)
        .await?
        .ok_or_else(|| DbError::Message("reference not found".into()))?;

    sqlx::query(
        "INSERT OR IGNORE INTO item_references (idea_id, reference_id) VALUES (?, ?)",
    )
    .bind(idea_id)
    .bind(reference_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn remove(pool: &SqlitePool, idea_id: &str, reference_id: &str) -> DbResult<()> {
    sqlx::query("DELETE FROM item_references WHERE idea_id = ? AND reference_id = ?")
        .bind(idea_id)
        .bind(reference_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::items::capture;
    use crate::db::models::{CaptureItemInput, CapturedOn, Platform};
    use crate::db::pool::connect_memory;

    #[tokio::test]
    async fn attaches_and_lists_references() {
        let pool = connect_memory().await.unwrap();

        let idea = capture(
            &pool,
            CaptureItemInput {
                url: None,
                platform: Platform::Manual,
                title: None,
                author: None,
                note: "main idea".into(),
                tags: vec![],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        let reference = capture(
            &pool,
            CaptureItemInput {
                url: Some("https://example.com/ref".into()),
                platform: Platform::Web,
                title: Some("Reference".into()),
                author: None,
                note: "supporting spark".into(),
                tags: vec![],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        add(&pool, &idea.item.id, &reference.item.id)
            .await
            .unwrap();

        let refs = list_for_idea(&pool, &idea.item.id).await.unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].item.id, reference.item.id);
    }
}
