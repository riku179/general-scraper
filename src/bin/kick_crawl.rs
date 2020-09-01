use chrono::Duration;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenv::dotenv;
use env_logger;
use lib::dispatcher::{DataStoreAdapter, Dispatcher};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok().unwrap();
    env_logger::init();

    let dispatcher = Dispatcher::new(DataStoreAdapter::new(Pool::new(ConnectionManager::new(
        env::var("DATABASE_URL")?,
    ))?));

    dispatcher.start(Duration::minutes(0)).await?;

    Ok(())
}
