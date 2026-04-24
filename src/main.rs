mod db;
mod matcher;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "smart-cd", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { path: String },
    List {
        #[arg(long)]
        paths_only: bool,
    },
    Query {
        keywords: Vec<String>,
    },
    Init { shell: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { path } => cmd_add(&path),
        Command::List { paths_only } => cmd_list(paths_only),
        Command::Query { .. } => {
            eprintln!("query: ui.rs 未実装");
        }
        Command::Init { .. } => {
            eprintln!("init: shell.rs 未実装");
        }
    }
}

fn cmd_add(path: &str) {
    let mut db = db::Database::load().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    db.add(path);
    if let Err(e) = db.save() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn cmd_list(paths_only: bool) {
    let db = db::Database::load().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    for entry in db.sorted_entries() {
        if paths_only {
            println!("{}", entry.path);
        } else {
            println!("{:.2}\t{}", entry.score(), entry.path);
        }
    }
}
