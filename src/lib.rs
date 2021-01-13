pub mod app;
pub mod domain;
pub mod infrastructure;

#[cfg(test)]
mod test {
    use git2::{Error, Revspec};
    use std::path::PathBuf;

    #[test]
    fn should_open_repo() {
        use git2::Repository;

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let repo = match Repository::open(path) {
            Ok(repo) => {
                let result = repo.revparse("master");
                match result {
                    Ok(so) => {
                        println!("{:?}", so.mode());
                        println!("{:?}", so.from());
                        println!("{:?}", so.to());

                        let commit = repo.find_commit(so.from().unwrap().id());
                        println!("{:?}", commit);
                    }
                    Err(_) => {}
                }

                let branches = repo.branches(None).unwrap();
                // println!("{:?}", branches)
                for x in branches {
                    let branch = x.unwrap().0;
                    println!("{:?}", branch.name());
                }
            }
            Err(e) => panic!("failed to open: {}", e),
        };
    }
}
