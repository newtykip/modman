use clap::{Parser, Subcommand};

pub mod init;

#[derive(Parser)]
#[clap(version)]
pub struct Value {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialise a project with modman
    Init(init::Args),
}
