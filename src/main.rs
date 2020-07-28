use crate::selector_node::SelectorTree;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod executor;
mod formatter;
mod selector_node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let default_input_filename = "input.json".to_string();
    let default_output_filename = "output.json".to_string();
    let default_access_log_filename = "access_log.json".to_string();

    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).unwrap_or(&default_input_filename);
    let output_filename = args.get(2).unwrap_or(&default_output_filename);
    let access_log_filename = args.get(3).unwrap_or(&default_access_log_filename);

    let mut output_file = File::create(Path::new(output_filename))?;
    let access_log_path = Path::new(access_log_filename);

    let mut access_log_file = if access_log_path.exists() {
        fs::OpenOptions::new().write(true).open(access_log_path)?
    } else {
        File::create(access_log_path)?
    };

    let sitemap = fs::read_to_string(input_filename)?;
    let access_log_str = fs::read_to_string(access_log_filename)?;

    let selector = SelectorTree::new(sitemap)?;
    let access_log: HashMap<String, String> = serde_json::from_str(&access_log_str)?;

    let executor = executor::Executor::new(executor::WebFetcher::new(access_log));
    let (artifacts, access_log) = executor.crawl(&selector).await?;

    let formatted = formatter::format(artifacts);

    output_file.write_all(serde_json::to_string_pretty(&formatted)?.as_bytes())?;
    access_log_file.write_all(serde_json::to_string_pretty(&access_log)?.as_bytes())?;

    Ok(())
}
