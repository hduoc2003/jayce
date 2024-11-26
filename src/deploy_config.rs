use std::path::PathBuf;
use aptos_sdk::types::LocalAccount;
use clap::ValueEnum;
use config::{Config as ConfigLoader, File, FileFormat};
use serde::Deserialize;


#[derive(Deserialize, Clone, Debug, PartialEq, ValueEnum)]
pub enum DeployModuleType {
    Account,
    Object,
}

#[derive(Debug)]
pub struct DeployConfig {
    account: LocalAccount,
    module_type: DeployModuleType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeployArgs {
    pub private_key: String,
    pub module_type: DeployModuleType,
    pub modules_path: Vec<PathBuf>
}

impl DeployArgs {
    pub fn from_path(path: &str) -> anyhow::Result<DeployArgs> {
        let content = ConfigLoader::builder()
            .add_source(File::new(path, FileFormat::Toml))
            .build()?;
        let args: DeployArgs = content.try_deserialize()?;

        Ok(args)
    }
}

impl TryFrom<DeployArgs> for DeployConfig {
    type Error = anyhow::Error;

    fn try_from(args: DeployArgs) -> anyhow::Result<DeployConfig> {
        let account = LocalAccount::from_private_key(&args.private_key, 0)?;
        Ok(DeployConfig { account, module_type: args.module_type })
    }
}
