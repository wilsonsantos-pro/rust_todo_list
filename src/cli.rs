use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Add(AddCommand),
    Done(DoneCommand),
    Undone(DoneCommand),
    Init,
    List,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    pub title: String,
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    pub id: i32,
}
