use crate::domain::suggest::ModelSuggest;
use core_model::coco_struct::ClassInfo;
use core_model::Settings;
use std::fs;

pub struct Suggester;

impl Suggester {
    pub fn run(project: String) {
        match Suggester::load_struct(project) {
            Ok(model) => {
                let suggest: ModelSuggest = ModelSuggest::new(model);
                suggest.analysis_all();
            }
            Err(_) => {}
        }
    }

    fn load_struct(project: String) -> Result<Vec<ClassInfo>, String> {
        let file_name = format!("{}.json", project);
        let path = Settings::struct_dir().join(file_name);
        let contents;
        match fs::read_to_string(path) {
            Ok(str) => contents = str,
            Err(error) => {
                return Err(format!("{}", error));
            }
        }
        let model: Vec<ClassInfo>;
        match serde_json::from_str(contents.as_str()) {
            Ok(m) => model = m,
            Err(error) => {
                return Err(format!("{}", error));
            }
        }
        return Ok(model);
    }
}
