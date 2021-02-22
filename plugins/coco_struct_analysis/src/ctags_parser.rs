use crate::coco_struct::{ClassInfo, MethodInfo};
use regex::Regex;
use std::collections::HashMap;

pub struct CtagsParser {
    pub class_map: HashMap<String, ClassInfo>,
    pub classes: Vec<ClassInfo>,
}

impl Default for CtagsParser {
    fn default() -> Self {
        CtagsParser {
            class_map: Default::default(),
            classes: vec![],
        }
    }
}

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(
        r"(?x)
^(?P<class_name>[A-Za-z0-9_]+)
\t(?P<file_name>([^\t]+))
\t([^\t]+)\tclass"
    )
    .unwrap();
    static ref INHERITS_RE: Regex = Regex::new(r"inherits:([A-Za-z0-9_\:,]+)").unwrap();
    static ref AVAILABLE_RE: Regex = Regex::new(
        r"(?x)
^(?P<name>[A-Za-z0-9_]+)
\t(?P<data_type>[^\t]+)
\t([^\t]+)
\t(?P<tag_type>[A-Za-z]+)"
    )
    .unwrap();
    static ref RE_CLASS: Regex = Regex::new(
        r"(?x)
class:(?P<class_name>[A-Za-z0-9_\.]+)"
    )
    .unwrap();
}

impl CtagsParser {
    pub fn parse_class(&mut self, str: &str) {
        if let Some(captures) = CLASS_RE.captures(str) {
            let class_name = &captures["class_name"];
            let clazz = ClassInfo::new(class_name);

            self.class_map.insert(class_name.to_string(), clazz);
        }
    }
    pub fn parse_method_methods(&mut self, str: &str) {
        if !AVAILABLE_RE.is_match(str) {
            return;
        }

        let captures = AVAILABLE_RE.captures(str).unwrap();

        let mut class_name = "".to_string();
        if let Some(capts) = RE_CLASS.captures(str) {
            class_name = capts["class_name"].to_string();
        }
        let split = class_name.split(".");
        if let Some(last) = split.last() {
            class_name = last.to_string();
        }

        if class_name.len() <= 0 {
            return;
        }
        let clazz;
        match self.class_map.get_mut(&*class_name) {
            Some(clz) => {
                clazz = clz;
            }
            None => {
                return;
            }
        };

        let name = &captures["name"];
        let tag_type = &captures["tag_type"];
        if tag_type.eq("method") {
            let method = MethodInfo::new(name);
            clazz.method.push(method);
        }
    }

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

    #[test]
    pub fn should_parse_java_method() {
        let class_tags = "AsyncEventBus	AsyncEventBus.java	/^public class AsyncEventBus extends EventBus {$/;\"	class	line:31	language:Java	inherits:EventBus";
        let tags = "AsyncEventBus	AsyncEventBus.java	/^  public AsyncEventBus(Executor executor) {$/;\"	method	line:69	language:Java	class:AsyncEventBus	access:public";
        let mut parser = CtagsParser::default();
        parser.parse_class(class_tags);
        parser.parse_method_methods(tags);

        let classes = parser.classes();
        let methods = &classes[0].method;
        assert_eq!(1, methods.len());
        assert_eq!("AsyncEventBus", methods[0].name);
    }
}
