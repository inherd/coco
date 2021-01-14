use tokei::{Config, Languages};

pub fn count() {
    let paths = &["src", "tests"];
    let excluded = &["target"];

    let config = Config::default();

    let mut languages = Languages::new();

    languages.get_statistics(paths, excluded, &config);
    // let rust = &languages[&LanguageType::Rust];

    println!("{:?}", languages);
}

#[cfg(test)]
mod test {
    use crate::infrastructure::cloc::count;

    #[test]
    fn should_cloc() {
        count();
    }
}
