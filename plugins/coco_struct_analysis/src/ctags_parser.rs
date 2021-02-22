use crate::coco_struct::ClassInfo;
use regex::Regex;
use std::collections::HashMap;

pub struct CtagsParser {
    pub class_map: HashMap<String, ClassInfo>,
    pub classes: Vec<ClassInfo>,
}

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(
        r"(?x)
^(?P<class_name>[A-Za-z0-9_]+)
\t(?P<file_name>([^\t]+))
\t([^\t]+)\tclass"
    )
    .unwrap();
    static ref INHERITS_RES: Regex = Regex::new(r"inherits:([A-Za-z0-9_\:,]+)").unwrap();
}

impl Default for CtagsParser {
    fn default() -> Self {
        CtagsParser {
            class_map: Default::default(),
            classes: vec![],
        }
    }
}

impl CtagsParser {
    pub fn parse_class(&mut self, str: &str) {
        if let Some(captures) = CLASS_RE.captures(str) {
            let class_name = &captures["class_name"];
            let clazz = ClassInfo::new(class_name);

            self.class_map.insert(class_name.to_string(), clazz);
        }
    }
    pub fn parse_method_methods() {}

    pub fn classes(&self) -> Vec<ClassInfo> {
        let mut classes = vec![];
        for (_str, clz) in &self.class_map {
            classes.push(clz.clone());
        }

        return classes;
    }
}

#[cfg(test)]
mod test {
    use crate::ctags_parser::CtagsParser;

    #[test]
    pub fn should_parse_java_class() {
        let tags = "AsyncEventBus	AsyncEventBus.java	/^public class AsyncEventBus extends EventBus {$/;\"	class	line:31	language:Java	inherits:EventBus";
        let mut parser = CtagsParser::default();
        parser.parse_class(tags);

        let classes = parser.classes();
        assert_eq!(1, classes.len());
        assert_eq!("AsyncEventBus", classes[0].name);
    }
}
