use std::path::PathBuf;

pub fn expand(path: &str) -> PathBuf {
    let input = &PathBuf::from(path).display().to_string();
    let path = shellexpand::tilde(input);
    return PathBuf::from(path.to_string());
}

#[cfg(test)]
mod test {
    use crate::support::path_format::expand;

    #[test]
    fn format_path() {
        let string = expand("~");
        assert_ne!(string.display().to_string(), "~");
    }
}
