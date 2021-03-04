use crate::infrastructure::git::git_file_history;
use core_model::url_format;
use git_scanner::flare::FlareTreeNode;

pub fn analysis(url: &str) -> FlareTreeNode {
    let local_path = url_format::uri_to_path(url);
    let tree_node = git_file_history::by_path(local_path);

    return tree_node;
}
