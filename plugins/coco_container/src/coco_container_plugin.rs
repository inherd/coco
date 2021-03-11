use dockerfile_parser::Dockerfile;
use std::fs;
use std::path::Path;

pub fn analysis(path: &Path) -> Dockerfile {
    let err_msg = format!("cannot find file: {:?}", path.display());
    let content = fs::read_to_string(path).expect(err_msg.as_str());
    let dockerfile = Dockerfile::parse(content.as_str()).unwrap();

    return dockerfile;
}
