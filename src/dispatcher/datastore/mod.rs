mod models;
mod schema;

use crate::crawler::SelectorTree;
use crate::dispatcher::DataStore;
use crate::entity::{Content, Source};
use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use diesel;
use diesel::prelude::*;
// use diesel::result::Error;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use futures::TryFutureExt;
use models::{ContentModel, SourceModel};
use tokio;

pub struct DataStoreAdapter {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DataStoreAdapter {
    fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        DataStoreAdapter { pool }
    }
}

#[async_trait]
impl DataStore for DataStoreAdapter {
    async fn get_stale_sources(&self, offset: Duration) -> Result<Vec<Source>> {
        use schema::sources::dsl::*;
        let pool = self.pool.clone();
        let results: Vec<SourceModel> = tokio::task::spawn_blocking(move || -> Result<_> {
            let con = pool.get()?;
            Ok(sources
                .filter(last_accessed.lt(Utc::now().naive_utc() - offset))
                .load::<SourceModel>(&con)?)
        })
        .await??;

        let (sources_results, errors): (Vec<Result<Source>>, Vec<Result<Source>>) = results
            .into_iter()
            .map(|model| {
                Ok(Source {
                    id: model.id,
                    name: model.name,
                    url: model.url,
                    selectors: SelectorTree::new(model.selectors)?,
                    last_accessed: DateTime::<Utc>::from_utc(model.last_accessed, Utc),
                    last_accessed_urls: model
                        .last_accessed_urls
                        .split(",")
                        .map(str::to_string)
                        .collect(),
                    created_at: DateTime::<Utc>::from_utc(model.created_at, Utc),
                })
            })
            .partition(Result::is_ok);
        if errors.len() != 0 {
            let errors: Vec<Error> = errors.into_iter().map(Result::unwrap_err).collect();
            for error in errors {
                println!("{:?}", error)
            }
        };
        sources_results.into_iter().collect()
    }

    async fn commit_job_result(
        &self,
        source_id: i32,
        contents_entities: Vec<Content>,
        accessed_urls: Vec<String>,
    ) -> Result<()> {
        let contents_models = contents_entities
            .into_iter()
            .map(|entity| ContentModel {
                id: entity.id,
                url: entity.url,
                source_id: entity.source_id,
                title: entity.title,
                body: entity.body,
            })
            .collect::<Vec<ContentModel>>();

        let pool = self.pool.clone();
        let insert_result = tokio::task::spawn_blocking(move || {
            let con = pool.get()?;

            con.transaction::<_, Error, _>(|| {
                {
                    use schema::contents::dsl::*;
                    diesel::insert_into(contents)
                        .values(&contents_models)
                        .execute(&*con)?;
                }
                {
                    use schema::sources::dsl::*;
                    let target_source = sources.filter(id.eq(source_id));
                    Ok(diesel::update(target_source)
                        .set(last_accessed_urls.eq(accessed_urls.join(",")))
                        .execute(&*con)?)
                }
            })
        })
        .await?;

        insert_result.map(|_| ())
    }
}
