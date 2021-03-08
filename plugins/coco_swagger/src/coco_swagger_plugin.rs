extern crate openapi;

use std::path::Path;

pub fn analysis(path: &Path) {
    match openapi::from_path(path) {
        Ok(spec) => println!("spec: {:?}", spec),
        Err(err) => println!("error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use crate::coco_swagger_plugin::analysis;
    use std::path::PathBuf;

    pub fn swagger_dir() -> PathBuf {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let ctags_dir = root_dir
            .clone()
            .join("_fixtures")
            .join("swagger")
            .join("petstore.yaml");

        return ctags_dir;
    }

    #[test]
    fn should_run_openapi_analysis() {
        analysis(&*swagger_dir());
        // println!("{:?}", spec);
        // assert_eq!("", spec.host.unwrap());
    }
}
