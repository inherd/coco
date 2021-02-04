use std::collections::{BTreeMap, HashSet};

pub fn light_detect<'a>(names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
    let mut tags = BTreeMap::new();
    tags.insert(
        "workspace.go",
        names.contains("go.mod") || names.contains("main.go"),
    );
    tags
}
