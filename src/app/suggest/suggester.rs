use crate::domain::cloc::ClocSummary;
use crate::domain::suggest::ModelSuggest;
use core_model::coco_struct::ClassInfo;
use core_model::Settings;
use std::fs;
use std::path::PathBuf;

pub struct Suggester;

impl Suggester {
    pub fn run(project: String) {
        if let Ok(model) = Suggester::load_struct(project) {
            let suggest: ModelSuggest = ModelSuggest::new(model);
            suggest.analysis_all();
        }
    }

    #[allow(dead_code)]
    fn load_cloc(project: String) -> Result<Vec<ClocSummary>, String> {
        let type_dir = Settings::cloc();
        let contents = Suggester::read_content(project, type_dir)?;

        let model: Vec<ClocSummary>;
        match serde_json::from_str(contents.as_str()) {
            Ok(m) => model = m,
            Err(error) => {
                return Err(format!("{}", error));
            }
        }
        return Ok(model);
    }

    fn load_struct(project: String) -> Result<Vec<ClassInfo>, String> {
        let type_dir = Settings::struct_dir();
        let contents = Suggester::read_content(project, type_dir)?;

        let model: Vec<ClassInfo>;
        match serde_json::from_str(contents.as_str()) {
            Ok(m) => model = m,
            Err(error) => {
                return Err(format!("{}", error));
            }
        }
        return Ok(model);
    }

    fn read_content(project: String, type_dir: PathBuf) -> Result<String, String> {
        let file_name = format!("{}.json", project);
        let path = type_dir.join(file_name);
        let contents;
        match fs::read_to_string(path) {
            Ok(str) => contents = str,
            Err(error) => {
                return Err(format!("{}", error));
            }
        }
        Ok(contents)
    }
}
