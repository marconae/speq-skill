use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

mod cli;
mod feature;
mod record;
mod search;
mod tree;
mod validate;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Domain { command } => handle_domain_command(command),
        cli::Commands::Feature { command } => handle_feature_command(command),
        cli::Commands::Record { plan_name } => handle_record_command(&plan_name),
        cli::Commands::Search { command } => handle_search_command(command),
    }
}

fn handle_search_command(command: cli::SearchCommands) -> ExitCode {
    let base = PathBuf::from("specs");

    match command {
        cli::SearchCommands::Index => {
            println!("Building search index...");
            match search::index_specs(&base) {
                Ok(count) => {
                    println!("Indexed {} scenarios.", count);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    println!("Error: {}", e);
                    ExitCode::from(1)
                }
            }
        }
        cli::SearchCommands::Query { query, limit } => {
            match search::search_specs(&query, limit) {
                Ok(results) => {
                    if results.is_empty() {
                        println!("No matches found.");
                    } else {
                        for result in results {
                            println!(
                                "{}/{}/{} (score: {:.3})",
                                result.domain, result.feature, result.scenario, result.score
                            );
                            // Show first line of content as snippet
                            if let Some(first_line) = result.content.lines().next() {
                                println!("  {}", first_line);
                            }
                            println!();
                        }
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    println!("{}", e);
                    ExitCode::from(1)
                }
            }
        }
    }
}

fn handle_domain_command(command: cli::DomainCommands) -> ExitCode {
    let base = PathBuf::from("specs");

    match command {
        cli::DomainCommands::List => {
            let domains = feature::discover_domains(&base);
            if domains.is_empty() {
                println!("No domains found.");
            } else {
                for domain in domains {
                    println!("{}/", domain);
                }
            }
            ExitCode::SUCCESS
        }
    }
}

fn handle_feature_get(base: &std::path::Path, path: &str) -> ExitCode {
    // Parse path: domain/feature or domain/feature/scenario
    let parts: Vec<&str> = path.splitn(3, '/').collect();

    if parts.len() < 2 {
        println!("Invalid path format. Use: domain/feature or domain/feature/scenario");
        return ExitCode::from(1);
    }

    let domain = parts[0];
    let feature_name = parts[1];
    let scenario_name = parts.get(2).copied();

    let fp = feature::FeaturePath::new(domain, feature_name);
    let spec_path = fp.spec_path(base);

    if !spec_path.exists() {
        println!("Feature not found: {}/{}", domain, feature_name);
        return ExitCode::from(1);
    }

    let content = match std::fs::read_to_string(&spec_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Error reading feature: {}", e);
            return ExitCode::from(1);
        }
    };

    let parsed = match validate::parser::parse(&content) {
        Ok(p) => p,
        Err(e) => {
            println!("Error parsing feature: {}", e);
            return ExitCode::from(1);
        }
    };

    if let Some(scenario_name) = scenario_name {
        // Find the specific scenario
        if let Some(scenario) = parsed.scenarios.iter().find(|s| s.name == scenario_name) {
            println!("{}/{}/{}", domain, feature_name, scenario_name);
            println!();
            for step in &scenario.steps {
                println!("  {:?} {}", step.kind, step.text);
            }
            ExitCode::SUCCESS
        } else {
            println!(
                "Scenario '{}' not found in {}/{}",
                scenario_name, domain, feature_name
            );
            ExitCode::from(1)
        }
    } else {
        // Display full feature
        if let Some(name) = &parsed.feature_name {
            println!("{}", name);
        }
        println!();
        if let Some(desc) = &parsed.description {
            println!("{}", desc);
            println!();
        }
        for scenario in &parsed.scenarios {
            println!("### {}", scenario.name);
            println!();
            for step in &scenario.steps {
                println!("  {:?} {}", step.kind, step.text);
            }
            println!();
        }
        ExitCode::SUCCESS
    }
}

fn handle_feature_command(command: cli::FeatureCommands) -> ExitCode {
    let base = PathBuf::from("specs");

    match command {
        cli::FeatureCommands::Get { path } => handle_feature_get(&base, &path),

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
