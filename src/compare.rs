use std::collections::HashSet;

use anyhow::{Context, Result};
use git2::{Commit, Oid, Repository, Sort};

pub fn run(repo: &Repository, target: String, base: Option<String>) -> Result<()> {
    let base = base.unwrap_or(String::from("HEAD"));

    let base_commit = repo
        .revparse_single(&base)
        .with_context(|| format!("specified base ref '{}' does not exist", base))?
        .peel_to_commit()
        .with_context(|| format!("'{}' does not resolve to a commit", base))?;
    let target_commit = repo
        .revparse_single(&target)
        .with_context(|| format!("specified target ref '{}' does not exist", target))?
        .peel_to_commit()
        .with_context(|| format!("'{}' does not resolve to a commit", base))?;

    let merge_base = repo.merge_base(base_commit.id(), target_commit.id()).ok();

    // collect target's change-ids
    let mut target_change_ids: HashSet<String> = HashSet::new();
    walk_history(
        repo,
        target_commit.id(),
        merge_base,
        |_commit, change_id| {
            target_change_ids.insert(change_id.to_string());
            Ok(())
        },
    )?;

    // walk base and print anything whose change-id is not in target set
    walk_history(repo, base_commit.id(), merge_base, |commit, change_id| {
        if target_change_ids.contains(change_id) {
            let summary = commit.summary()?.unwrap_or("");
            println!("{:.7}\t{}", commit.id(), summary);
        }
        Ok(())
    })?;
    Ok(())
}

fn walk_history(
    repo: &Repository,
    start: Oid,
    hide: Option<Oid>,
    mut callback: impl FnMut(&Commit, &str) -> Result<()>,
) -> Result<()> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push(start)?;
    if let Some(hide) = hide {
        revwalk.hide(hide)?;
    }
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME)?;
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let message = commit
            .message()
            .context("commit message is not valid utf-8")?;

        if let Some(change_id) = extract_change_id(message) {
            callback(&commit, change_id)?;
        }
    }

    Ok(())
}

fn extract_change_id(msg: &str) -> Option<&str> {
    msg.lines()
        .find_map(|line| line.strip_prefix("CHANGE-ID: "))
}
