pub fn from(url: &str) -> String {
    let url_str = url
        .replace("https://", "")
        .replace("http://", "")
        .replace("/", "-")
        .replace(".", "-");

    return format!("{}.{}", url_str, "json");
}

#[cfg(test)]
mod test {
    use crate::infrastructure::url_format::from;

    #[test]
    fn format_github() {
        let string = from("github.com/coco-rs/coco.fixtures");
        assert_eq!("github-com-coco-rs-coco-fixtures.json", string);
    }

    #[test]
    fn format_github_with_url_https() {
        let string = from("https://github.com/coco-rs/coco.fixtures");
        assert_eq!("github-com-coco-rs-coco-fixtures.json", string);
    }

    #[test]
    fn format_github_with_url_http() {
        let string = from("http://github.com/coco-rs/coco.fixtures");
        assert_eq!("github-com-coco-rs-coco-fixtures.json", string);
    }
}
