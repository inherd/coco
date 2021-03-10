use std::path::{Path, PathBuf};

pub fn expand(path: &Path) -> PathBuf {
    let input = &path.display().to_string();
    let path = shellexpand::tilde(input);
    return PathBuf::from(path.to_string());
}

#[cfg(test)]
mod test {
    use crate::support::path_format::expand;
    use std::path::Path;

    #[test]
    fn format_path() {
        let path = Path::new("~");
        let string = expand(path);
        assert_ne!(string.display().to_string(), "~");
    }
}
