use std::path::{Path, PathBuf};

use url::Url;

use crate::settings::Settings;

pub fn from(url: &str) -> String {
    let url_str = url
        .replace("https://", "")
        .replace("http://", "")
        .replace("/", "-")
        .replace(".", "-");

    return format!("{}.{}", url_str, "json");
}

pub fn uri_to_path(url: &str) -> PathBuf {
    let uri_path = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => panic!("failed to parsed: {}", e),
    };

    let root = Path::new(Settings::root_dir());
    let mut buf = root.join(PathBuf::from(uri_path.host().unwrap().to_string()));

    let paths = uri_path
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();

    for path in paths {
        buf = buf.join(PathBuf::from(path));
    }

    buf
}

#[cfg(test)]
mod test {
    use crate::infrastructure::url_format::{from, uri_to_path};

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

    #[test]
    fn url_to_path() {
        let string = uri_to_path("http://github.com/coco-rs/coco.fixtures");
        assert_eq!(
            ".coco/github.com/coco-rs/coco.fixtures",
            string.to_str().unwrap()
        );
    }
}
