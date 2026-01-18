use anyhow::{Context, Error};
use git2::Repository;

use crate::cli::WorktreeOptions;

pub fn get_or_create_branch<'repo>(
    repo: &'repo Repository,
    options: &WorktreeOptions,
) -> Result<git2::Branch<'repo>, Error> {
    if let Some(branch) = find_branch(repo, &options.name) {
        return Ok(branch);
    }
    if options.no_create {
        return Err(anyhow::anyhow!("Branch '{}' does not exist", options.name));
    }

    create_branch(repo, &options.name, &options.track)
}

fn find_branch<'repo>(repo: &'repo Repository, name: &str) -> Option<git2::Branch<'repo>> {
    repo.find_branch(name, git2::BranchType::Local)
        .or_else(|_| repo.find_branch(&format!("origin/{}", name), git2::BranchType::Remote))
        .ok()
}

fn create_branch<'repo>(
    repo: &'repo Repository,
    name: &str,
    track: &Option<String>,
) -> Result<git2::Branch<'repo>, Error> {
    let head = repo
        .head()
        .context("'HEAD' is not a commit and a branch cannot be created from it")?;

    let commit = head.peel_to_commit()?;

    let mut branch = repo.branch(&name, &commit, false)?;
    if let Some(upstream) = track {
        let remote_branch_ref = format!("refs/remotes/{}", upstream);
        branch.set_upstream(Some(&remote_branch_ref))?;
    }

    Ok(branch)
}
