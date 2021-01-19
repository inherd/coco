use tokei::{Config, LanguageType, Languages};

pub fn by_dir() {
    let paths = &["src", "tests"];
    let excluded = &["target"];

    let config = Config::default();

    let mut languages = Languages::new();

    languages.get_statistics(paths, excluded, &config);
    let rust = &languages[&LanguageType::Rust];

    println!("{:?}", languages);
    println!("{:?}", rust);
}

#[cfg(test)]
mod test {
    use crate::infrastructure::cloc::by_dir;

    #[test]
    fn should_cloc_in_dir() {
        by_dir();
    }
}
