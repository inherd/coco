pub fn from_url(url: &str) -> String {
    return url.replace("/", "-").replace(".", "-");
}

#[cfg(test)]
mod test {
    use crate::infrastructure::name_format::from_url;

    #[test]
    fn format_github() {
        let string = from_url("github.com/inherd/coco.fixtures");
        assert_eq!("github-com-inherd-coco-fixtures", string);
    }
}
