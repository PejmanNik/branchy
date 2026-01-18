use clap::{Parser, Subcommand, ValueEnum};

#[derive(clap::Args, Debug)]
pub struct PruneOptions {
    /// Perform a dry run without deleting any worktrees
    #[arg(long)]
    pub dry_run: bool,

    /// Force deletion of worktrees even if they have uncommitted changes
    #[arg(long, short = 'f')]
    pub force: bool,

    /// Include worktrees associated with branches
    #[arg(long, short = 'b')]
    pub include_branch: bool,

    /// Filter worktrees by name reqEx pattern
    #[arg(long, short = 'e')]
    pub filter: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct WorktreeOptions {
    // Name of the branch and worktree to open or create
    pub name: String,

    /// optionally track a remote branch
    #[arg(short, long)]
    pub track: Option<String>,

    /// Disable automatic branch creation if it doesn't exist
    #[arg(long, short = 'n')]
    pub no_create: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ConfigKey {
    /// Default path for the worktree root, default is ./worktree inside the repo
    BasePath,
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    name = "mygit",
    about = "A custom git helper CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create if not exists and open the branch in the worktree directory
    Go(WorktreeOptions),
    /// Create a worktree and branch if not exists in the worktree directory
    Create(WorktreeOptions),
    /// Remove clean worktrees
    Prune(PruneOptions),
    /// Configuration settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Update the tool to the latest version
    SelfUpdate {},
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set a configuration value
    Set {
        #[arg(value_enum)]
        key: ConfigKey,

        /// The value to assign to the key
        #[arg()]
        value: String,

        /// Apply setting globally
        #[arg(long)]
        global: bool,
    },
    /// Unset a configuration value
    Unset {
        #[arg(value_enum)]
        key: ConfigKey,

        /// Apply unsetting globally
        #[arg(long)]
        global: bool,
    },
    /// Get all configuration values
    GetAll {},
}
