//! CLI for profile runner

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;
use profile_runner::{run_profile, RunConfig, Action};
use profile_witness_generator::{JsonTestCaseGenerator, TestCaseGenerator};

#[derive(Parser)]
#[command(name = "profile-runner")]
#[command(about = "Profile runner for zkEVM workloads")]
#[command(version)]
struct Cli {
    /// Output folder for profile results
    #[arg(short, long, default_value = "results")]
    output_folder: PathBuf,

    /// Force rerun profiles even if output files already exist
    #[arg(long, default_value_t = false)]
    force_rerun: bool,

    /// Source of test cases
    #[command(subcommand)]
    source: SourceCommand,
}

#[derive(Subcommand, Clone, Debug)]
enum SourceCommand {
    /// Generate and run profiles from JSON test files
    Tests {
        /// Directory containing JSON test files
        #[arg(short, long, default_value = "tests")]
        test_directory: String,

        /// Action to perform
        #[arg(long, default_value = "full")]
        action: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    
    let cli = Cli::parse();

    info!("Starting profile runner with output folder: {:?}", cli.output_folder);
    if !cli.output_folder.exists() {
        std::fs::create_dir_all(&cli.output_folder)
            .with_context(|| format!("Failed to create output folder: {:?}", cli.output_folder))?;
    }

    let run_config = build_run_config(&cli)?;
    let test_cases = build_test_cases(&cli.source).await?;

    info!("Running profiles with {} test cases", test_cases.len());
    run_profile(&run_config, &test_cases).await
        .context("Failed to run profiles")?;

    info!("Profile run completed successfully");

    Ok(())
}

fn build_run_config(cli: &Cli) -> Result<RunConfig> {
    let action = match cli.source {
        SourceCommand::Tests { ref action, .. } => {
            match action.as_str() {
                "execute" => Action::Execute,
                "prove" => Action::Prove,
                "full" => Action::Full,
                _ => return Err(anyhow::anyhow!("Invalid action: {}. Must be 'execute' or 'prove' or 'full'", action)),
            }
        }
    };

    Ok(RunConfig {
        output_folder: cli.output_folder.clone(),
        action,
        force_rerun: cli.force_rerun,
    })
}

async fn build_test_cases(source: &SourceCommand) -> Result<Vec<profile_witness_generator::NamedTestCase>> {
    match source {
        SourceCommand::Tests { test_directory, .. } => {
            let generator = JsonTestCaseGenerator::new(test_directory.clone());
            generator.generate().await
                .context("Failed to generate test cases")
        }
    }
} 