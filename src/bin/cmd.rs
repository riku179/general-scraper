use ::lib::crawler;
use serde_json;
use std::env;
use std::error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let default_input_filename = "input.json".to_string();
    let default_output_filename = "output.json".to_string();
    let default_access_log_filename = "access_log.json".to_string();
    let default_skip_urls_filename = "skip_urls.json".to_string();

    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).unwrap_or(&default_input_filename);
    let output_filename = args.get(2).unwrap_or(&default_output_filename);
    let access_log_filename = args.get(3).unwrap_or(&default_access_log_filename);
    let skip_urls_filename = args.get(4).unwrap_or(&default_skip_urls_filename);

    let mut output_file = File::create(Path::new(output_filename))?;
    let mut access_log_file = File::create(Path::new(access_log_filename))?;

    let sitemap = fs::read_to_string(input_filename)?;
    let skip_urls_str = fs::read_to_string(skip_urls_filename)?;

    let selector = crawler::SelectorTree::new(sitemap)?;
    let skip_urls: Vec<String> = serde_json::from_str(&skip_urls_str)?;

    let executor = crawler::Executor::new(crawler::WebFetcher::new(), skip_urls);
    let (artifacts, access_log) = executor.crawl(&selector).await?;

    let formatted = crawler::format(artifacts);

    output_file.write_all(serde_json::to_string_pretty(&formatted)?.as_bytes())?;
    access_log_file.write_all(serde_json::to_string_pretty(&access_log)?.as_bytes())?;

    Ok(())
}
