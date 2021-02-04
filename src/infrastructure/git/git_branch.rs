use crate::domain::git::CocoBranch;
use git2::{Commit, Repository, TreeWalkMode, TreeWalkResult};

pub struct GitBranch {}

pub struct SimpleCommit {
    pub author: String,
}

impl GitBranch {
    pub fn list(repo: Repository) -> Vec<CocoBranch> {
        let branches = repo.branches(None).unwrap();
        let mut coco_branches = vec![];
        for x in branches {
            let branch = x.unwrap();
            let br = &branch.0;
            let branch_type = format!("{:?}", &branch.1);

            // todo: add branch type support
            let branch_name = br.name().unwrap().unwrap();
            let branch = GitBranch::calculate_branch(&repo, branch_name, &*branch_type);

            coco_branches.push(branch);
        }

        coco_branches
    }

    fn calculate_branch(repo: &Repository, branch_name: &str, branch_type: &str) -> CocoBranch {
        let mut branch = CocoBranch::new(branch_name);
        let oid = repo.revparse_single(branch_name).unwrap().id();

        let mut walk = repo.revwalk().unwrap();
        let _ = walk.push(oid);

        let mut commit_times = vec![];
        let mut revwalk = walk.into_iter();
        while let Some(oid_result) = revwalk.next() {
            if oid_result.is_err() {
                continue;
            }
            let oid = oid_result.unwrap();
            let commit = repo.find_commit(oid).unwrap();

            commit_times.push(commit.author().when().seconds());
            branch.commits.push(oid.to_string());
        }

        if commit_times.len() <= 0 {
            panic!("not found commits");
        }

        branch.latest_changeset = branch.commits.last().unwrap().to_string();
        branch.last_commit_date = commit_times[0];
        branch.commits_count = commit_times.len();
        branch.first_commit_date = *commit_times.last().unwrap();
        branch.branch_type = branch_type.to_string();

        branch.duration = branch.last_commit_date - branch.first_commit_date;

        branch
    }

    pub fn get(name: &str, repo: Repository) -> Option<CocoBranch> {
        let filter: Vec<CocoBranch> = GitBranch::list(repo)
            .iter()
            .filter(|br| br.name == name)
            .cloned()
            .collect();

        return if filter.len() > 0 {
            Some(filter[0].clone())
        } else {
            None
        };
    }

    pub fn build_changes(commit: &Commit) {
        match commit.tree() {
            Ok(tree) => {
                tree.walk(TreeWalkMode::PreOrder, |_, entry| {
                    println!("{:?}", entry.name().unwrap());
                    TreeWalkResult::Ok
                })
                .unwrap();
            }
            Err(_) => {
                println!()
            }
        }
    }
}
