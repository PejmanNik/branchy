use colored::Colorize;

use super::eligibility::{PruneEligibility, PruneReason};

pub fn get_worktree_info(eligibility: &PruneEligibility) -> String {
    match &eligibility.branch {
        Some(branch) => format!(
            "worktree {} and branch {}",
            eligibility.worktree_name.bright_black(),
            branch.branch_name.bright_black()
        ),
        None => format!("worktree {}", eligibility.worktree_name.bright_black()),
    }
}

pub fn prune_reason_to_string(reason: &Option<PruneReason>) -> String {
    match reason {
        Some(PruneReason::DirtyWorktree) => "Worktree has uncommitted changes.".into(),
        Some(PruneReason::UnpushedCommits) => "Branch has unpushed commits.".into(),
        None => "".into(),
    }
}
