#[cfg(test)]
mod test;

use crate::crawler::Artifact;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub fn format(artifacts: Vec<Artifact>, column: Vec<&'static str>) -> Result<Vec<Vec<Arc<String>>>> {
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

fn format_map_list(artifacts: Vec<Artifact>) -> Vec<HashMap<String, Arc<String>>> {
    let mut result: Vec<HashMap<String, Arc<String>>> = Vec::with_capacity(artifacts.len());

    let mut leaves: Vec<Artifact> = vec![];

    for artifact in artifacts {
        if artifact.children.len() == 0 && artifact.tag != "source_url" {
            leaves.push(artifact);
            continue;
        }

        let mut children = format_map_list(artifact.children);

        for child in &mut children {
            child.insert(artifact.tag.clone(), artifact.data.clone());
        }
        result.append(&mut children)
    }

    if leaves.len() != 0 {
        let mut map = HashMap::new();
        for leaf in leaves {
            map.insert(leaf.tag, leaf.data);
        }

        result.push(map);
    }

    result
}
