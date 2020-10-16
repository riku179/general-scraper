use crate::crawler::format;
use crate::crawler::Artifact;
use std::sync::Arc;

#[test]
fn format_test() {
    let test_data = vec![
        (
            // normal
            vec!["title", "link", "body", "source_url"],
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Some(Arc::new("http://url-root.com/article".to_string())),
                children: vec![
                    Artifact {
                        tag: "link".to_string(),
                        data: Some(Arc::new("http://url-a.com".to_string())),
                        children: vec![
                            Artifact {
                                tag: "title".to_string(),
                                data: Some(Arc::new("title A".to_string())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".to_string(),
                                data: Some(Arc::new("body A1 body A2".to_string())),
                                children: vec![],
                            },
                        ],
                    },
                    Artifact {
                        tag: "link".to_string(),
                        data: Some(Arc::new("http://url-b.com".to_string())),
                        children: vec![
                            Artifact {
                                tag: "title".to_string(),
                                data: Some(Arc::new("title B".to_string())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".to_string(),
                                data: Some(Arc::new("body B1 body B2".to_string())),
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
            vec![
                vec![
                    Arc::new("title A".to_string()),
                    Arc::new("http://url-a.com".to_string()),
                    Arc::new("body A1 body A2".to_string()),
                    Arc::new("http://url-root.com/article".to_string()),
                ],
                vec![
                    Arc::new("title B".to_string()),
                    Arc::new("http://url-b.com".to_string()),
                    Arc::new("body B1 body B2".to_string()),
                    Arc::new("http://url-root.com/article".to_string()),
                ],
            ],
        ),
        (
            // empty
            vec![],
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Some(Arc::new("http://url-root.com/article".to_string())),
                children: vec![],
            }],
            vec![],
        ),
        (
            vec!["title", "body", "source_url"],
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Some(Arc::new("http://url-root.com/article".to_string())),
                children: vec![
                    Artifact {
                        tag: "content".to_string(),
                        data: None,
                        children: vec![
                            Artifact {
                                tag: "title".to_string(),
                                data: Some(Arc::new("title a".to_string())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".to_string(),
                                data: Some(Arc::new("body a".to_string())),
                                children: vec![],
                            },
                        ],
                    },
                    Artifact {
                        tag: "content".to_string(),
                        data: None,
                        children: vec![
                            Artifact {
                                tag: "title".to_string(),
                                data: Some(Arc::new("title b".to_string())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".to_string(),
                                data: Some(Arc::new("body b".to_string())),
                                children: vec![],
                            },
                        ],
                    },
                    Artifact {
                        tag: "content".to_string(),
                        data: None,
                        children: vec![
                            Artifact {
                                tag: "title".to_string(),
                                data: Some(Arc::new("title c".to_string())),
                                children: vec![],
                            },
                            Artifact {
                                tag: "body".to_string(),
                                data: Some(Arc::new("body c".to_string())),
                                children: vec![],
                            },
                        ],
                    },
                ],
            }],
            vec![
                vec![
                    Arc::new("title a".to_string()),
                    Arc::new("body a".to_string()),
                    Arc::new("http://url-root.com/article".to_string()),
                ],
                vec![
                    Arc::new("title b".to_string()),
                    Arc::new("body b".to_string()),
                    Arc::new("http://url-root.com/article".to_string()),
                ],
                vec![
                    Arc::new("title c".to_string()),
                    Arc::new("body c".to_string()),
                    Arc::new("http://url-root.com/article".to_string()),
                ],
            ],
        ),
    ];

    for (column, artifacts, expected) in test_data {
        let actual = format(artifacts, column).unwrap();
        assert_eq!(expected, actual)
    }
}
