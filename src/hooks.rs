use anyhow::{Context, Result, bail};
use git2::Repository;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

const HOOK_MARKER: &str = "# Installed by git-drift; do not edit directly.";

fn commit_msg_hook_path(repo: &Repository) -> PathBuf {
    repo.path().join("hooks").join("commit-msg")
}

pub fn is_initialized(repo: &Repository) -> Result<bool> {
    let hook_path = commit_msg_hook_path(repo);
    if !hook_path.exists() {
        return Ok(false);
    }

    let contents = fs::read_to_string(&hook_path)
        .with_context(|| format!("failed to read existing hook at {}", hook_path.display()))?;
    Ok(contents.contains(HOOK_MARKER))
}

fn write_commitmsg_hook(hook_path: &Path, binary: &Path) -> Result<()> {
    let script = format!(
        "#!/bin/sh\n{}\nexec {:?} commit-msg \"$@\"\n",
        HOOK_MARKER, binary,
    );

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o755)
        .open(hook_path)?;
    file.write_all(script.as_bytes())?;
    Ok(())
}

pub fn init(repo: &Repository) -> Result<()> {
    if is_initialized(repo)? {
        println!("git-drift is already initialized in this repository.");
        return Ok(());
    }

    let hook_path = commit_msg_hook_path(repo);
    let binary = env::current_exe().context("failed to resolve path to the git-drift binary")?;
    if hook_path.exists() {
        bail!(
            "a commit-msg hook already exists at {}; refusing to overwrite it.\n\
             Add this line to it manually:\n  exec {} commit-msg \"$@\"",
            hook_path.display(),
            binary.display(),
        )
    }

    let hooks_dir = hook_path.parent().expect("hook_path always has a parent");
    fs::create_dir_all(hooks_dir).with_context(|| {
        format!(
            "failed to create hooks directory at {}",
            hooks_dir.display()
        )
    })?;
    write_commitmsg_hook(&hook_path, &binary)
        .with_context(|| format!("failed to write hooks file at {}", hook_path.display()))?;

    Ok(())
}

pub fn inject_change_id(message_file: PathBuf) -> Result<()> {
    todo!("Append the changeid to the end of the commit message");
}
