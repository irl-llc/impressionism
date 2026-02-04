//! Command-line argument definitions.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Intelligent skill discovery and context-aware activation for Claude Code.
#[derive(Parser, Debug)]
#[command(name = "impressionism")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize configuration and data directories.
    Init(InitArgs),

    /// Index skills from configured directories.
    Index(IndexArgs),

    /// Select skills for current context (called by hooks).
    Select(SelectArgs),

    /// Log a message or event to history (called by hooks).
    Log(LogArgs),

    /// Show status of skill index.
    Status,
}

/// Arguments for the init command.
#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Only initialize if not already initialized.
    #[arg(long)]
    pub if_needed: bool,
}

/// Arguments for the index command.
#[derive(Parser, Debug)]
pub struct IndexArgs {
    /// Force re-indexing of all skills, ignoring file hashes.
    #[arg(long)]
    pub force: bool,

    /// Quick index: only check for new/modified files since last index.
    #[arg(long)]
    pub quick: bool,
}

/// Arguments for the select command.
#[derive(Parser, Debug)]
pub struct SelectArgs {
    /// Session ID for tracking context.
    #[arg(long)]
    pub session: String,

    /// Workspace path for project-specific skills.
    #[arg(long)]
    pub workspace: PathBuf,

    /// Only evaluate deactivation rules, skip activation.
    #[arg(long)]
    pub deactivate_only: bool,
}

/// Arguments for the log command.
#[derive(Parser, Debug)]
pub struct LogArgs {
    /// Session ID for tracking context.
    #[arg(long)]
    pub session: String,

    /// Event type being logged.
    #[arg(long)]
    pub event: EventType,
}

/// Event types that can be logged.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum EventType {
    /// User submitted a prompt.
    UserPromptSubmit,
    /// Tool was used by Claude.
    PostToolUse,
    /// Session is stopping.
    Stop,
}
