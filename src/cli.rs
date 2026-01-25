use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "speq")]
#[command(about = "Feature specification toolkit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage feature specifications
    Feature {
        #[command(subcommand)]
        command: FeatureCommands,
    },

    /// Record approved plan deltas to permanent specs
    Record {
        /// Name of the plan to record
        plan_name: String,
    },
}

#[derive(Subcommand)]
pub enum FeatureCommands {
    /// List all features or features in a domain
    List {
        /// Domain to list features from (optional)
        domain: Option<String>,
    },

    /// Validate feature specifications
    Validate {
        /// Target: empty=all, domain name, or domain/feature
        target: Option<String>,
    },
}
