use crate::domain::git::coco_tag::CocoTag;
use crate::infrastructure::git::cmd_git;
use crate::infrastructure::git::git_tag_parser::GitTagParser;
use core_model::url_format;

pub fn analysis(url: &str) -> Vec<CocoTag> {
    let local_path = url_format::uri_to_path(url);

    let messages = cmd_git::tags(Some(format!("{}", local_path.display())));
    let results = GitTagParser::parse(messages.as_str());

    return results;
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn local_project_test() {
        let tags = analysis(".");
        println!("{:?}", tags);
        assert!(tags.len() >= 2);
    }
}
