use std::io::{Write, stdout};

use crate::cli::PruneOptions;
use anyhow::{Context, Error};
use colored::Colorize;

use super::eligibility::{BranchEligibility, PruneEligibility};
use super::log::{get_worktree_info, prune_reason_to_string};

pub fn safe_prune_worktree(
    eligibility: &mut PruneEligibility,
    options: &PruneOptions,
) -> Result<(), Error> {
    if !eligibility.can_prune && !options.force {
        println!(
            "{} Failed to prune {} - {}",
            "✗".red().bold(),
            get_worktree_info(&eligibility),
            prune_reason_to_string(&eligibility.reason).red()
        );

        return Ok(());
    }

    if options.dry_run {
        println!(
            "{} Would prune {}",
            "○".cyan().bold(),
            get_worktree_info(&eligibility)
        );

        return Ok(());
    }

    print!(
        "{} Pruning {} ",
        "⋯".cyan().bold(),
        get_worktree_info(&eligibility)
    );
    Write::flush(&mut stdout()).context("Failed to flush stdout")?;

    prune_worktree(&eligibility.worktree)?;
    if let Some(branch) = eligibility.branch.as_mut() {
        prune_branch(branch)?;
    }

    println!(
        "\r{} Pruned {} ",
        "✓".green().bold(),
        get_worktree_info(&eligibility)
    );
    return Ok(());
}

fn prune_worktree(wt: &git2::Worktree) -> Result<(), Error> {
    let mut opts = git2::WorktreePruneOptions::new();
    opts.valid(true);
    opts.working_tree(true);
    wt.prune(Some(&mut opts))
        .context("Failed to prune worktree")
}

fn prune_branch(branch: &mut BranchEligibility) -> Result<(), Error> {
    branch.branch.delete().context("Failed to delete branch")
}
