use sqlx::SqlitePool;

use super::error::DbResult;
use super::models::now_iso;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct TranscriptRow {
    pub text: String,
}

pub async fn latest_for_item(pool: &SqlitePool, item_id: &str) -> DbResult<Option<TranscriptRow>> {
    let row = sqlx::query_as::<_, TranscriptRow>(
        "SELECT text FROM transcripts WHERE item_id = ? ORDER BY created_at DESC LIMIT 1",
    )
    .bind(item_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert_whisper(pool: &SqlitePool, item_id: &str, text: &str) -> DbResult<()> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_iso();

    sqlx::query(
        r#"
        INSERT INTO transcripts (id, item_id, lang, text, source, created_at)
        VALUES (?, ?, 'en', ?, 'whisper', ?)
        "#,
    )
    .bind(id)
    .bind(item_id)
    .bind(text)
    .bind(created_at)
    .execute(pool)
    .await?;

    Ok(())
}
