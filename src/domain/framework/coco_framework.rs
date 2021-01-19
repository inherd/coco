use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CocoFramework {
    pub name: String,
    pub path: String,
    // for find the projects
    pub relative_path: String,
    // in some languages has different framework file
    // |   languages |   files    |
    // |-------------|------------|
    // | Java        | build.gradle, settings.gradle |
    pub framework_files: Vec<String>,
    // in JVM projects, has different languages, such as Java, Groovy, Kotlin...
    pub languages: Vec<String>,
}
