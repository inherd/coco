use std::path::{Path, PathBuf};

use url::Url;

use crate::settings::Settings;

pub fn from(url: &str) -> String {
    let uri_path = match Url::parse(url) {
        Ok(url) => url,
        Err(e) => panic!("failed to parsed: {}", e),
    };

    let paths = uri_path
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();

    return format!("{}.{}", paths.last().unwrap(), "json");
}
http://mp.weixin.qq.com/s?__biz=MjM5MzI5NTU3MQ==&mid=2651796733&idx=1&sn=01511abd9bf16493ff17a3e18c0ef913&chksm=bd62a9418a152057d1ba02bbcbbba21ee5ee9b3d44884212d213e75fd5ccb6aa56750880f755&scene=0&xtrack=1#rd
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
    fn format_github_with_url_http() {
        let string = from("http://github.com/coco-rs/coco.fixtures");
        assert_eq!("coco.fixtures.json", string);
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
