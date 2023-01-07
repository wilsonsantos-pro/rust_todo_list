use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Add(AddCommand),
    Init,
    List,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    pub title: String,
}
