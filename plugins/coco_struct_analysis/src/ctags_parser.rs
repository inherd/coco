use crate::coco_struct::{ClassInfo, MethodInfo};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

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
        r#"(?x)
^(?P<name>[A-Za-z0-9_]+)
\t(?P<data_type>[^\t]+)
\t([^\t]+.*?")
\t(?P<tag_type>[A-Za-z]+)"#
    )
    .unwrap();
    static ref RE_CLASS: Regex = Regex::new(r"class:(?P<class_name>[A-Za-z0-9_\.]+)").unwrap();
    static ref RE_ACCESS: Regex = Regex::new(r"access:(?P<access>[A-Za-z0-9_]+)").unwrap();
    static ref RE_LANGUAGE: Regex = Regex::new(r"language:(?P<language>[A-Za-z0-9_\#]+)").unwrap();
    static ref RE_TYPE: Regex =
        Regex::new(r"/\^([ ]*)(?P<datatype>[A-Za-z0-9_.]+)([^A-Za-z0-9_]+)(.*)\$/").unwrap();
    static ref TYPE_KEYWORDS: [&'static str; 18] = [
        "private",
        "public",
        "protected",
        "static",
        "volatile",
        "synchronized",
        "final",
        "const",
        "abstract",
        "struct",
        "union",
        "enum",
        "override",
        "internal",
        "extern",
        "readonly",
        "*",
        ":",
    ];
}

impl CtagsParser {
    pub fn parse_str(str: &str) -> CtagsParser {
        let mut parser = CtagsParser::default();
        let split = str.split("\n");
        for line in split.clone() {
            parser.parse_class(line);
        }

        for line in split {
            println!("{}", line);
            parser.parse_method_methods(line);
        }

        parser
    }

    pub fn parse(dir: PathBuf) -> CtagsParser {
        let file = File::open(format!("{}", dir.display()).as_str()).expect("cannot find file");
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

        let file = File::open(format!("{}", dir.display()).as_str()).expect("cannot find file");
        let reader = BufReader::new(file);
        for result in reader.lines() {
            match result {
                Ok(line) => {
                    parser.parse_method_methods(line.as_str());
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

    pub fn parse_method_methods(&mut self, line: &str) {
        if !AVAILABLE_RE.is_match(line) {
            return;
        }

        let captures = AVAILABLE_RE.captures(line).unwrap();

        let clazz;
        match self.lookup_class_from_map(line) {
            None => return,
            Some(clz) => clazz = clz,
        }

        let name = &captures["name"];
        let tag_type = &captures["tag_type"];

        let mut access = "";
        if let Some(capts) = RE_ACCESS.captures(line) {
            let match_access = &capts["access"];
            match match_access {
                "public" => access = "+",
                "private" => access = "-",
                "protected" => access = "#",
                &_ => {}
            }
        }

        let lang_capts = RE_LANGUAGE.captures(line).unwrap();
        let language = &lang_capts["language"];

        let without_keywords = CtagsParser::remove_keywords(line.to_string());
        let match_type = RE_TYPE.captures(without_keywords.as_str());

        let mut data_type = "".to_string();
        if match_type.is_some() && (CtagsParser::datatype_supported(language)) {
            let capts = match_type.unwrap();
            data_type = (&capts["datatype"]).to_string();
        }

        if tag_type.eq("method") {
            let method = MethodInfo::new(name, access, data_type);
            clazz.method.push(method);
        }
    }

    pub fn datatype_supported(lang: &str) -> bool {
        return lang == "C++" || lang == "C#" || lang == "Java";
    }

    pub fn remove_keywords(mut line: String) -> String {
        for keyword in TYPE_KEYWORDS.iter() {
            line = line.replacen(keyword, "", 1)
        }

        return line;
    }

    fn lookup_class_from_map(&mut self, line: &str) -> Option<&mut ClassInfo> {
        let mut class_name = "".to_string();
        if let Some(capts) = RE_CLASS.captures(line) {
            class_name = capts["class_name"].to_string();
        }
        let split = class_name.split(".");
        if let Some(last) = split.last() {
            class_name = last.to_string();
        }

        if class_name.len() <= 0 {
            return None;
        }

        let clazz: &mut ClassInfo;
        match self.class_map.get_mut(&*class_name) {
            Some(clz) => {
                clazz = clz;
            }
            None => {
                return None;
            }
        };

        Some(clazz)
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
        let parser = CtagsParser::parse(dir);
        let vec = parser.classes();
        assert_eq!(25, vec.len());
    }

    #[test]
    pub fn should_replace_keyword() {
        assert_eq!("", CtagsParser::remove_keywords("public".to_string()));
    }

    #[test]
    pub fn should_get_return_type() {
        let str = "MethodIdentifier	SubscriberRegistry.java	/^    MethodIdentifier(Method method) {$/;\"	method	line:239	language:Java	class:SubscriberRegistry.MethodIdentifier	access:default
MethodIdentifier	SubscriberRegistry.java	/^  private static final class MethodIdentifier {$/;\"	class	line:234	language:Java	class:SubscriberRegistry	access:private";

        let parser = CtagsParser::parse_str(str);
        let classes = parser.classes();

        assert_eq!(1, classes.len());
        let first_method = classes[0].method[0].clone();
        assert_eq!("MethodIdentifier", first_method.return_type);
    }

    #[test]
    pub fn should_parse_java_file() {
        let dir = tags_dir().join("java_tags");
        let parser = CtagsParser::parse(dir);
        let classes = parser.classes();
        assert_eq!(1, classes.len());
        assert_eq!(9, classes[0].method.len());

        let first_method = classes[0].method[0].clone();
        assert_eq!("TypeName", first_method.name);
        assert_eq!("-", first_method.access)
    }
}
