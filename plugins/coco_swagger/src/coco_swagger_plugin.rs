extern crate openapi;

use self::openapi::OpenApi;
use std::path::Path;

pub fn analysis(path: &Path) -> openapi::Result<OpenApi> {
    openapi::from_path(path)
}

#[cfg(test)]
mod tests {
    use super::openapi::OpenApi;
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
        let result = analysis(&*swagger_dir()).unwrap();
        if let OpenApi::V3_0(spec) = result {
            let url = &spec.servers.unwrap()[0].url;
            assert_eq!("http://petstore.swagger.io/v1", url)
        }
    }
}
