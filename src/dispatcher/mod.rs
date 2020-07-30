mod job;

use crate::entity::{Content, Source};
use anyhow::Result;
use chrono::Duration;
use crate::dispatcher::job::kick;
use tokio;

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
        for source in sources {
            let x = tokio::spawn(kick(source));
        }

        Ok(())
    }
}
