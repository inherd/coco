use std::path::{Path, PathBuf};

use git2::Repository;
use tempdir::TempDir;
use url::Url;

pub struct GitLogParser {}

impl GitLogParser {
    pub fn clone(url: &str) -> Repository {
        let root = Path::new(".coco");
        let uri_path = match Url::parse(url) {
            Ok(url) => url,
            Err(e) => panic!("failed to parsed: {}", e),
        };

        let buf = GitLogParser::uri_to_path(root, uri_path);

        println!("tempdir: {:?}", buf);
        if buf.exists() {
            // todo: make update for repo
            println!("todo: make update for repo");
            let repo = match Repository::open(buf) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to clone: {}", e),
            };

            return repo;
        };

        let repo = match Repository::clone(url, buf) {
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

#[cfg(test)]
mod test {
    use crate::infrastructure::git_log_parser::GitLogParser;
    use std::path::Path;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            GitLogParser::clone("https://github.com/phodal/coco.fixtures");
        });
    }

    #[test]
    fn should_support_clone() {
        initialize();
        let repo = GitLogParser::clone("https://github.com/phodal/coco.fixtures");
        let result = repo.revparse("master");
        assert!(result.is_ok());
    }

    #[test]
    fn should_verify_github_dir() {
        initialize();
        let repo = GitLogParser::clone("https://github.com/phodal/coco.fixtures");
        let path_str = repo.path().to_str().unwrap();
        let path = Path::new("github.com/phodal/coco.fixtures");
        assert!(path_str.contains(path.to_str().unwrap()));
    }
}
