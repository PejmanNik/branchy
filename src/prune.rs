use anyhow::Error;

use crate::cli::PruneOptions;

mod eligibility;
mod log;
mod safe_prune;

pub fn safe_prune_worktrees(repo: &git2::Repository, options: &PruneOptions) -> Result<(), Error> {
    return eligibility::find_eligible_worktrees(repo, options)?
        .iter_mut()
        .map(|eligibility| safe_prune::safe_prune_worktree(eligibility, options))
        .collect();
}
