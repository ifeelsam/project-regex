use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Instagram,
    X,
    Tiktok,
    Youtube,
    Web,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Inbox,
    Brewing,
    Ready,
    Producing,
    Shipped,
    Archived,
}

impl ItemStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inbox => "inbox",
            Self::Brewing => "brewing",
            Self::Ready => "ready",
            Self::Producing => "producing",
            Self::Shipped => "shipped",
            Self::Archived => "archived",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CapturedOn {
    Desktop,
    Mobile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProjectFormat {
    Video,
    Article,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub id: String,
    pub url: Option<String>,
    pub platform: Platform,
    pub title: String,
    pub author: String,
    pub note: String,
    pub thumbnail_path: Option<String>,
    pub status: ItemStatus,
    pub project_id: Option<String>,
    pub captured_at: String,
    pub captured_on: CapturedOn,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub brief: String,
    pub format: ProjectFormat,
    pub status: String,
    pub created_at: String,
    pub shipped_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureItemInput {
    pub url: Option<String>,
    pub platform: Platform,
    pub title: Option<String>,
    pub author: Option<String>,
    pub note: String,
    pub tags: Vec<String>,
    pub captured_on: CapturedOn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureItemResult {
    pub item: Item,
    pub created: bool,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit {
    pub item: Item,
    pub rank: f64,
    pub snippet: String,
    pub tags: Vec<Tag>,
}

pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}
