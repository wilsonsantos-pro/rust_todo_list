mod cli;
mod tasks;

use cli::Action;
use cli::Cli;

use tasks::{add_task, init, list_tasks};

use clap::Parser;
use rusqlite::{Error, Result};

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    match args.action {
        Action::Init => match init() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Action::Add(task) => match add_task(task) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Action::List => match list_tasks() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
    }
}
