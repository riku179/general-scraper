#[cfg(test)]
mod test;

use crate::selector_node::{SelectorNode, SelectorTree, SelectorType};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest;
use reqwest::header::{HeaderValue, IF_MODIFIED_SINCE};
use reqwest::{Client, Url};
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchParam {
    pub if_modified_since: Option<DateTime<Utc>>,
}

impl FetchParam {
    pub fn new() -> Self {
        FetchParam {
            if_modified_since: None,
        }
    }
}

#[async_trait]
pub trait FetchClient {
    async fn fetch(&self, url: &String, param: FetchParam) -> Result<Option<Html>>;
}

pub struct WebFetcher {
    client: Client,
}

impl WebFetcher {
    pub fn new() -> Self {
        WebFetcher {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl FetchClient for WebFetcher {
    async fn fetch(&self, url: &String, param: FetchParam) -> Result<Option<Html>> {
        let mut req = self.client.get(Url::parse(url)?);
        if let Some(date) = param.if_modified_since {
            req = req.header(
                IF_MODIFIED_SINCE,
                HeaderValue::from_str(&date.to_rfc2822())?,
            );
        }
        let resp = req.send().await?.error_for_status()?;

        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(None);
        }

        let body = resp.text().await?;
        Ok(Some(Html::parse_document(&body)))
    }
}

#[derive(Debug, PartialEq)]
pub struct Artifact {
    pub tag: String,
    pub data: Rc<String>,
    pub children: Vec<Artifact>,
}

pub struct Executor<F: FetchClient> {
    fetcher: Arc<F>,
    access_log: HashMap<String, String>,
}

impl<F: FetchClient> Executor<F> {
    pub fn new(fetcher: F, access_log: HashMap<String, String>) -> Self {
        Executor {
            fetcher: Arc::new(fetcher),
            access_log,
        }
    }
}

impl<F: 'static + FetchClient> Executor<F> {
    pub async fn crawl(
        mut self,
        selector_tree: &SelectorTree,
    ) -> Result<(Vec<Artifact>, HashMap<String, String>)> {
        let mut param = FetchParam::new();
        if let Some(value) = self.access_log.get(&selector_tree.start_url) {
            param.if_modified_since =
                Some(DateTime::parse_from_rfc2822(value)?.with_timezone(&Utc));
        }

        let children;
        if let Some(doc) = self.fetcher.fetch(&selector_tree.start_url, param).await? {
            let now: DateTime<Utc> = Utc::now();
            self.access_log
                .insert(selector_tree.start_url.clone(), now.to_rfc2822());

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
            self.access_log,
        ))
    }

    async fn track_nodes(
        &self,
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

    async fn track_link_node(&self, node: &SelectorNode, doc: &Html) -> Result<Vec<Artifact>> {
        let selector = Selector::parse(&node.selector).unwrap();
        let urls: Vec<Rc<String>> = doc
            .select(&selector)
            .map(|element| Rc::new(element.value().attr("href").unwrap().to_string()))
            .collect();

        // TODO: use Arc<T> to 'SelectorNode'
        let mut artifacts: Vec<Artifact> = vec![];
        for url in urls {
            let mut param = FetchParam::new();
            if let Some(value) = self.access_log.get(&**url) {
                param.if_modified_since =
                    Some(DateTime::parse_from_rfc2822(value)?.with_timezone(&Utc));
            }

            let children = self
                .execute_url(node.clone(), Rc::clone(&url), param)
                .await?;
            artifacts.push(Artifact {
                tag: node.id.clone(),
                data: Rc::clone(&url),
                children,
            });
        }

        Ok(artifacts)
    }

    // helper for recursive async function. ref here: https://doc.rust-lang.org/error-index.html#E0733
    fn execute_url<'a>(
        &'a self,
        node: SelectorNode,
        url: Rc<String>,
        param: FetchParam,
    ) -> Pin<Box<(dyn Future<Output = Result<Vec<Artifact>>> + 'a)>> {
        Box::pin(async move {
            if let Some(doc) = self.fetcher.fetch(&url, param).await? {
                Ok(self.track_nodes(&node.children, &doc).await?)
            } else {
                Ok(vec![])
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
