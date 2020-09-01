use crate::crawler::selector_node::SelectorTree;
use crate::crawler::{Artifact, Crawler, FetchClient};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio;

// mapping Url -> Html
#[derive(Debug)]
struct MockedFetcher {
    mapping: HashMap<String, String>,
}

impl MockedFetcher {
    fn new(mapping_vec: Vec<(String, String)>) -> Self {
        let mut mapping = HashMap::new();

        for (url, html) in mapping_vec {
            mapping.insert(url, html);
        }
        MockedFetcher { mapping }
    }
}

#[async_trait]
impl FetchClient for MockedFetcher {
    async fn fetch(&mut self, url: &String, _: bool) -> Result<String> {
        if let Some(content) = self.mapping.get(url) {
            Ok(content.clone())
        } else {
            Err(anyhow!("html not found by the url: {}", &url))
        }
    }

    fn gen_access_logs(self) -> Vec<String> {
        vec![]
    }
}

#[tokio::test]
async fn fetcher_crawler_test() {
    let test_data = vec![(
        vec![
            (
                "http://url-root.com/article".to_string(),
                r###"
                    <a class="url" href="http://url-a.com">url a</a>
                    <a class="url" href="http://url-b.com">url b</a>
                "###
                .to_string(),
            ),
            (
                "http://url-a.com".to_string(),
                r###"
                    <p class="title">title A</p>
                    <p class="body">body A1</p>
                    <p class="body">body A2</p>
                "###
                .to_string(),
            ),
            (
                "http://url-b.com".to_string(),
                r###"
                    <p class="title">title B</p>
                    <p class="body">body B1</p>
                    <p class="body">body B2</p>
                "###
                .to_string(),
            ),
        ],
        r###"
{
  "_id": "test",
  "startUrl": [
    "http://url-root.com/article"
  ],
  "selectors": [
    {
      "id": "link",
      "type": "SelectorLink",
      "parentSelectors": [
        "_root"
      ],
      "selector": ".url",
      "multiple": true,
      "delay": 0
    },
    {
      "id": "title",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": ".title",
      "multiple": false,
      "regex": "",
      "delay": 0
    },
    {
      "id": "body",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": ".body",
      "multiple": true,
      "regex": "",
      "delay": 0
    }
  ]
}
    "###
        .to_string(),
        vec![Artifact {
            tag: "source_url".to_string(),
            data: Arc::new("http://url-root.com/article".to_string()),
            children: vec![
                Artifact {
                    tag: "link".to_string(),
                    data: Arc::new("http://url-a.com".to_string()),
                    children: vec![
                        Artifact {
                            tag: "title".to_string(),
                            data: Arc::new("title A".to_string()),
                            children: vec![],
                        },
                        Artifact {
                            tag: "body".to_string(),
                            data: Arc::new("body A1 body A2".to_string()),
                            children: vec![],
                        },
                    ],
                },
                Artifact {
                    tag: "link".to_string(),
                    data: Arc::new("http://url-b.com".to_string()),
                    children: vec![
                        Artifact {
                            tag: "title".to_string(),
                            data: Arc::new("title B".to_string()),
                            children: vec![],
                        },
                        Artifact {
                            tag: "body".to_string(),
                            data: Arc::new("body B1 body B2".to_string()),
                            children: vec![],
                        },
                    ],
                },
            ],
        }],
    )];

    for (url_map, selector_json, expected) in test_data {
        let mocked_fetcher = MockedFetcher::new(url_map);
        let executor = Crawler::new(mocked_fetcher, vec![]);
        let selector = SelectorTree::new(selector_json).unwrap();
        let (actual, _) = executor.crawl(&selector).await.unwrap();

        assert_eq!(expected, actual)
    }
}
