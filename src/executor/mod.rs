use crate::selector_node::{SelectorNode, SelectorType};
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize)]
pub struct Artifact {
    tag: String,
    data: String,
    children: Vec<Artifact>,
}

pub async fn execute(
    nodes: &Vec<SelectorNode>,
    url: &String,
) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    let doc = Html::parse_document(&body);

    track_nodes(nodes, url, doc).await
}

async fn track_nodes(
    nodes: &Vec<SelectorNode>,
    url: &String,
    doc: Html,
) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
    let mut artifacts: Vec<Artifact> = vec![];
    for node in nodes {
        let artifact = match node.selector_type {
            SelectorType::Link => {
                let artifacts = track_link_node(&node, &doc).await?;
                Artifact {
                    tag: node.id.clone(),
                    data: url.clone(),
                    children: artifacts,
                }
            }
            SelectorType::Text => {
                let data = track_text_node(&node, &doc).await?;
                Artifact {
                    tag: node.id.clone(),
                    data,
                    children: vec![],
                }
            }
            _ => panic!(),
        };
        artifacts.push(artifact);
    }

    Ok(artifacts)
}

async fn track_link_node(
    node: &SelectorNode,
    doc: &Html,
) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
    let selector = Selector::parse(&node.selector).unwrap();
    let urls: Vec<String> = doc
        .select(&selector)
        .map(|element| element.value().attr("href").unwrap().to_string())
        .collect();

    // helper for recursive async function. ref here: https://doc.rust-lang.org/error-index.html#E0733
    fn execute_urls(
        node: SelectorNode,
        urls: Vec<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Artifact>, Box<dyn std::error::Error>>>>> {
        Box::pin(async move {
            let mut artifacts: Vec<Artifact> = vec![];
            for url in urls {
                let children = execute(&node.children, &url).await?;
                artifacts.push(Artifact {
                    tag: node.id.clone(),
                    data: url,
                    children,
                });
            }
            Ok(artifacts)
        })
    }

    execute_urls(node.clone(), urls).await
}

async fn track_text_node(
    node: &SelectorNode,
    doc: &Html,
) -> Result<String, Box<dyn std::error::Error>> {
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

    Ok(text)
}
