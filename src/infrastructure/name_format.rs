pub fn from_url(url: &str) -> String {
    let url_str = url
        .replace("https://", "")
        .replace("http://", "")
        .replace("/", "-")
        .replace(".", "-");
    return format!("{}.{}", url_str, "json");
}

#[cfg(test)]
mod test {
    use crate::infrastructure::name_format::from_url;

    #[test]
    fn format_github() {
        let string = from_url("github.com/inherd/coco.fixtures");
        assert_eq!("github-com-inherd-coco-fixtures.json", string);
    }

    #[test]
    fn format_github_with_url_https() {
        let string = from_url("https://github.com/inherd/coco.fixtures");
        assert_eq!("github-com-inherd-coco-fixtures.json", string);
    }

    #[test]
    fn format_github_with_url_http() {
        let string = from_url("http://github.com/inherd/coco.fixtures");
        assert_eq!("github-com-inherd-coco-fixtures.json", string);
    }
}
