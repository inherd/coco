use std::collections::{BTreeMap, HashSet};

pub fn light_detect<'a>(names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
    let mut tags = BTreeMap::new();
    tags.insert(
        "workspace.bower",
        names.contains("bower.json") || names.contains("bower_components"),
    );
    tags.insert(
        "workspace.npm",
        names.contains("package.json") || names.contains("node_modules"),
    );
    tags
}
