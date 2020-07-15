use crate::selector_node::SelectorTree;
use serde_json;

mod formatter;
mod executor;
mod selector_node;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://formula1-data.com/article".to_string();
    let sitemap_json = r###"
{
  "_id": "formula1-dataq",
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
      "id": "pub_date",
      "type": "SelectorText",
      "parentSelectors": [
        "link"
      ],
      "selector": "li time",
      "multiple": false,
      "regex": "",
      "delay": 0
    }
  ]
}
    "###;
    let selector = SelectorTree::new(url, sitemap_json.to_string())?;

    let artifacts = executor::crawl(&selector).await?;
    // println!("{:?}", &artifacts);

    let formatted = formatter::format(artifacts);

    println!("{}", serde_json::to_string_pretty(&formatted)?);

    Ok(())
}
