use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

mod cli;
mod feature;
mod record;
mod tree;
mod validate;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Feature { command } => handle_feature_command(command),
        cli::Commands::Record { plan_name } => handle_record_command(&plan_name),
    }
}

fn handle_feature_command(command: cli::FeatureCommands) -> ExitCode {
    let base = PathBuf::from("specs");

    match command {
        cli::FeatureCommands::List { domain } => {
            if let Some(domain) = domain {
                let features = feature::discover_features_in_domain(&base, &domain);
                print!("{}", tree::render_domain_tree(&domain, &features));
            } else {
                let features = feature::discover_features(&base);
                print!("{}", tree::render_tree(&features));
            }
            ExitCode::SUCCESS
        }

        cli::FeatureCommands::Validate { target } => {
            let results = match target.as_deref() {
                None => validate::run_all(&base),
                Some(t) if t.contains('/') => {
                    let parts: Vec<&str> = t.splitn(2, '/').collect();
                    let fp = feature::FeaturePath::new(parts[0], parts[1]);
                    vec![(fp.clone(), validate::run_feature(&base, &fp))]
                }
                Some(domain) => validate::run_domain(&base, domain),
            };

            print_validation_results(&results)
        }
    }
}

fn print_validation_results(
    results: &[(
        feature::FeaturePath,
        Result<validate::report::ValidationResult, validate::report::ValidationError>,
    )],
) -> ExitCode {
    if results.is_empty() {
        println!("No features found to validate.");
        return ExitCode::SUCCESS;
    }

    let mut has_errors = false;

    for (fp, result) in results {
        match result {
            Ok(vr) => {
                let status = if vr.is_success() {
                    if vr.warnings.is_empty() { "✓" } else { "⚠" }
                } else {
                    has_errors = true;
                    "✗"
                };
                println!(
                    "{} {} ({} errors, {} warnings)",
                    status,
                    fp,
                    vr.errors.len(),
                    vr.warnings.len()
                );

                for error in &vr.errors {
                    println!("    ERROR: {}", error);
                }
                for warning in &vr.warnings {
                    println!("    WARN: {}", warning);
                }
            }
            Err(e) => {
                has_errors = true;
                println!("✗ {} - {}", fp, e);
            }
        }
    }

    if has_errors {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn handle_record_command(plan_name: &str) -> ExitCode {
    let base = PathBuf::from("specs");

    match record::record_plan(&base, plan_name) {
        Ok(features) => {
            println!("Recorded plan '{}' to specs/_recorded/", plan_name);
            for feature in &features {
                println!("  ✓ {}", feature);
            }

            // Validate recorded specs
            println!("\nValidating recorded specs...");
            let mut has_errors = false;
            for feature_path in &features {
                let parts: Vec<&str> = feature_path.splitn(2, '/').collect();
                if parts.len() == 2 {
                    let fp = feature::FeaturePath::new(parts[0], parts[1]);
                    match validate::run_feature(&base, &fp) {
                        Ok(result) => {
                            if !result.is_success() {
                                has_errors = true;
                                println!("  ✗ {} - {} errors", feature_path, result.errors.len());
                            } else {
                                println!("  ✓ {}", feature_path);
                            }
                        }
                        Err(e) => {
                            has_errors = true;
                            println!("  ✗ {} - {}", feature_path, e);
                        }
                    }
                }
            }

            if has_errors {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            ExitCode::from(1)
        }
    }
}
