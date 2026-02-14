use anyhow::{Context, Error};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::cli::ConfigKey;
use crate::config::get_config;

pub fn setup_worktree(repo: &git2::Repository) -> Result<PathBuf, Error> {
    let plan = calculate_worktree_plan(repo)?;

    if plan.should_update_exclude {
        let repo_path = repo.workdir().context("Failed to get repository working directory")?;
        update_git_exclude(&plan.path, &repo_path)?;
    }

    create_dir_all(&plan.path).context("Failed to create worktree directory")?;
    Ok(plan.path)
}

struct WorktreePlan {
    pub path: PathBuf,
    pub should_update_exclude: bool,
}

fn calculate_worktree_plan(repo: &git2::Repository) -> Result<WorktreePlan, Error> {
    let repo_path = repo.workdir().context("Failed to get repository working directory")?;
    let default_path = repo_path.join(".worktree");

    let mut path = get_config(repo, &ConfigKey::BasePath)?
        .filter(|s| !s.trim().is_empty())
        .map(|s| PathBuf::from(s))
        .unwrap_or(default_path);

    // worktree is inside the repo directory or not
    let is_sub_dir_of_repo = path.starts_with(repo_path);

    if !is_sub_dir_of_repo {
        // add repo name as subdirectory
        path.push(
            repo_path
                .file_name()
                .context("Failed to get repository directory name")?,
        );
    }

    Ok(WorktreePlan {
        path,
        should_update_exclude: is_sub_dir_of_repo,
    })
}

fn update_git_exclude(path: &PathBuf, repo_path: &Path) -> Result<(), Error> {
    let rule = format!("/{}", path.strip_prefix(repo_path)?.to_string_lossy());
    let gitignore_path = repo_path.join(".git/info/exclude");

    let content = if gitignore_path.exists() {
        std::fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    if !content.contains(&rule) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gitignore_path)?;

        writeln!(file, "{}", rule)?;
    }

    Ok(())
}