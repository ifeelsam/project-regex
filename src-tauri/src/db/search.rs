use sqlx::SqlitePool;

use super::error::DbResult;
use super::items::get;
use super::models::SearchHit;
use super::tags::get_item_tags;

#[derive(sqlx::FromRow)]
struct RawHit {
    item_id: String,
    body: String,
}

pub async fn search(pool: &SqlitePool, query: &str, limit: i64) -> DbResult<Vec<SearchHit>> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let match_query = trimmed
        .split_whitespace()
        .map(|token| format!("\"{}\"*", token.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" ");

    let rows = sqlx::query_as::<_, RawHit>(
        r#"
        SELECT item_id, body
        FROM search_index
        WHERE search_index MATCH ?
        ORDER BY rowid
        LIMIT ?
        "#,
    )
    .bind(&match_query)
    .bind(limit * 4)
    .fetch_all(pool)
    .await?;

    let mut hits = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for row in rows {
        if !seen.insert(row.item_id.clone()) {
            continue;
        }

        if let Some(item) = get(pool, &row.item_id).await? {
            let tags = get_item_tags(pool, &item.id).await?;
            let snippet = excerpt(&row.body, trimmed, 120);
            hits.push(SearchHit {
                item,
                rank: 0.0,
                snippet,
                tags,
            });
        }

        if hits.len() as i64 >= limit {
            break;
        }
    }

    Ok(hits)
}

fn excerpt(body: &str, query: &str, max_len: usize) -> String {
    let lower_body = body.to_lowercase();
    let needle = query.to_lowercase();
    let start = lower_body.find(&needle).unwrap_or(0);
    let slice_start = start.saturating_sub(24);
    let slice = &body[slice_start..];
    let mut out = String::new();
    if slice_start > 0 {
        out.push('…');
    }
    out.push_str(&slice.chars().take(max_len).collect::<String>());
    if slice.chars().count() > max_len {
        out.push('…');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::items::capture;
    use crate::db::models::{CaptureItemInput, CapturedOn, Platform};
    use crate::db::pool::connect_memory;

    #[tokio::test]
    async fn search_finds_note_text() {
        let pool = connect_memory().await.unwrap();

        capture(
            &pool,
            CaptureItemInput {
                url: None,
                platform: Platform::Manual,
                title: Some("Hook ideas".into()),
                author: None,
                note: "The opening beat should feel like a cold open".into(),
                tags: vec!["video".into()],
                captured_on: CapturedOn::Desktop,
            },
        )
        .await
        .unwrap();

        let hits = search(&pool, "cold open", 10).await.unwrap();
        assert_eq!(hits.len(), 1);
        assert!(hits[0].snippet.to_lowercase().contains("cold"));
    }
}
