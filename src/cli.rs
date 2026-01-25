use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "scope-kit")]
#[command(about = "Feature specification toolkit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Validate a feature specification
    Validate {
        /// Name of the feature to validate
        feature_name: String,
    },
}
