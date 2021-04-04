use core_model::coco_struct::ClassInfo;

#[allow(dead_code)]
pub struct ModelSuggest {
    model: Vec<ClassInfo>,
}

impl ModelSuggest {
    pub fn new(model: Vec<ClassInfo>) -> ModelSuggest {
        ModelSuggest { model }
    }
    /// zh-CN: 过长参数
    /// en-US: Long Parameter List
    /// suggest:
    /// zh-CN: 引入参数对象
    /// en-US: Introduce Parameter Object
    pub fn find_long_parameter_list_method(&self) {
        // let max_parameters = 5;
        for info in &self.model {
            for method in &info.methods {
                if method.parameter_too_long() {
                    println!("Parameter list too loong: {:?}", method);
                }
            }
        }
    }

    pub fn analysis_all(&self) {
        self.find_long_parameter_list_method();
    }
}
