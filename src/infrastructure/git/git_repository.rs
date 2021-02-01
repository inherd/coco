use git2::Repository;

use crate::infrastructure::url_format;

pub struct GitRepository {}

impl GitRepository {
    pub fn open(url: &str) -> Repository {
        let local_path = url_format::uri_to_path(url);

        println!("target dir: {:?}", local_path.display());
        if local_path.exists() {
            // todo: make update for repo
            let repo = match Repository::open(local_path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {}", e),
            };

            return repo;
        };

        // for windows https://github.com/rust-lang/git2-rs/issues/475
        let repo = match Repository::clone(url, local_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };

        return repo;
    }
}
