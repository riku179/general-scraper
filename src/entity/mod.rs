use crate::crawler::SelectorTree;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub selectors: SelectorTree,
    pub last_accessed: DateTime<Utc>,
    pub last_accessed_urls: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Content {
    pub id: String,
    pub url: String,
    pub source_id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
