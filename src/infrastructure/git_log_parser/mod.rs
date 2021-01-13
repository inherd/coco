use git2::Repository;
use std::path::PathBuf;
use tempdir::TempDir;

pub struct GitLogParser {}

impl GitLogParser {
    pub fn clone(url: &str) -> Repository {
        let dir = PathBuf::from(".coco");
        println!("tempdir: {:?}", dir.clone());
        let repo = match Repository::clone(url, dir) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };

        return repo;
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

    #[test]
    fn should_support_clone() {
        let repo = GitLogParser::clone("https://github.com/phodal/coco.fixtures");
        let result = repo.revparse("master");
        assert!(result.is_ok());
    }
}
