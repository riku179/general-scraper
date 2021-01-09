use crate::crawler::selector_node::{SelectorNode, SelectorType, SiteMap};

#[test]
fn test_selector_tree_new() {
    let testdata = r###"
{
  "_id": "formula1-data",
  "startUrl": [
    "https://formula1-data.com/article"
  ],
  "selectors": [
    {
      "id": "link",
      "type": "SelectorLink",
      "parentSelectors": [
        "_root"
      ],
      "selector": ".mdlGrid__col12 a",
      "multiple": true,
      "delay": 0
    },
    {
      "id": "title",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": "h1.entryHeader__title",
      "multiple": false,
      "regex": "",
      "delay": 0
    },
    {
      "id": "content",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": ".entry > p",
      "multiple": true,
      "regex": "",
      "delay": 0
    }
  ]
}
    "###;

    let expected = vec![SelectorNode {
        id: "link".into(),
        selector_type: SelectorType::Link,
        selector: ".mdlGrid__col12 a".into(),
        multiple: true,
        children: vec![
            SelectorNode {
                id: "title".into(),
                selector_type: SelectorType::Text,
                selector: "h1.entryHeader__title".into(),
                multiple: false,
                children: vec![],
            },
            SelectorNode {
                id: "content".into(),
                selector_type: SelectorType::Text,
                selector: ".entry > p".into(),
                multiple: true,
                children: vec![],
            },
        ],
    }];

    let sitemap = SiteMap::new(testdata.into()).unwrap();
    let actual = SelectorNode::new(sitemap);

    assert_eq!(actual, expected)
}
