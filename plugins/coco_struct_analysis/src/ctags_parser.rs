use crate::coco_struct::{ClassInfo, MethodInfo};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct CtagsParser {
    class_map: HashMap<String, ClassInfo>,
}

impl Default for CtagsParser {
    fn default() -> Self {
        CtagsParser {
            class_map: Default::default(),
        }
    }
}

lazy_static! {
    static ref CLASS_RE: Regex = Regex::new(
        r"(?x)
^(?P<class_name>[A-Za-z0-9_]+)
\t(?P<file_name>([^\t]+))
\t([^\t]+)\t[class|struct]"
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
    pub fn parse(path: &str) -> CtagsParser {
        let file = File::open(path).expect("cannot find file");
        let reader = BufReader::new(file);

        let mut parser = CtagsParser::default();
        for result in reader.lines() {
            match result {
                Ok(line) => {
                    parser.parse_class(line.as_str());
                }
                Err(_) => {}
            };
        }

        parser
    }

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
    use std::path::PathBuf;

    pub fn tags_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let ctags_dir = root_dir.clone().join("_fixtures").join("ctags");

        return ctags_dir;
    }

    #[test]
    pub fn should_parse_local_file() {
        let dir = tags_dir().join("coco_tags");
        let parser = CtagsParser::parse(format!("{}", dir.display()).as_str());
        let vec = parser.classes();
        assert_eq!(25, vec.len());
    }

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
