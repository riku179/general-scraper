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

    fn dump_access_logs(self) -> Vec<String> {
        vec![]
    }
}

#[tokio::test]
async fn fetcher_crawler_test() {
    let test_data = vec![
        (
            "test SelectorLink",
            vec![
                (
                    "http://url-root.com/article".into(),
                    r###"
                    <a class="url" href="http://url-a.com">url a</a>
                    <a class="url" href="http://url-b.com">url b</a>
                "###
                    .into(),
                ),
                (
                    "http://url-a.com".into(),
                    r###"
                    <p class="title">title A</p>
                    <p class="body">body A1</p>
                    <p class="body">body A2</p>
                "###
                    .into(),
                ),
                (
                    "http://url-b.com".into(),
                    r###"
                    <p class="title">title B</p>
                    <p class="body">body B1</p>
                    <p class="body">body B2</p>
                "###
                    .into(),
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
            .into(),
            vec![Artifact {
                tag: "source_url".into(),
                data: Some(Arc::new("http://url-root.com/article".into())),
                children: vec![
                    Artifact {
                        tag: "link".into(),
                        data: Some(Arc::new("http://url-a.com".into())),
                        children: vec![
                            Artifact {
                                tag: "title".into(),
                                data: Some(Arc::new("title A".into())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".into(),
                                data: Some(Arc::new("body A1 body A2".into())),
                                children: vec![],
                            },
                        ],
                    },
                    Artifact {
                        tag: "link".into(),
                        data: Some(Arc::new("http://url-b.com".into())),
                        children: vec![
                            Artifact {
                                tag: "title".into(),
                                data: Some(Arc::new("title B".into())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".into(),
                                data: Some(Arc::new("body B1 body B2".into())),
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
        ),
        (
            "test SelectorElement",
            vec![(
                "http://url-root.com/article".into(),
                r###"
                    <div class="contents">
                        <p class="title">title a</p>
                        <p class="body">body a</p>
                    </div>
                    <div class="contents">
                        <p class="title">title b</p>
                        <p class="body">body b</p>
                    </div>
                    <div class="contents">
                        <p class="title">title c</p>
                        <p class="body">body c</p>
                    </div>
                "###
                .into(),
            )],
            r###"
{
  "_id": "test",
  "startUrl": [
    "http://url-root.com/article"
  ],
  "selectors": [
    {
      "id": "content",
      "type": "SelectorElement",
      "parentSelectors": [
        "_root"
      ],
      "selector": ".contents",
      "multiple": true,
      "delay": 0
    },
    {
      "id": "title",
      "type": "SelectorText",
      "parentSelectors": [
        "content"
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
        "content"
      ],
      "selector": ".body",
      "multiple": true,
      "regex": "",
      "delay": 0
    }
  ]
}
    "###
            .into(),
            vec![Artifact {
                tag: "source_url".into(),
                data: Some(Arc::new("http://url-root.com/article".into())),
                children: vec![
                    Artifact {
                        tag: "content".into(),
                        data: None,
                        children: vec![
                            Artifact {
                                tag: "title".into(),
                                data: Some(Arc::new("title a".into())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".into(),
                                data: Some(Arc::new("body a".into())),
                                children: vec![],
                            },
                        ],
                    },
                    Artifact {
                        tag: "content".into(),
                        data: None,
                        children: vec![
                            Artifact {
                                tag: "title".into(),
                                data: Some(Arc::new("title b".into())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".into(),
                                data: Some(Arc::new("body b".into())),
                                children: vec![],
                            },
                        ],
                    },
                    Artifact {
                        tag: "content".into(),
                        data: None,
                        children: vec![
                            Artifact {
                                tag: "title".into(),
                                data: Some(Arc::new("title c".into())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".into(),
                                data: Some(Arc::new("body c".into())),
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
        ),
        (
            "test SelectorImage",
            vec![(
                "http://url-root.com/article".into(),
                r###"
                <img class="thumbnail" src="http://image-a.com">
                <img class="thumbnail" src="http://image-b.com">
                <img class="thumbnail" src="http://image-c.com">
                "###
                .into(),
            )],
            r###"
{
  "_id": "test",
  "startUrl": [
    "http://url-root.com/article"
  ],
  "selectors": [
    {
      "id": "image",
      "type": "SelectorImage",
      "parentSelectors": [
        "_root"
      ],
      "selector": ".thumbnail",
      "multiple": true,
      "delay": 0
    }
  ]
}
    "###
            .into(),
            vec![Artifact {
                tag: "source_url".into(),
                data: Some(Arc::new("http://url-root.com/article".into())),
                children: vec![
                    Artifact {
                        tag: "image".into(),
                        data: Some(Arc::new("http://image-a.com".into())),
                        children: vec![],
                    },
                    Artifact {
                        tag: "image".into(),
                        data: Some(Arc::new("http://image-b.com".into())),
                        children: vec![],
                    },
                    Artifact {
                        tag: "image".into(),
                        data: Some(Arc::new("http://image-c.com".into())),
                        children: vec![],
                    },
                ],
            }],
        ),
    ];

    for (name, url_map, selector_json, expected) in test_data {
        let mocked_fetcher = MockedFetcher::new(url_map);
        let executor = Crawler::new(mocked_fetcher, vec![]);
        let selector = SelectorTree::new(selector_json).unwrap();
        let (actual, _) = executor.crawl(&selector).await.unwrap();

        assert_eq!(expected, actual, "{}", name)
    }
}
