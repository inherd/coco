use std::collections::{BTreeMap, HashSet};

pub fn light_detect<'a>(names: &HashSet<String>) -> BTreeMap<&'a str, bool> {
    let mut tags = BTreeMap::new();
    tags.insert("workspace.java.gradle", names.contains("build.gradle"));
    tags.insert(
        "workspace.java.gradle.composite",
        names.contains("build.gradle") && names.contains("settings.gradle"),
    );
    tags.insert("workspace.java.pom", names.contains("pom.xml"));
    tags
}
