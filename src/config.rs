use anyhow::{Context, Error};
use clap::ValueEnum;

use crate::cli::ConfigKey;

const BASE_KEY: &str = "gt.";

fn get_config_name(key: &ConfigKey) -> Result<String, Error> {
    let possible_value = key
        .to_possible_value()
        .context("Failed to get base configuration key")?;
    let config_key = possible_value.get_name();

    return Ok(format!("{}{}", BASE_KEY, config_key));
}

fn get_git_config(repo: &git2::Repository, global: bool) -> Result<git2::Config, Error> {
    let mut config = repo.config().context("Failed to read git configuration")?;
    if global {
        Ok(config
            .open_global()
            .context("Failed to open global git configuration")?)
    } else {
        Ok(config)
    }
}

pub fn set_config(
    repo: &git2::Repository,
    key: &ConfigKey,
    value: &str,
    global: bool,
) -> Result<(), Error> {
    let mut config = get_git_config(repo, global)?;
    let name = get_config_name(key)?;
    config
        .set_str(&name, value)
        .context("Failed to set configuration value")
}

pub fn unset_config(repo: &git2::Repository, key: &ConfigKey, global: bool) -> Result<(), Error> {
    let mut config = get_git_config(repo, global)?;
    let name = get_config_name(key)?;
    config
        .remove(&name)
        .context("Failed to unset configuration value")
}

pub fn get_config(repo: &git2::Repository, key: &ConfigKey) -> Result<Option<String>, Error> {
    let config = get_git_config(repo, false)?;
    let name = get_config_name(key)?;
    match config.get_string(&name) {
        Ok(value) => Ok(Some(value)),
        Err(e) if e.code() == git2::ErrorCode::NotFound => Ok(None),
        Err(e) => Err(e).context("Failed to read configuration value"),
    }
}

pub fn get_all_configs(repo: &git2::Repository) -> Result<Vec<String>, Error> {
    let config = get_git_config(repo, false)?;
    let mut entries = config
        .entries(Some(&format!("{}*", BASE_KEY)))
        .context("Failed to list configuration entries")?;

    let mut results = Vec::new();
    while let Some(entry) = entries.next() {
        let entry = entry.context("Failed to read configuration entry")?;
        match (entry.name(), entry.value()) {
            (Some(name), Some(value)) => {
                results.push(format!("{}={}", name, value));
            }
            _ => {}
        }
    }

    Ok(results)
}
