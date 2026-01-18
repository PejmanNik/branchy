use anyhow::{Context, Error};
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::cli::ConfigKey;
use crate::config::get_config;

pub fn get_worktree_path(repo: &git2::Repository) -> Result<PathBuf, Error> {
    let repo_path = repo
        .workdir()
        .context("Failed to get repository working directory")?;
    let default_path = repo_path.join("worktree");

    let mut path = get_config(repo, &ConfigKey::BasePath)?
        .filter(|s| !s.trim().is_empty())
        .map(|s| PathBuf::from(s))
        .unwrap_or(default_path);

    // worktree is inside the repo directory or not
    let is_sub_dir_of_repo = path.starts_with(repo_path);
    if is_sub_dir_of_repo {
        update_git_ignore(&path, repo_path)?;
    } else {
        // add repo name as subdirectory
        path.push(
            repo_path
                .file_name()
                .context("Failed to get repository directory name")?,
        );
    }

    create_dir_all(&path).context("Failed to create worktree directory")?;
    Ok(path)
}

fn update_git_ignore(path: &PathBuf, repo_path: &Path) -> Result<(), Error> {
    let rule = format!("/{}", path.strip_prefix(repo_path)?.to_string_lossy());
    let gitignore_path = repo_path.join(".gitignore");

    let mut content = String::new();
    if gitignore_path.exists() {
        content = std::fs::read_to_string(&gitignore_path)?;
    }

    if !content.contains(&rule) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&gitignore_path)?;

        writeln!(file, "{}", rule)?;
    }

    Ok(())
}
