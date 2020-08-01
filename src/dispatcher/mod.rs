mod job;
mod datastore;

use crate::entity::{Content, Source};
use anyhow::Result;
use chrono::Duration;
use crate::dispatcher::job::kick;
use futures::future::join_all;

pub trait DataStore {
    // offset期間以上に更新されていない古いsourcesをすべて取得する
    fn get_stale_sources(&self, offset: Duration) -> Result<Vec<Source>>;
    // jobの結果をDatastoreに保存する
    fn commit_job_result(&self, contents: Vec<Content>, accessed_urls: Vec<String>) -> Result<()>;
}

pub struct Dispatcher<D: DataStore> {
    data_store: D,
}

impl<D: DataStore> Dispatcher<D> {
    fn new(data_store: D) -> Self {
        Dispatcher { data_store }
    }

    async fn start(&self) -> Result<()> {
        let sources = self.data_store.get_stale_sources(Duration::hours(1))?;
        let mut jobs = vec![];
        for source in sources {
            let x = kick(source);
            jobs.push(x);
        }
        let results = join_all(jobs).await;

        Ok(())
    }
}
