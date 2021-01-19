use crate::domain::git::CocoBranch;
use crate::domain::git::CocoCommit;
use git2::{Commit, Oid, Repository};

pub struct GitBranch {}

pub struct SimpleCommit {
    pub author: String,
}

impl GitBranch {
    pub fn list(repo: Repository) -> Vec<CocoBranch> {
        let branches = repo.branches(None).unwrap();
        let mut coco_branches = vec![];
        for x in branches {
            let br = x.unwrap().0;
            // todo: add branch type support
            let branch_name = br.name().unwrap().unwrap();

            let branch = GitBranch::calculate_branch(&repo, branch_name).0;

            coco_branches.push(branch);
        }

        coco_branches
    }

    fn calculate_branch(repo: &Repository, branch_name: &str) -> (CocoBranch, Vec<CocoCommit>) {
        let mut branch = CocoBranch::new(branch_name);
        let oid = repo.revparse_single(branch_name).unwrap().id();

        let mut walk = repo.revwalk().unwrap();
        let _ = walk.push(oid);

        let mut commits = vec![];
        let mut revwalk = walk.into_iter();
        while let Some(oid_result) = revwalk.next() {
            let oid = oid_result.unwrap();
            let commit = repo.find_commit(oid).unwrap();

            commits.push(GitBranch::convert_commit(branch_name, oid, commit));
        }

        branch.last_commit_date = commits[0].date;

        let last_commit = commits.last().unwrap();

        branch.commits_count = commits.len();
        branch.author = last_commit.author.clone();
        branch.committer = last_commit.committer.clone();
        branch.first_commit_date = last_commit.date.clone();

        branch.duration = branch.last_commit_date - branch.first_commit_date;

        (branch, commits)
    }

    // todo: thinking in refactor to application, is not clean in infrastructure
    fn convert_commit(branch_name: &str, oid: Oid, commit: Commit) -> CocoCommit {
        CocoCommit {
            branch: branch_name.to_string(),
            rev: oid.to_string(),
            author: commit.author().name().unwrap().to_string(),
            committer: commit.committer().name().unwrap().to_string(),
            date: commit.author().when().seconds(),
            message: commit.message().unwrap().to_string(),
            changes: vec![],
        }
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
}
