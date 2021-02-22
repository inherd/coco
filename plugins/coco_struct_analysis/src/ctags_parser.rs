use crate::coco_struct::ClassInfo;
use regex::Regex;
use std::collections::HashMap;

pub struct CtagsParser {
    pub class_map: HashMap<String, ClassInfo>,
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

impl CtagsParser {
    pub fn parse_class(str: &str) -> Option<ClassInfo> {
        if let Some(captures) = CLASS_RE.captures(str) {
            let class_name = &captures["class_name"];
            let clazz = ClassInfo::new(class_name);
            println!("{}", class_name);

            return Some(clazz);
        }

        None
    }
    pub fn parse_method_methods() {}
}

#[cfg(test)]
mod test {
    use crate::ctags_parser::CtagsParser;

    #[test]
    pub fn should_parse_java_class() {
        let tags = "AsyncEventBus	AsyncEventBus.java	/^public class AsyncEventBus extends EventBus {$/;\"	class	line:31	language:Java	inherits:EventBus";
        let clazz = CtagsParser::parse_class(tags).unwrap();
        assert_eq!("AsyncEventBus", clazz.name);
    }
}
