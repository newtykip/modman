use clap::{Parser, Subcommand};

pub mod export;
pub mod init;

#[derive(Parser)]
#[clap(version)]
pub struct Value {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // todo: move to own subcommand under profile command
    /// Initialise a new profile
    Init(init::Args),

    /// Export the currently selected profile
    Export(export::Command),
}
