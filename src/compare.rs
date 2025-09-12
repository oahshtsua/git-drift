use std::collections::HashSet;

use anyhow::{Context, Result};
use git2::{Oid, Repository, Sort};

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
    let target_change_ids = collect_change_ids(repo, target_commit.id(), merge_base)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push(base_commit.id())?;
    if let Some(merge_base) = merge_base {
        revwalk.hide(merge_base)?;
    }
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::TIME | Sort::REVERSE)?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let message = commit
            .message()
            .context("commit does not have valid utf-8 message")?;
        if let Some(change_id) = extract_change_id(message)
            && !target_change_ids.contains(change_id)
        {
            let summary = commit.summary()?.unwrap_or("");
            println!("{:.7}\t{}", commit.id(), summary);
        }
    }
    Ok(())
}

fn collect_change_ids(repo: &Repository, start: Oid, hide: Option<Oid>) -> Result<HashSet<String>> {
    let mut ids: HashSet<String> = HashSet::new();

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
            ids.insert(change_id.to_string());
        }
    }
    Ok(ids)
}
fn extract_change_id(msg: &str) -> Option<&str> {
    msg.lines()
        .find_map(|line| line.strip_prefix("CHANGE-ID: "))
}
