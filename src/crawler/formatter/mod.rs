#[cfg(test)]
mod test;

use crate::crawler::Artifact;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub fn format(
    artifacts: Vec<Artifact>,
    column: Vec<&'static str>,
) -> Result<Vec<Vec<Arc<String>>>> {
    let mut result = Vec::with_capacity(artifacts.len());

    let contents = format_map_list(artifacts);

    for content in contents {
        let mut row = Vec::with_capacity(column.len());
        for col in &column {
            if let Some(v) = content.get(*col) {
                row.push(v.clone());
            } else {
                return Err(anyhow!("field '{}' not found in {:?}", col, content));
            }
        }
        result.push(row);
    }

    Ok(result)
}

// flatten artifact tree to list of key-value
fn format_map_list(artifacts: Vec<Artifact>) -> Vec<HashMap<String, Arc<String>>> {
    let mut result: Vec<HashMap<String, Arc<String>>> = Vec::with_capacity(artifacts.len());

    // set of artifacts which do not have children
    let mut leaves: Vec<Artifact> = vec![];

    for artifact in artifacts {
        // skip leaf node and "source_url" node to push all rows after all
        if artifact.children.len() == 0 && artifact.tag != "source_url" {
            leaves.push(artifact);
            continue;
        }

        let mut children = format_map_list(artifact.children);

        for child in &mut children {
            // ex: ElementType not included in result
            if let Some(data) = &artifact.data {
                child.insert(artifact.tag.clone(), data.clone());
            }
        }
        result.append(&mut children)
    }

    if leaves.len() != 0 {
        let mut map: HashMap<String, Arc<String>> = HashMap::new();
        for leaf in leaves {
            // ex: ElementType not included in result
            if let Some(data) = leaf.data {
                map.insert(leaf.tag, data);
            }
        }

        result.push(map);
    }

    result
}
