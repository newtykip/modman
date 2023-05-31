use clap::{Parser, Subcommand};

pub mod add;
pub mod profile;
pub mod sync;

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

    /// Synchronize your profiles
    Sync(sync::Command),

    /// Add mods to a profile
    Add(add::Command),
}
