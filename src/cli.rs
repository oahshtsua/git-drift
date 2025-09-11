use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Track branch drift by CHANGE-ID instead of commit hash, so cherry-picked
/// or rebased commits are still recognized as the same change.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Install the commit-msg hook into this repository
    Init,
    /// Compare two refs by CHANGE-ID
    Compare {
        /// Ref to compare against base
        target: String,
        /// Ref to compare from; defaults to HEAD if omitted
        base: Option<String>,
    },
    /// Invoked by the installed commit-msg hook; not meant to be run by hand
    #[command(hide = true)]
    CommitMsg {
        /// Path to the file holding the edited commit message ($1 from git)
        message_file: PathBuf,
    },
}
