use std::path::Path;

use git2::Repository;

use crate::infrastructure::url_format;

pub struct GitRepository {}

impl GitRepository {
    pub fn open(url: &str) -> Repository {
        let buf = url_format::uri_to_path(url);

        let path_str = buf.as_path().to_str().unwrap();
        println!("target dir: {:?}", path_str);
        if buf.exists() {
            // todo: make update for repo
            println!("todo: make update for repo");
            let repo = match Repository::open(Path::new(path_str)) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {}", e),
            };

            return repo;
        };

        // for windows https://github.com/rust-lang/git2-rs/issues/475
        let repo = match Repository::clone(url, Path::new(path_str)) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };

        return repo;
    }
}
