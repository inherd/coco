use framework::framework_detector::Framework;

#[derive(Serialize, PartialEq, Debug, Clone)]
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

impl CocoFramework {
    pub fn from(f: Framework) -> CocoFramework {
        CocoFramework {
            name: f.name,
            path: f.path,
            relative_path: f.relative_path,
            framework_files: f.files,
            languages: f.languages,
        }
    }
}
