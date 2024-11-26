use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use jayce::deploy_config::{DeployArgs, DeployConfig, DeployModuleType};
use jayce::tasks::deploy::deploy_contracts;

#[derive(Parser, Debug)]
#[command(name = "jayce")]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
enum Commands {
    /// Deploy contracts
    Deploy {
        #[arg(short, long)]
        private_key: Option<String>,
        #[arg(short, long, default_value = "object")]
        module_type: DeployModuleType,
        #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
        modules_path: Option<Vec<PathBuf>>,
        /// Sets a custom config file
        #[arg(short, long)]
        config_path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    if args.version {
        println!(env!("APP_VERSION"));
        return Ok(());
    }
    match args.command {
        None => {
            Cli::command().print_help()?;
            Ok(())
        }
        Some(command) => match command {
            Commands::Deploy {
                private_key,
                config_path, module_type, modules_path,
            } => {
                let deploy_args= if let Some(config_path) = config_path {
                    DeployArgs::from_path(config_path.to_str().unwrap())?
                } else {
                    let private_key = private_key.expect("Missing argument private key");
                    let modules_path = modules_path.expect("Missing argument modules path");
                    DeployArgs {
                        private_key,
                        module_type,
                        modules_path
                    }
                };

                let config = DeployConfig::try_from(deploy_args)?;
                deploy_contracts(&config).await
            }
        },
    }
}
