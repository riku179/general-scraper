mod datastore;
mod job;

use crate::dispatcher::job::kick;
use crate::entity::{Content, Source};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Duration;
use futures::future::join_all;
use log;

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
    // sourceの新規作成
    // async fn add_source(&self, Source) -> Result<Source>;
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

        for result in results {
            if let Ok((source_id, contents, accessed_urls)) = result {
                let result = self.data_store.commit_job_result(source_id, contents, accessed_urls).await;
                if let Err(err) = result {
                    log::error!("failed to store job result: {:?}", err)
                }
            } else {
                log::error!("failed to execute job: {:?}", result.unwrap_err())
            }
        }

        Ok(())
    }
}
