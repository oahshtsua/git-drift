# git-drift

Track branch drift across cherry-picks.

## Why

This came out of working at a startup where independent devs each cherry-picked
their own bugfixes straight to prod. By the time we synced branches, `main` and
`prod` had the same fixes under different commit hashes, and hash-based diffing
(`git log a..b`) reported drift that didn't actually exist. syncing became a
pain in the ass. The inspiration for the implementation came from Gerrit solves
this with a `Change-Id` trailer that survives rebases and cherry-picks.

## How it works

1. `git-drift init` installs a `commit-msg` hook that appends a stable
    `CHANGE-ID` trailer to every commit, allowing commits to be tracked across
    cherry-picks and rebases even when their hashes change.
2. `git-drift compare <target> [base]` matches commits between two refs by
    `CHANGE-ID` instead of hash.

## Usage

```sh
git-drift init
git-drift compare staging main
git-drift compare main   # base defaults to HEAD
```
