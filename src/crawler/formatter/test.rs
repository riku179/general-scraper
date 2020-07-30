use crate::crawler::format;
use crate::crawler::Artifact;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn format_test() {
    let test_data = vec![
        (
            // normal
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
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Arc::new("http://url-root.com/article".to_string()),
                children: vec![],
            }],
            vec![],
        ),
    ];

    for (artifacts, expected) in test_data {
        let actual = format(artifacts, vec!["title", "link", "body", "source_url"]).unwrap();
        assert_eq!(expected, actual)
    }
}
