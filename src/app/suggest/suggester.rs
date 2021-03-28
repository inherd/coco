use crate::domain::suggest::ModelSuggest;
use core_model::coco_struct::ClassInfo;
use core_model::Settings;
use std::fs;

pub struct Suggester;

impl Suggester {
    pub fn run(project: String) {
        let file_name = format!("{}.json", project);
        let path = Settings::struct_dir().join(file_name);
        let contents = fs::read_to_string(path).expect("lost path");
        let model: Vec<ClassInfo> = serde_json::from_str(contents.as_str()).expect("error format");
        let suggest: ModelSuggest = ModelSuggest::new(model);
        suggest.analysis_all();
    }
}
