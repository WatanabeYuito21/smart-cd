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
    Remove { path: String },
    Clean,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Add { path } => cmd_add(&path),
        Command::List { paths_only } => cmd_list(paths_only),
        Command::Query { keywords } => cmd_query(&keywords),
        Command::Init { shell } => cmd_init(&shell),
        Command::Remove { path } => cmd_remove(&path),
        Command::Clean => cmd_clean(),
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
        "fish" => print!("{}", shell::init_fish()),
        other => {
            eprintln!("Error: unknown shell '{other}'. Use bash, powershell, cmd, or fish.");
            std::process::exit(1);
        }
    }
}

fn cmd_remove(path: &str) {
    let mut db = db::Database::load().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    if !db.remove(path) {
        eprintln!("Error: no entry found for '{path}'");
        std::process::exit(1);
    }
    db.save().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    println!("Removed: {path}");
}

fn cmd_clean() {
    let mut db = db::Database::load().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    let count = db.clean();
    if count > 0 {
        db.save().unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        });
    }
    println!("Removed {count} stale entries.");
}

fn cmd_query(keywords: &[String]) {
    let db = db::Database::load().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
    let sorted = db.sorted_entries();
    let initial_query = keywords.join(" ");

    match ui::select(&sorted, &initial_query) {
        Ok(ui::SelectResult::Selected(path)) => println!("{path}"),
        Ok(ui::SelectResult::Cancelled) => std::process::exit(1),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
