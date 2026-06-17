use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::db::error::DbResult;
use crate::db::models::Platform;
use sqlx::SqlitePool;

pub async fn fetch_auto_transcript(pool: &SqlitePool, item_id: &str, url: &str, platform: Platform) -> DbResult<()> {
    if platform != Platform::Youtube {
        return Ok(());
    }

    let Some(video_id) = youtube_id(url) else {
        return Ok(());
    };

    let Some(text) = fetch_youtube_captions(&video_id).await else {
        return Ok(());
    };

    if text.trim().is_empty() {
        return Ok(());
    }

    let id = Uuid::new_v4().to_string();
    let created_at = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO transcripts (id, item_id, lang, text, source, created_at)
        VALUES (?, ?, 'en', ?, 'auto', ?)
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

fn youtube_id(url: &str) -> Option<String> {
    if let Some(id) = url.split("v=").nth(1).and_then(|rest| rest.split('&').next()) {
        return Some(id.to_string());
    }

    let re = Regex::new(r"youtu\.be/([\w-]{6,})").ok()?;
    re.captures(url)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

async fn fetch_youtube_captions(video_id: &str) -> Option<String> {
    let watch_url = format!("https://www.youtube.com/watch?v={video_id}");
    let body = reqwest::get(&watch_url).await.ok()?.text().await.ok()?;

    let re = Regex::new(r#"\"captionTracks\":\[(.*?)\]"#).ok()?;
    let captures = re.captures(&body)?;
    let block = captures.get(1)?.as_str();

    let url_re = Regex::new(r#"\"baseUrl\":\"(https:[^\"]+)\""#).ok()?;
    let caption_url = url_re
        .captures(block)?
        .get(1)?
        .as_str()
        .replace("\\u0026", "&");

    let xml = reqwest::get(&caption_url).await.ok()?.text().await.ok()?;
    let text_re = Regex::new(r"<text[^>]*>([^<]*)</text>").ok()?;
    let mut parts = Vec::new();

    for cap in text_re.captures_iter(&xml) {
        let segment = cap.get(1)?.as_str();
        let decoded = segment
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&lt;", "<")
            .replace("&gt;", ">");
        if !decoded.trim().is_empty() {
            parts.push(decoded);
        }
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_youtube_watch_url() {
        let id = youtube_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=share");
        assert_eq!(id.as_deref(), Some("dQw4w9WgXcQ"));
    }
}
