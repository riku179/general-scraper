#[cfg(test)]
mod test;

use crate::selector_node::{SelectorNode, SelectorTree, SelectorType};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use reqwest;
use reqwest::header::{HeaderValue, IF_MODIFIED_SINCE};
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

#[async_trait]
pub trait FetchClient {
    async fn fetch(&mut self, url: &String) -> Result<Option<Html>>;
    fn gen_access_logs(self) -> HashMap<String, String>;
}

pub struct WebFetcher {
    access_log: HashMap<String, String>,
    client: Client,
}

impl WebFetcher {
    pub fn new(access_log: HashMap<String, String>) -> Self {
        WebFetcher {
            client: Client::new(),
            access_log,
        }
    }
}

#[async_trait]
impl FetchClient for WebFetcher {
    async fn fetch(&mut self, url: &String) -> Result<Option<Html>> {
        let mut req = self.client.get(Url::parse(url)?);
        if let Some(date) = self.access_log.get(url) {
            req = req.header(IF_MODIFIED_SINCE, HeaderValue::from_str(date)?);
        }
        let resp = req.send().await?.error_for_status()?;
        if let Some(v) = resp.headers().get(reqwest::header::LAST_MODIFIED) {
            self.access_log.insert(url.clone(), v.to_str()?.to_string());
        } else {
            self.access_log.insert(url.clone(), Utc::now().to_rfc2822());
        }
        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(None);
        }

        let body = resp.text().await?;
        Ok(Some(Html::parse_document(&body)))
    }

    fn gen_access_logs(self) -> HashMap<String, String> {
        self.access_log
    }
}

#[derive(Debug, PartialEq)]
pub struct Artifact {
    pub tag: String,
    pub data: Rc<String>,
    pub children: Vec<Artifact>,
}

pub struct Executor<F: FetchClient> {
    fetcher: F,
}

impl<F: FetchClient> Executor<F> {
    pub fn new(fetcher: F) -> Self {
        Executor { fetcher }
    }
}

impl<F: 'static + FetchClient> Executor<F> {
    pub async fn crawl(
        mut self,
        selector_tree: &SelectorTree,
    ) -> Result<(Vec<Artifact>, HashMap<String, String>)> {
        let children;
        if let Some(doc) = self.fetcher.fetch(&selector_tree.start_url).await? {
            children = self.track_nodes(&selector_tree.selectors, &doc).await?;
        } else {
            children = vec![]
        }

        Ok((
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Rc::new(selector_tree.start_url.clone()),
                children,
            }],
            self.fetcher.gen_access_logs(),
        ))
    }

    async fn track_nodes(
        &mut self,
        nodes: &Vec<SelectorNode>, // title, body
        doc: &Html,                // contents page
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
                        data: Rc::new(data),
                        children: vec![],
                    })
                }
                _ => panic!(),
            };
        }

        Ok(artifacts)
    }

    async fn track_link_node(&mut self, node: &SelectorNode, doc: &Html) -> Result<Vec<Artifact>> {
        let selector = Selector::parse(&node.selector).unwrap();
        let urls: Vec<Rc<String>> = doc
            .select(&selector)
            .map(|element| Rc::new(element.value().attr("href").unwrap().to_string()))
            .collect();

        // TODO: use Arc<T> to 'SelectorNode'
        let mut artifacts: Vec<Artifact> = vec![];
        for url in urls {
            // save as artifact only if it is first time
            if let Some(children) = self.execute_url(node.clone(), Rc::clone(&url)).await? {
                artifacts.push(Artifact {
                    tag: node.id.clone(),
                    data: Rc::clone(&url),
                    children,
                });
            }
        }

        Ok(artifacts)
    }

    // helper for recursive async function. ref here: https://doc.rust-lang.org/error-index.html#E0733
    fn execute_url<'a>(
        &'a mut self,
        node: SelectorNode,
        url: Rc<String>,
    ) -> Pin<Box<(dyn Future<Output = Result<Option<Vec<Artifact>>>> + 'a)>> {
        Box::pin(async move {
            if let Some(doc) = self.fetcher.fetch(&url).await? {
                Ok(Some(self.track_nodes(&node.children, &doc).await?))
            } else {
                Ok(None)
            }
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
