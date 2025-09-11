use anyhow::Result;
use git2::Repository;

pub fn run(repo: &Repository, target: String, base: Option<String>) -> Result<()> {
    todo!("Implement branch comparison based on change-id");
}
