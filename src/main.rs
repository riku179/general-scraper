mod executor;
mod selector_node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://formula1-data.com/article".to_string();
    let sitemap_json = r###"
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
    }
  ]
}
    "###;
    let sitemap = selector_node::SiteMap::new(sitemap_json.to_string()).unwrap();
    let selectors = selector_node::SelectorNode::new(sitemap);

    executor::execute(&selectors, url).await
}
