use crate::executor::{Artifact, Executor, FetchClient, FetchParam};
use crate::selector_node::SelectorTree;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use scraper::Html;
use std::collections::HashMap;
use std::rc::Rc;
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
    async fn fetch(&self, url: &String, _param: FetchParam) -> Result<Option<Html>> {
        if let Some(content) = self.mapping.get(url) {
            Ok(Some(Html::parse_fragment(content)))
        } else {
            Err(anyhow!("html not found by the url: {}", &url))
        }
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
        HashMap::new(),
        vec![Artifact {
            tag: "source_url".to_string(),
            data: Rc::new("http://url-root.com/article".to_string()),
            children: vec![
                Artifact {
                    tag: "link".to_string(),
                    data: Rc::new("http://url-a.com".to_string()),
                    children: vec![
                        Artifact {
                            tag: "title".to_string(),
                            data: Rc::new("title A".to_string()),
                            children: vec![],
                        },
                        Artifact {
                            tag: "body".to_string(),
                            data: Rc::new("body A1 body A2".to_string()),
                            children: vec![],
                        },
                    ],
                },
                Artifact {
                    tag: "link".to_string(),
                    data: Rc::new("http://url-b.com".to_string()),
                    children: vec![
                        Artifact {
                            tag: "title".to_string(),
                            data: Rc::new("title B".to_string()),
                            children: vec![],
                        },
                        Artifact {
                            tag: "body".to_string(),
                            data: Rc::new("body B1 body B2".to_string()),
                            children: vec![],
                        },
                    ],
                },
            ],
        }],
    )];

    for (url_map, selector_json, access_log, expected) in test_data {
        let mocked_fetcher = MockedFetcher::new(url_map);
        let executor = Executor::new(mocked_fetcher, access_log);
        let selector = SelectorTree::new(selector_json).unwrap();
        let (actual, _) = executor.crawl(&selector).await.unwrap();

        assert_eq!(expected, actual)
    }
}
