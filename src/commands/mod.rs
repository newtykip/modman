use clap::{Parser, Subcommand};

pub mod modrinth;
pub mod profile;

#[derive(Parser)]
#[clap(version)]
pub struct Value {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage your profiles
    Profile(profile::Command),

    /// Manage modrinth-based mods
    Modrinth(modrinth::Command),
}
