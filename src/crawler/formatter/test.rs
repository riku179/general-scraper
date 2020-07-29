use crate::crawler::Artifact;
use crate::formatter::format;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
fn format_test() {
    let test_data = vec![
        (
            // normal
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
            vec![
                {
                    let mut map: HashMap<String, Rc<String>> = HashMap::new();
                    map.insert("link".to_string(), Rc::new("http://url-a.com".to_string()));
                    map.insert("title".to_string(), Rc::new("title A".to_string()));
                    map.insert("body".to_string(), Rc::new("body A1 body A2".to_string()));
                    map.insert(
                        "source_url".to_string(),
                        Rc::new("http://url-root.com/article".to_string()),
                    );
                    map
                },
                {
                    let mut map: HashMap<String, Rc<String>> = HashMap::new();
                    map.insert("link".to_string(), Rc::new("http://url-b.com".to_string()));
                    map.insert("title".to_string(), Rc::new("title B".to_string()));
                    map.insert("body".to_string(), Rc::new("body B1 body B2".to_string()));
                    map.insert(
                        "source_url".to_string(),
                        Rc::new("http://url-root.com/article".to_string()),
                    );
                    map
                },
            ],
        ),
        (
            // empty
            vec![Artifact {
                tag: "source_url".to_string(),
                data: Rc::new("http://url-root.com/article".to_string()),
                children: vec![],
            }],
            vec![],
        ),
    ];

    for (artifacts, expected) in test_data {
        let actual = format(artifacts);
        assert_eq!(expected, actual)
    }
}
