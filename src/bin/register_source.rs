use ::lib::crawler;
use ::lib::dispatcher::DataStoreAdapter;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenv::dotenv;
use env_logger;
use lib::dispatcher::DataStore;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok().unwrap();
    env_logger::init();

    let sitemap = fs::read_to_string("input.json")?;

    let selector = crawler::SelectorTree::new(sitemap).unwrap();

    let data_store = DataStoreAdapter::new(Pool::new(ConnectionManager::new(env::var(
        "DATABASE_URL",
    )?))?);

    data_store.add_source(selector).await?;

    Ok(())
}
