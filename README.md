# git-drift

Track branch drift across cherry-picks.

## How it works

1. `git-drift init` installs a `commit-msg` hook that appends a stable `CHANGE-ID` trailer to every commit, allowing commits to be tracked across cherry-picks and rebases even when their hashes change.
2. `git-drift compare <target> [base]` matches commits between two refs by `CHANGE-ID` instead of hash.

## Usage

```sh
git-drift init
git-drift compare staging main
git-drift compare main   # base defaults to HEAD
```
