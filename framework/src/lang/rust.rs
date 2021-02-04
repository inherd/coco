use std::collections::{BTreeMap, HashSet};

pub fn light_detect<'a>(names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
    let mut tags = BTreeMap::new();
    tags.insert("workspace.rust.cargo", names.contains("Cargo.toml"));
    tags
}
