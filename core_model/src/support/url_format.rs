use std::path::{Path, PathBuf};

use url::Url;

use crate::Settings;

pub fn json_filename_suffix(text: &str, suffix_str: Option<&str>) -> String {
    filename_suffix(text, suffix_str, Some("json"))
}

pub fn filename_suffix(text: &str, name_suffix: Option<&str>, file_suffix: Option<&str>) -> String {
    let mut filename_suffix = "";
    if let Some(suf) = name_suffix {
        filename_suffix = suf;
    }

    let mut suffix = "";
    if let Some(suf) = file_suffix {
        suffix = suf;
    }

    let uri_path = match Url::parse(text) {
        Ok(url) => url,
        Err(_e) => {
            let path = Path::new(text);
            return match path.file_name() {
                Some(name) => {
                    let filename = name.to_str().unwrap().to_string();
                    format!("{}{}.{}", filename, filename_suffix, suffix)
                }
                None => {
                    format!("default{}.{}", filename_suffix, suffix)
                }
            };
        }
    };

    if uri_path.host().is_none() {
        let pth = PathBuf::from(uri_path.path());
        let last = pth.file_name();

        return format!(
            "{}{}.{}",
            last.unwrap().to_str().unwrap(),
            filename_suffix,
            suffix
        );
    }

    let paths = uri_path
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();

    return format!("{}{}.{}", paths.last().unwrap(), filename_suffix, suffix);
}

pub fn json_filename(text: &str) -> String {
    json_filename_suffix(text, None)
}

pub fn puml_filename(text: &str) -> String {
    filename_suffix(text, None, Some("puml"))
}

pub fn uri_to_path(url: &str) -> PathBuf {
    let uri_path = match Url::parse(url) {
        Ok(url) => url,
        Err(_e) => {
            return PathBuf::from(url);
        }
    };

    let root = Path::new(Settings::root());
    if uri_path.host().is_none() {
        return PathBuf::from(url);
    }
    let mut buf = root.join(PathBuf::from(uri_path.host().unwrap().to_string()));

    let segments = uri_path
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap();

    for path in segments {
        buf = buf.join(PathBuf::from(path));
    }

    buf
}

#[cfg(test)]
mod test {
    use crate::url_format::{json_filename, json_filename_suffix, uri_to_path};
    use std::path::PathBuf;

    #[test]
    fn should_format_github_with_url_http() {
        let string = json_filename("http://github.com/coco-rs/coco.fixtures");
        assert_eq!("coco.fixtures.json", string);
    }

    #[test]
    fn should_format_local_url() {
        let path = PathBuf::from(".coco");
        let framework_path = path.join("framework");

        let string = json_filename(&*format!("{}", framework_path.display()));
        assert_eq!("framework.json", string);
    }

    #[test]
    fn should_format_with_suffix() {
        let path = PathBuf::from(".coco");
        let framework_path = path.join("git");

        let string =
            json_filename_suffix(&*format!("{}", framework_path.display()), Some("-commits"));
        assert_eq!("git-commits.json", string);
    }

    #[test]
    fn should_format_with_default_suffix() {
        let path = PathBuf::from(".");

        let string = json_filename_suffix(&*format!("{}", path.display()), Some("-commits"));
        assert_eq!("default-commits.json", string);
    }

    #[test]
    fn should_url_to_path() {
        let url = "http://github.com/coco-rs/coco.fixtures";
        let string = uri_to_path(url);

        let path = PathBuf::from(".coco");
        let local_url = path
            .join("github.com")
            .join("coco-rs")
            .join("coco.fixtures");

        let local_str = format!("{}", local_url.display());

        assert_eq!(local_str, string.to_str().unwrap());
    }

    #[test]
    fn should_return_origin_when_is_git() {
        let string = uri_to_path(".coco/framework");
        assert_eq!(".coco/framework", string.to_str().unwrap());
    }
}
