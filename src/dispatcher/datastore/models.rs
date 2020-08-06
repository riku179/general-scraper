use crate::dispatcher::datastore::schema::contents;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct SourceModel {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub selectors: String,
    pub last_accessed: NaiveDateTime,
    pub last_accessed_urls: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "contents"]
pub struct ContentModel {
    pub id: String,
    pub url: String,
    pub source_id: i32,
    pub title: String,
    pub body: String,
}
