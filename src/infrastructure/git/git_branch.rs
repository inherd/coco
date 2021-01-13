use crate::domain::git::branch::CocoBranch;
use git2::Repository;

pub struct GitBranch {}

impl GitBranch {
    pub fn list(repo: Repository) -> Vec<CocoBranch> {
        let branches = repo.branches(None).unwrap();
        let mut coco_branches = vec![];
        for x in branches {
            let br = x.unwrap().0;
            // todo: add branch type support
            // let br_type: BranchType = x.unwrap().1;
            let branch_name = br.name().unwrap().unwrap();

            let branch = GitBranch::create_branch_by_name(&repo, branch_name);

            coco_branches.push(branch);
        }

        coco_branches
    }

    fn create_branch_by_name(repo: &Repository, branch_name: &str) -> CocoBranch {
        let mut branch = CocoBranch::new(branch_name);
        let oid = repo.revparse_single(branch_name).unwrap().id();

        let mut walk = repo.revwalk().unwrap();
        let _re = walk.push(oid);

        let mut walk_iter = walk.into_iter();

        let last_id = walk_iter.next().unwrap().unwrap();
        let last_commit = repo.find_commit(last_id).unwrap();

        branch.last_commit_date = last_commit.author().when().seconds();

        while let Some(oid_result) = walk_iter.next() {
            if walk_iter.next().is_none() {
                let first_commit = repo.find_commit(oid_result.unwrap()).unwrap();

                branch.author = first_commit.author().name().unwrap().to_string();
                branch.committer = first_commit.committer().name().unwrap().to_string();
                branch.first_commit_date = first_commit.author().when().seconds();
            }
        }

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
}
