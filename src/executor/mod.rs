#[cfg(test)]
mod test;

use crate::selector_node::{SelectorNode, SelectorTree, SelectorType};
use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use reqwest::Url;
use scraper::{Html, Selector};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[async_trait]
pub trait Fetcher {
    async fn fetch(&self, url: &String) -> Result<Html>;
}

pub struct WebFetcher();

#[async_trait]
impl Fetcher for WebFetcher {
    async fn fetch(&self, url: &String) -> Result<Html> {
        let body = reqwest::get(Url::parse(url)?).await?.text().await?;
        Ok(Html::parse_document(&body))
    }
}

#[derive(Debug, PartialEq)]
pub struct Artifact {
    pub tag: String,
    pub data: Rc<String>,
    pub children: Vec<Artifact>,
}

pub struct Executor<F: Fetcher> {
    fetcher: Arc<F>,
}

impl<F: Fetcher> Executor<F> {
    pub fn new(fetcher: F) -> Self {
        Executor {
            fetcher: Arc::new(fetcher),
        }
    }
}

impl<F: 'static + Fetcher> Executor<F> {
    pub async fn crawl(&self, selector_tree: &SelectorTree) -> Result<Vec<Artifact>> {
        let doc = self.fetcher.fetch(&selector_tree.start_url).await?;

        Ok(vec![Artifact {
            tag: "source_url".to_string(),
            data: Rc::new(selector_tree.start_url.clone()),
            children: Self::track_nodes(Arc::clone(&self.fetcher), &selector_tree.selectors, &doc)
                .await?,
        }])
    }

    async fn track_nodes(
        fetcher: Arc<F>,
        nodes: &Vec<SelectorNode>, // title, body
        doc: &Html,                // contents page
    ) -> Result<Vec<Artifact>> {
        let mut artifacts: Vec<Artifact> = vec![];
        for node in nodes {
            match node.selector_type {
                SelectorType::Link => {
                    let mut children =
                        Self::track_link_node(Arc::clone(&fetcher), &node, &doc).await?;
                    // [link[title, body], link[title, body], link[title, body] ...]
                    artifacts.append(&mut children);
                }
                SelectorType::Text => {
                    let data = Self::track_text_node(&node, &doc).await?;
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
        fetcher: Arc<F>,
        node: &SelectorNode,
        doc: &Html,
    ) -> Result<Vec<Artifact>> {
        let selector = Selector::parse(&node.selector).unwrap();
        let urls: Vec<Rc<String>> = doc
            .select(&selector)
            .map(|element| Rc::new(element.value().attr("href").unwrap().to_string()))
            .collect();

        // TODO: use Arc<T> to 'SelectorNode'
        Self::execute_urls(fetcher, node.clone(), urls).await
    }

    // helper for recursive async function. ref here: https://doc.rust-lang.org/error-index.html#E0733
    fn execute_urls(
        fetcher: Arc<F>,
        node: SelectorNode,
        urls: Vec<Rc<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Artifact>>>>> {
        Box::pin(async move {
            let mut artifacts: Vec<Artifact> = vec![];
            for url in urls {
                let doc = fetcher.fetch(&url).await?;
                let children =
                    Self::track_nodes(Arc::clone(&fetcher), &node.children, &doc).await?;
                artifacts.push(Artifact {
                    tag: node.id.clone(),
                    data: Rc::clone(&url),
                    children,
                });
            }
            Ok(artifacts)
        })
    }

    async fn track_text_node(node: &SelectorNode, doc: &Html) -> Result<String> {
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
}
