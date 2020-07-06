use crate::selector_node::{SelectorNode, SelectorType};
use reqwest;
use scraper::{Html, Selector};
use std::future::Future;
use std::pin::Pin;

pub async fn execute(
    nodes: &Vec<SelectorNode>,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get(&url).await?.text().await?;
    let doc = Html::parse_document(&body);

    track_nodes(nodes, doc).await
}

async fn track_nodes(
    nodes: &Vec<SelectorNode>,
    doc: Html,
) -> Result<(), Box<dyn std::error::Error>> {
    for node in nodes {
        match node.selector_type {
            SelectorType::Link => track_link_node(node.clone(), doc.clone()).await,
            SelectorType::Text => track_text_node(&node, &doc).await?,
            _ => panic!(),
        }
    }

    Ok(())
}

fn track_link_node(node: SelectorNode, doc: Html) -> Pin<Box<dyn Future<Output = ()>>> {
    let selector = Selector::parse(&node.selector).unwrap();
    let urls: Vec<String> = doc
        .select(&selector)
        .map(|element| element.value().attr("href").unwrap().to_string())
        .collect();

    Box::pin(async move {
        for url in urls {
            execute(&node.children, url).await;
        }
    })
}
async fn track_text_node(
    node: &SelectorNode,
    doc: &Html,
) -> Result<(), Box<dyn std::error::Error>> {
    let selector = Selector::parse(&node.selector).unwrap();

    let text: String = doc
        .select(&selector)
        .map(|element| {
            element
                .text()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", text);

    Ok(())
}
