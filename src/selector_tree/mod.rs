#[cfg(test)]
mod test;

use serde::Deserialize;
use serde_json;

#[derive(PartialEq, Debug)]
pub struct SelectorNode {
    pub id: String,
    pub selector_type: SelectorType,
    pub selector: String,
    pub multiple: bool,
    pub children: Vec<SelectorNode>,
}

impl SelectorNode {
    fn new(mut sitemap: SiteMap) -> Vec<Self> {
        build_selector_node(&mut sitemap.selectors, &"_root".to_string())
    }

    fn from_raw(raw: &RawSelector) -> Self {
        SelectorNode {
            id: raw.id.clone(),
            selector_type: SelectorType::from_str(&raw._type),
            selector: raw.selector.clone(),
            multiple: raw.multiple,
            children: vec![],
        }
    }
}

fn build_selector_node(raw_selectors: &Vec<RawSelector>, parent_id: &String) -> Vec<SelectorNode> {
    let mut children_selectors = vec![];

    for raw_selector in raw_selectors {
        if raw_selector.parent_selectors.contains(parent_id) {
            children_selectors.push(SelectorNode::from_raw(raw_selector));
        }
    }

    for child_selector in &mut children_selectors {
        child_selector
            .children
            .append(&mut build_selector_node(raw_selectors, &child_selector.id));
    }
    children_selectors
}

#[derive(PartialEq, Debug)]
pub enum SelectorType {
    Text,
    Link,
    Image,
}

impl SelectorType {
    fn from_str(s: &str) -> Self {
        match s {
            "SelectorText" => SelectorType::Text,
            "SelectorLink" => SelectorType::Link,
            "SelectorImage" => SelectorType::Image,
            _ => panic!("unknown selector type"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SiteMap {
    _id: String,
    #[serde(rename(deserialize = "startUrl"))]
    start_url: Vec<String>,
    selectors: Vec<RawSelector>,
}

impl SiteMap {
    fn new(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }
}

#[derive(Deserialize, Debug)]
struct RawSelector {
    id: String,
    #[serde(rename(deserialize = "type"))]
    _type: String,
    selector: String,
    multiple: bool,
    #[serde(rename(deserialize = "parentSelectors"))]
    parent_selectors: Vec<String>,
    delay: i32,
}
