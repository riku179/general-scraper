use crate::executor::Artifact;
use std::collections::HashMap;
use std::rc::Rc;

pub fn format(artifacts: Vec<Artifact>) -> Vec<HashMap<String, Rc<String>>> {
    let mut result: Vec<HashMap<String, Rc<String>>> = vec![];

    let mut leaves: Vec<Artifact> = vec![];

    for artifact in artifacts {
        if artifact.children.len() == 0 {
            leaves.push(artifact);
            continue
        }

        let mut children = format(artifact.children);

        for child in &mut children {
            child.insert(artifact.tag.clone(), Rc::clone(&artifact.data));
        }
        result.append(&mut children)
    };

    if leaves.len() != 0 {
        let mut map = HashMap::new();
        for leaf in leaves {
            map.insert(leaf.tag, leaf.data);
        };
        // pattern A
        result.push(map);
    }

    result
}