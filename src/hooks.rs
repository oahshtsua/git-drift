use anyhow::Result;
use git2::Repository;
use std::path::PathBuf;

pub fn init(repo: &Repository) -> Result<()> {
    todo!("Initialize git-drift commit message hook in the repository");
}

pub fn inject_change_id(message_file: PathBuf) -> Result<()> {
    todo!("Append the changeid to the end of the commit message");
}
