use crate::domain::git::branch::Branch;
use git2::Repository;

pub struct GitBranch {}

impl GitBranch {
    pub fn list(repo: Repository) -> Vec<Branch> {
        let branches = repo.branches(None).unwrap();
        let mut coco_branches = vec![];
        for x in branches {
            let branch = x.unwrap().0;
            let branch_name = branch.name().unwrap().unwrap();
            coco_branches.push(Branch::new(branch_name));
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
