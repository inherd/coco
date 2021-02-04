use std::path::Path;

pub fn expand<P: AsRef<Path>>(p: P) -> String {
    shellexpand::tilde(&p.as_ref().to_str().unwrap()).to_string()
}

#[cfg(test)]
mod test {
    use crate::infrastructure::path_format::expand;

    #[test]
    fn format_path() {
        let string = expand("~");
        assert_ne!(string, "~");
    }
}
