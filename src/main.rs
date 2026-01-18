use anyhow::{Context, Error};
use clap::Parser;
use colored::Colorize;
use git2::Repository;
use std::process::ExitCode;
use self_update::{cargo_crate_version};

use crate::cli::{Cli, Commands, ConfigAction};
use crate::config::{get_all_configs, set_config, unset_config};
use crate::prune::safe_prune_worktrees;

mod cli;
mod config;
mod create;
mod prune;
mod sub_shell;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Error> {
    let repo: Repository = Repository::discover(".")
        .context("Error: Not a git repository! Please run this command inside a git project.")?;

    let cli = Cli::parse();
    match &cli.command {
        Commands::Go(options) => {
            let work_tree_path = create::get_or_create_worktree_path(&repo, &options)?;
            sub_shell::open_sub_shell(&work_tree_path)?;
        }
        Commands::Create(options) => {
            let work_tree_path = create::get_or_create_worktree_path(&repo, &options)?;
            println!(
                "{} Created worktree at {}",
                "✓".green().bold(),
                work_tree_path.display()
            );
        }
        Commands::Config { action } => {
            run_config_action(&repo, action)?;
        }
        Commands::SelfUpdate {} => {
            update()?;
        }
        Commands::Prune(options) => {
            safe_prune_worktrees(&repo, options)?;
        }
    };

    Ok(())
}

fn run_config_action(repo: &git2::Repository, action: &ConfigAction) -> Result<(), Error> {
    match action {
        ConfigAction::Set { key, value, global } => {
            set_config(repo, key, value, *global)?;
        }
        ConfigAction::Unset { key, global } => {
            unset_config(repo, key, *global)?;
        }
        ConfigAction::GetAll {} => {
            for entry in get_all_configs(repo)? {
                println!("{}", entry);
            }
        }
    }

    Ok(())
}

fn update() -> Result<(), Error> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("PejmanNik")
        .repo_name("branchy")
        .bin_name("by")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .no_confirm(true)
        .build()?
        .update()
        .context("Failed to update wt")?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}