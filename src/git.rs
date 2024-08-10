use git2::Repository;

pub fn get_repo() -> Repository  {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open git repository: {}", e),
    };

    repo
}