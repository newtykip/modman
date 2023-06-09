use clap::{Parser, Subcommand};

pub mod config;
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
    #[clap(alias = "p")]
    Profile(profile::Command),

    /// Manage modrinth-based mods
    #[clap(alias = "mr")]
    Modrinth(modrinth::Command),

    /// Manage your global configuration
    #[clap(aliases = &["c", "cfg"])]
    Config(config::Command),
}
