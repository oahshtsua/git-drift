mod cli;
mod compare;
mod hooks;

use anyhow::{Context, Result};
use clap::Parser;
use git2::Repository;

use crate::cli::{Cli, Command};

fn main() -> Result<()> {
    let args = Cli::parse();
    let repo = Repository::discover(".").context("Not in a git repository")?;

    match args.command {
        Command::Init => hooks::init(&repo)?,
        Command::Compare { target, base } => compare::run(&repo, target, base)?,
        Command::CommitMsg { message_file } => hooks::inject_change_id(message_file)?,
    }
    Ok(())
}
