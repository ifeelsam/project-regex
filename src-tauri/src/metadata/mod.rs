use scraper::{Html, Selector};
use serde::Serialize;

use crate::db::models::Platform;

#[derive(Debug, Clone, Default, Serialize)]
pub struct LinkMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub thumbnail_url: Option<String>,
}

pub async fn fetch(url: &str, platform: Platform) -> LinkMetadata {
    match platform {
        Platform::Youtube => fetch_youtube_oembed(url).await,
        Platform::X | Platform::Instagram | Platform::Tiktok => fetch_oembed(url).await,
        _ => fetch_open_graph(url).await,
    }
}

async fn fetch_youtube_oembed(url: &str) -> LinkMetadata {
    let endpoint = format!(
        "https://www.youtube.com/oembed?url={}&format=json",
        urlencoding::encode(url)
    );

    if let Ok(response) = reqwest::get(&endpoint).await {
        if let Ok(json) = response.json::<serde_json::Value>().await {
            return LinkMetadata {
                title: json.get("title").and_then(|v| v.as_str()).map(str::to_string),
                author: json
                    .get("author_name")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                thumbnail_url: json
                    .get("thumbnail_url")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
            };
        }
    }

    fetch_open_graph(url).await
}

async fn fetch_oembed(url: &str) -> LinkMetadata {
    let endpoint = format!(
        "https://publish.twitter.com/oembed?url={}",
        urlencoding::encode(url)
    );

    if let Ok(response) = reqwest::get(&endpoint).await {
        if response.status().is_success() {
            if let Ok(json) = response.json::<serde_json::Value>().await {
                return LinkMetadata {
                    title: json.get("title").and_then(|v| v.as_str()).map(str::to_string),
                    author: json
                        .get("author_name")
                        .and_then(|v| v.as_str())
                        .map(str::to_string),
                    thumbnail_url: None,
                };
            }
        }
    }

    fetch_open_graph(url).await
}

async fn fetch_open_graph(url: &str) -> LinkMetadata {
    let Ok(response) = reqwest::get(url).await else {
        return LinkMetadata::default();
    };

    let Ok(body) = response.text().await else {
        return LinkMetadata::default();
    };

    let document = Html::parse_document(&body);
    let title = meta_content(&document, "og:title")
        .or_else(|| meta_content(&document, "twitter:title"))
        .or_else(|| document.select(&Selector::parse("title").unwrap()).next().map(|n| {
            n.text().collect::<String>()
        }));

    let author = meta_content(&document, "og:site_name")
        .or_else(|| meta_content(&document, "author"))
        .or_else(|| meta_content(&document, "article:author"));

    let thumbnail_url = meta_content(&document, "og:image")
        .or_else(|| meta_content(&document, "twitter:image"));

    LinkMetadata {
        title,
        author,
        thumbnail_url,
    }
}

fn meta_content(document: &Html, property: &str) -> Option<String> {
    let selector = Selector::parse(&format!("meta[property=\"{property}\"], meta[name=\"{property}\"]"))
        .ok()?;
    document
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
}
