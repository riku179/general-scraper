use crate::selector_node::SelectorTree;
use serde_json;
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
    let args: Vec<String> = env::args().collect();
    let input_filename = args.get(1).unwrap_or(&default_input_filename);
    let output_filename = args.get(2).unwrap_or(&default_output_filename);

    let mut output_file = File::create(Path::new(output_filename))?;

    let sitemap = fs::read_to_string(input_filename)?;

    let selector = SelectorTree::new(sitemap)?;

    let executor = executor::Executor::new(executor::WebFetcher());
    let artifacts = executor.crawl(&selector).await?;

    let formatted = formatter::format(artifacts);

    output_file.write_all(serde_json::to_string_pretty(&formatted)?.as_bytes())?;

    Ok(())
}
