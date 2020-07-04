mod selector_tree;

use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://formula1-data.com/article".to_string();
    let selector = ".mdlGrid__col12 a".to_string();

    for url in get_urls(&url, &selector).await? {
        println!("url: {}", url)
    }
    Ok(())
}

async fn get_urls(
    url: &String,
    selector: &String,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse(selector).unwrap();

    Ok(document
        .select(&selector)
        .map(|element| element.value().attr("href").unwrap().to_string())
        .collect())
}
