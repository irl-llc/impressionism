//! Impressionism CLI - Intelligent skill discovery for Claude Code.

use clap::Parser;
use impressionism::cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init(args) => run_init(args),
        Command::Index(args) => run_index(args),
        Command::Select(args) => run_select(args),
        Command::Log(args) => run_log(args),
        Command::Status => run_status(),
    }
}

fn run_init(args: impressionism::cli::args::InitArgs) {
    if args.if_needed {
        println!("Checking if initialization is needed...");
    }
    println!("Initializing impressionism...");
    // TODO: Implement initialization
}

fn run_index(args: impressionism::cli::args::IndexArgs) {
    if args.force {
        println!("Force re-indexing all skills...");
    } else if args.quick {
        println!("Quick indexing (checking for changes)...");
    } else {
        println!("Indexing skills...");
    }
    // TODO: Implement indexing
}

fn run_select(args: impressionism::cli::args::SelectArgs) {
    println!(
        "Selecting skills for session {} in {}",
        args.session,
        args.workspace.display()
    );
    if args.deactivate_only {
        println!("Evaluating deactivation rules only...");
    }
    // TODO: Implement selection
}

fn run_log(args: impressionism::cli::args::LogArgs) {
    println!(
        "Logging event {:?} for session {}",
        args.event, args.session
    );
    // TODO: Implement logging
}

fn run_status() {
    println!("Skill index status:");
    println!("  (not yet implemented)");
    // TODO: Implement status
}
