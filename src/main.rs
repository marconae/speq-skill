use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

mod cli;
mod validate;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Validate { feature_name } => {
            let path = PathBuf::from(format!("specs/{}/spec.md", feature_name));

            match validate::run(&path) {
                Ok(result) => {
                    validate::report::print_result(&result);
                    if result.is_success() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::from(1)
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    ExitCode::from(1)
                }
            }
        }
    }
}
