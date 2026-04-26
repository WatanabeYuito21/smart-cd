mod db;
mod matcher;
mod shell;
mod ui;

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
        Command::Query { keywords } => cmd_query(&keywords),
        Command::Init { shell } => cmd_init(&shell),
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

fn cmd_init(shell: &str) {
    match shell {
        "bash" => print!("{}", shell::init_bash()),
        "powershell" => print!("{}", shell::init_powershell()),
        "cmd" => print!("{}", shell::init_cmd()),
        other => {
            eprintln!("Error: unknown shell '{other}'. Use bash, powershell, or cmd.");
            std::process::exit(1);
        }
    }
}

fn cmd_query(keywords: &[String]) {
    let db = db::Database::load().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    let sorted = db.sorted_entries();
    let kw_refs: Vec<&str> = keywords.iter().map(|s| s.as_str()).collect();
    let matched = matcher::filter(&sorted, &kw_refs);

    if matched.is_empty() {
        std::process::exit(1);
    }

    match ui::select(&matched) {
        Ok(ui::SelectResult::Selected(path)) => println!("{path}"),
        Ok(ui::SelectResult::Cancelled) => std::process::exit(1),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
