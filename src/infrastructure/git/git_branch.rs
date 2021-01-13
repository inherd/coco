use crate::domain::git::branch::Branch;
use git2::Repository;

pub struct GitBranch {}

impl GitBranch {
    pub fn list(repo: Repository) -> Vec<Branch> {
        let branches = repo.branches(None).unwrap();
        let mut coco_branches = vec![];
        for x in branches {
            let br = x.unwrap().0;
            // todo: add branch type support
            // let br_type: BranchType = x.unwrap().1;
            let branch_name = br.name().unwrap().unwrap();

            let mut branch = Branch::new(branch_name);
            let result = repo.revparse(branch_name);
            match result {
                Ok(so) => {
                    let find_commit = repo.find_commit(so.from().unwrap().id());
                    let commit = find_commit.unwrap();

                    branch.author = commit.committer().name().unwrap().to_string();
                    branch.date = commit.committer().when().seconds().to_string();
                }
                Err(_) => {}
            }

            coco_branches.push(branch);
        }

        coco_branches
    }

    pub fn get(name: &str, repo: Repository) -> Option<Branch> {
        let filter: Vec<Branch> = GitBranch::list(repo)
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
