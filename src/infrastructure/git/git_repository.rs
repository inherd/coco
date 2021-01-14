use std::path::{Path, PathBuf};

use git2::Repository;
use tempdir::TempDir;
use url::Url;

pub struct GitRepository {}

impl GitRepository {
    pub fn clone(url: &str) -> Repository {
        let root = Path::new(".coco");
        let uri_path = match Url::parse(url) {
            Ok(url) => url,
            Err(e) => panic!("failed to parsed: {}", e),
        };

        let buf = GitRepository::uri_to_path(root, uri_path);

        let path_str = buf.as_path().to_str().unwrap();
        println!("tempdir: {:?}", path_str);
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

    pub fn uri_to_path(root: &Path, uri_path: Url) -> PathBuf {
        let mut buf = root.join(PathBuf::from(uri_path.host().unwrap().to_string()));

        let paths = uri_path
            .path_segments()
            .map(|c| c.collect::<Vec<_>>())
            .unwrap();

        for path in paths {
            buf = buf.join(PathBuf::from(path));
        }

        buf
    }

    #[allow(dead_code)]
    fn create_temp_dir() -> PathBuf {
        let temp = TempDir::new_in(".coco", "");
        let dir = match temp {
            Ok(tempdir) => tempdir.into_path(),
            Err(e) => panic!("failed to create dir: {}", e),
        };
        dir
    }
}
