use sqlx::SqlitePool;

use super::error::{DbError, DbResult};
use super::items::{get, list_with_tags, ItemWithTags};
use super::models::{Item, ItemStatus, Tag};
use super::references;
use super::tags::get_item_tags;

#[derive(serde::Serialize)]
pub struct IdeaDetail {
    pub item: Item,
    pub tags: Vec<Tag>,
    pub references: Vec<ItemWithTags>,
}

pub async fn list_develop(pool: &SqlitePool) -> DbResult<Vec<ItemWithTags>> {
    let brewing = list_with_tags(pool, Some(ItemStatus::Brewing)).await?;
    let ready = list_with_tags(pool, Some(ItemStatus::Ready)).await?;
    Ok(brewing.into_iter().chain(ready).collect())
}

pub async fn get_detail(pool: &SqlitePool, id: &str) -> DbResult<IdeaDetail> {
    let item = get(pool, id)
        .await?
        .ok_or_else(|| DbError::Message("idea not found".into()))?;

    let tags = get_item_tags(pool, id).await?;
    let references = references::list_for_idea(pool, id).await?;

    Ok(IdeaDetail {
        item,
        tags,
        references,
    })
}

pub async fn update_develop_note(pool: &SqlitePool, id: &str, develop_note: &str) -> DbResult<Item> {
    sqlx::query("UPDATE items SET develop_note = ? WHERE id = ?")
        .bind(develop_note)
        .bind(id)
        .execute(pool)
        .await?;

    get(pool, id)
        .await?
        .ok_or_else(|| DbError::Message("idea not found".into()))
}
