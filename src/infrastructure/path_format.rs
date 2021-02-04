use std::path::Path;

pub fn expand<P: AsRef<Path>>(p: P) -> String {
    shellexpand::tilde(p).to_string()
}
