use anyhow::{Context, Error};
use git2::Worktree;
use regex::Regex;

use crate::cli::PruneOptions;

pub struct BranchEligibility<'repo> {
    pub branch: git2::Branch<'repo>,
    pub branch_name: String,
}

pub struct PruneEligibility<'repo> {
    pub worktree: Worktree,
    pub worktree_name: String,
    pub can_prune: bool,
    pub branch: Option<BranchEligibility<'repo>>,
    pub reason: Option<PruneReason>,
}

pub enum PruneReason {
    DirtyWorktree,
    UnpushedCommits,
}

pub fn find_eligible_worktrees<'repo>(
    repo: &'repo git2::Repository,
    options: &PruneOptions,
) -> Result<Vec<PruneEligibility<'repo>>, Error> {
    let regex_filter = options
        .filter
        .as_ref()
        .map(|f| Regex::new(&f).context("Invalid regex filter"))
        .transpose()?;

    return repo
        .worktrees()?
        .iter()
        .flatten()
        .filter(|wt| regex_filter.as_ref().map_or(true, |re| re.is_match(wt)))
        .map(|wt| can_prune_worktree(repo, wt, options))
        .collect();
}

fn can_prune_worktree<'repo>(
    repo: &'repo git2::Repository,
    name: &str,
    options: &PruneOptions,
) -> Result<PruneEligibility<'repo>, Error> {
    let wt = repo.find_worktree(name)?;
    let wt_repo = git2::Repository::open(wt.path())?;
    let mut result = PruneEligibility {
        worktree: wt,
        worktree_name: name.to_string(),
        branch: None,
        can_prune: options.force,
        reason: None,
    };

    if !options.force && is_worktree_dirty(&wt_repo) {
        result.can_prune = false;
        result.reason = Some(PruneReason::DirtyWorktree);
        return Ok(result);
    }
    if !options.include_branch {
        result.can_prune = true;
        return Ok(result);
    }

    let branch = wt_repo
        .head()
        .context("s")
        .and_then(|head| {
            head.shorthand()
                .map(|f| f.to_string())
                .ok_or(Error::msg("Head has no shorthand"))
        })
        .and_then(|shorthand| {
            repo.find_branch(&shorthand, git2::BranchType::Local)
                .context("can't find branch")
        })?;

    if !options.force && has_unpushed_commits(&wt_repo, &branch) {
        result.reason = Some(PruneReason::UnpushedCommits);
        return Ok(result);
    } else {
        let branch_name = branch
            .name()?
            .ok_or(Error::msg("Branch has no name"))?
            .into();
        result.branch = Some(BranchEligibility {
            branch,
            branch_name,
        });
    }

    result.can_prune = true;
    Ok(result)
}

fn is_worktree_dirty(wt_repo: &git2::Repository) -> bool {
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);

    if let Ok(statuses) = wt_repo.statuses(Some(&mut status_opts)) {
        return !statuses.is_empty();
    }

    false
}

fn has_unpushed_commits(wt_repo: &git2::Repository, local_branch: &git2::Branch<'_>) -> bool {
    let upstream = match local_branch.upstream() {
        Ok(u) => u,
        Err(_) => return true, // No upstream means it's local-only (unpushed)
    };

    let local_oid = local_branch.get().target();
    let upstream_oid = upstream.get().target();

    if local_oid == upstream_oid {
        return false;
    }

    // finds the "merge base" and checks if local has unique commits
    if let (Some(l), Some(u)) = (local_oid, upstream_oid) {
        if let Ok((ahead, _behind)) = wt_repo.graph_ahead_behind(l, u) {
            return ahead > 0;
        }
    }
    true
}
