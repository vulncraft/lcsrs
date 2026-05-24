mod cli;
mod commands;
mod db;
mod ids;
mod scheduler;
mod url_norm;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Command};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Cli::parse();
    let mut conn = db::open()?;
    match args.command {
        Command::Add(a) => commands::add(&mut conn, a),
        Command::Next(a) => commands::next(&conn, a),
        Command::Grade(a) => commands::grade(&mut conn, a),
        Command::Delete(a) => commands::delete(&conn, a),
        Command::List(a) => commands::list(&conn, a),
    }
}
