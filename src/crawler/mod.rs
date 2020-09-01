mod formatter;
mod selector_node;
#[cfg(test)]
mod test;

pub use formatter::format;
pub use selector_node::{SelectorNode, SelectorTree, SelectorType};

use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[async_trait]
pub trait FetchClient {
    async fn fetch(&mut self, url: &String, logging: bool) -> Result<String>;
    fn gen_access_logs(self) -> Vec<String>;
}

pub struct WebFetcher {
    access_logs: Vec<String>,
    client: Client,
}

impl WebFetcher {
    pub fn new() -> Self {
        WebFetcher {
            client: Client::new(),
            access_logs: vec![],
        }
    }
}

#[async_trait]
impl FetchClient for WebFetcher {
    async fn fetch(&mut self, url: &String, logging: bool) -> Result<String> {
        let req = self.client.get(Url::parse(url)?);
        let resp = req.send().await?.error_for_status()?;
        if logging {
            self.access_logs.push(url.clone());
        }

        let body = resp.text().await?;
        Ok(body)
    }

    fn gen_access_logs(self) -> Vec<String> {
        self.access_logs
    }
}

#[derive(Debug, PartialEq)]
pub struct Artifact {
    pub tag: String,
    pub data: Arc<String>,
    pub children: Vec<Artifact>,
}

pub struct Crawler<F: FetchClient> {
    fetcher: F,
    skip_urls: HashSet<String>,
}

impl<F: FetchClient> Crawler<F> {
    pub fn new(fetcher: F, skip_urls_vec: Vec<String>) -> Self {
        let mut skip_urls = HashSet::new();
        for skip_url in skip_urls_vec {
            skip_urls.insert(skip_url);
        }

        Crawler { fetcher, skip_urls }
    }
}

impl<F: 'static + FetchClient + Send> Crawler<F> {
    pub async fn crawl(
        mut self,
        selector_tree: &SelectorTree,
    ) -> Result<(Vec<Artifact>, Vec<String>)> {
        let doc = self.fetcher.fetch(&selector_tree.start_url, false).await?;
        let children = self
            .track_nodes(&selector_tree.selectors, &doc.clone())
            .await?;

        Ok((
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Arc::new(selector_tree.start_url.clone()),
                children,
            }],
            self.fetcher.gen_access_logs(),
        ))
    }

    async fn track_nodes(
        &mut self,
        nodes: &Vec<SelectorNode>,
        doc: &String,
    ) -> Result<Vec<Artifact>> {
        let mut artifacts: Vec<Artifact> = vec![];
        for node in nodes {
            match node.selector_type {
                SelectorType::Link => {
                    let mut children = self.track_link_node(&node, &doc).await?;
                    artifacts.append(&mut children);
                }
                SelectorType::Text => {
                    let data = Self::track_text_node(&node, &doc).await?;
                    artifacts.push(Artifact {
                        tag: node.id.clone(),
                        data: Arc::new(data),
                        children: vec![],
                    })
                }
                _ => panic!(),
            };
        }

        Ok(artifacts)
    }

    async fn track_link_node(
        &mut self,
        node: &SelectorNode,
        doc: &String,
    ) -> Result<Vec<Artifact>> {
        let urls: Vec<Arc<String>>;
        {
            let doc = Html::parse_document(doc);
            let selector = Selector::parse(&node.selector).unwrap();
            urls = doc
                .select(&selector)
                .map(|element| Arc::new(element.value().attr("href").unwrap().to_string()))
                .collect();
        }

        let mut artifacts: Vec<Artifact> = vec![];
        for url in urls {
            if self.skip_urls.contains(&*url) {
                continue;
            }
            let children = self.execute_url(node.clone(), url.clone()).await?;
            artifacts.push(Artifact {
                tag: node.id.clone(),
                data: url.clone(),
                children,
            });
        }

        Ok(artifacts)
    }

    // helper for recursive async function. ref here: https://doc.rust-lang.org/error-index.html#E0733
    fn execute_url<'a>(
        &'a mut self,
        node: SelectorNode,
        url: Arc<String>,
    ) -> Pin<Box<(dyn Future<Output = Result<Vec<Artifact>>> + 'a + Send)>> {
        Box::pin(async move {
            let doc = self.fetcher.fetch(&url, true).await?;
            Ok(self.track_nodes(&node.children, &doc).await?)
        })
    }

    async fn track_text_node(node: &SelectorNode, doc: &String) -> Result<String> {
        let doc = Html::parse_document(doc);
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
