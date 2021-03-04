use git_scanner::flare::FlareTreeNode;
use git_scanner::git::GitCalculator;
use git_scanner::git_logger::GitLogConfig;
use git_scanner::{file_walker, IndicatorCalculator};
use std::path::PathBuf;

pub fn by_path(root: PathBuf) -> FlareTreeNode {
    let mut tics: Vec<Box<dyn IndicatorCalculator>> = vec![];
    let calculator = Box::new(GitCalculator::new(
        GitLogConfig::default().include_merges(true).since_years(3),
        true,
    ));

    tics.push(calculator);

    let mut tree = file_walker::walk_directory(&root, &mut tics).unwrap();

    for tic in tics {
        if let Some(metadata) = tic.metadata().unwrap() {
            tree.add_data(tic.name() + "_meta", metadata);
        }
    }

    return tree;
}
