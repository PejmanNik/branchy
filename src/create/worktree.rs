use anyhow::{Context, Error};
use git2::Repository;
use std::path::PathBuf;

use super::branch::get_or_create_branch;
use super::config::get_worktree_path;
use crate::cli::WorktreeOptions;

pub fn get_or_create_worktree_path(
    repo: &Repository,
    options: &WorktreeOptions,
) -> Result<PathBuf, Error> {
    match find_worktree_path(repo, &options.name) {
        Some(p) => Ok(p),
        None => create_worktree(repo, &options),
    }
}

fn find_worktree_path(repo: &Repository, name: &str) -> Option<PathBuf> {
    repo.find_worktree(name)
        .ok()
        .map(|wt| wt.path().to_path_buf())
}

fn create_worktree(repo: &Repository, options: &WorktreeOptions) -> Result<PathBuf, Error> {
    let branch = get_or_create_branch(repo, options)?;
    let worktree_path = get_worktree_path(repo)?.join(&options.name);

    let branch_ref = branch.get();

    let mut opts = git2::WorktreeAddOptions::new();
    opts.checkout_existing(true);
    opts.reference(Some(branch_ref));

    repo.worktree(&options.name, &worktree_path, Some(&mut opts))
        .context("Failed to create worktree")?;

    Ok(worktree_path)
}
