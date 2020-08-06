mod datastore;
mod job;

use crate::dispatcher::job::kick;
use crate::entity::{Content, Source};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use futures::future::join_all;

#[async_trait]
pub trait DataStore {
    // offset期間以上に更新されていない古いsourcesをすべて取得する
    async fn get_stale_sources(&self, offset: Duration) -> Result<Vec<Source>>;
    // jobの結果をDatastoreに保存する
    async fn commit_job_result(
        &self,
        source_id: i32,
        contents: Vec<Content>,
        accessed_urls: Vec<String>,
    ) -> Result<()>;
}

pub struct Dispatcher<D: DataStore> {
    data_store: D,
}

impl<D: DataStore> Dispatcher<D> {
    fn new(data_store: D) -> Self {
        Dispatcher { data_store }
    }

    async fn start(&self) -> Result<()> {
        let sources = self
            .data_store
            .get_stale_sources(Duration::hours(1))
            .await?;
        let mut jobs = vec![];
        for source in sources {
            let x = kick(source);
            jobs.push(x);
        }
        let results = join_all(jobs).await;

        Ok(())
    }
}
