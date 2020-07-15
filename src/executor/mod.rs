use crate::selector_node::{SelectorTree, SelectorNode, SelectorType};
use reqwest;
use reqwest::Url;
use scraper::{Html, Selector};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

pub async fn crawl(selector_tree: &SelectorTree) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
    let doc = fetch(&selector_tree.start_url).await?;

    Ok(vec![Artifact {
        tag: "source_url".to_string(),
        data: Rc::new(selector_tree.start_url.clone()),
        children: track_nodes(&selector_tree.selectors, &doc).await?
    }])
}

#[derive(Debug)]
pub struct Artifact {
    pub tag: String,
    pub data: Rc<String>,
    pub children: Vec<Artifact>,
}

async fn fetch(
    url: &String,
) -> Result<Html, Box<dyn std::error::Error>> {
    let body = reqwest::get(Url::parse(url)?).await?.text().await?;
    Ok(Html::parse_document(&body))
}

async fn track_nodes(
    nodes: &Vec<SelectorNode>, // title, body
    doc: &Html // contents page
) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
    let mut artifacts: Vec<Artifact> = vec![];
    for node in nodes {
        match node.selector_type {
            SelectorType::Link => {
                let mut children = track_link_node(&node, &doc).await?;
                // [link[title, body], link[title, body], link[title, body] ...]
                artifacts.append(&mut children);
            }
            SelectorType::Text => {
                let data = track_text_node(&node, &doc).await?;
                artifacts.push(Artifact {
                    tag: node.id.clone(),
                    data: Rc::new(data),
                    children: vec![],
                })
            }
            _ => panic!(),
        };
    }

    Ok(artifacts)
}

async fn track_link_node(
    node: &SelectorNode,
    doc: &Html,
) -> Result<Vec<Artifact>, Box<dyn std::error::Error>> {
    let selector = Selector::parse(&node.selector).unwrap();
    let urls: Vec<Rc<String>> = doc
        .select(&selector)
        .map(|element| Rc::new(element.value().attr("href").unwrap().to_string()))
        .collect();

    // helper for recursive async function. ref here: https://doc.rust-lang.org/error-index.html#E0733
    fn execute_urls(
        node: SelectorNode,
        urls: Vec<Rc<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Artifact>, Box<dyn std::error::Error>>>>> {
        Box::pin(async move {

            let mut artifacts: Vec<Artifact> = vec![];
            for url in urls {
                let doc = fetch(&url).await?;
                let children = track_nodes(&node.children, &doc).await?;
                artifacts.push(Artifact {
                    tag: node.id.clone(),
                    data: Rc::clone(&url),
                    children,
                });
            }
            Ok(artifacts)
        })
    }

    // TODO: use Arc<T> to 'SelectorNode'
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
