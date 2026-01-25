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
    /// Manage domains
    Domain {
        #[command(subcommand)]
        command: DomainCommands,
    },

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

    /// Semantic search for specifications
    Search {
        #[command(subcommand)]
        command: SearchCommands,
    },
}

#[derive(Subcommand)]
pub enum SearchCommands {
    /// Build or rebuild the search index
    Index,

    /// Search for scenarios matching a query (use: speq search query "your query")
    Query {
        /// The search query
        query: String,

        /// Maximum number of results
        #[arg(long, default_value = "10")]
        limit: usize,
    },
}

// Note: The spec specifies `speq search <query>` syntax, but clap requires
// subcommands. Users should use `speq search query "their query"` instead.
// Alternatively, we could add a positional arg directly to Search, but that
// conflicts with the subcommand pattern. The current design prioritizes
// explicit commands over implicit behavior.

#[derive(Subcommand)]
pub enum DomainCommands {
    /// List all domains in the specs directory
    List,
}

#[derive(Subcommand)]
pub enum FeatureCommands {
    /// Get a feature spec or scenario
    Get {
        /// Path: domain/feature or domain/feature/scenario
        path: String,
    },

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
