use crate::error::*;
use botifactory_common::{botifactory_api::Botifactory, error::BotifactoryError};
use config::Config;

use serde::{Deserialize, Deserializer};
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use url::Url;

pub const CONFIG_PATH_ENVIRONMENT_KEY: &'static str = "BOTIFACTORY_CLI_CONFIG_FILE";
pub const CONFIG_ENVIRONMENT_KEY: &'static str = "BOTIFACTORY_CLI_USE_PURE_ENV";

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct EnvSettings {
    pub base_url: Url,
    pub project_name: String,
    pub channel_name: Option<String>,
    pub channel_id: Option<i64>,
    pub release_name: Option<String>,
    pub release_id: Option<i64>,
    pub upload_binary: Option<PathBuf>,
}

impl EnvSettings {
    pub fn from_env() -> Result<EnvSettings, config::ConfigError> {
        let config_file_location = std::env::var(CONFIG_PATH_ENVIRONMENT_KEY);

        let builder = Config::builder();
        let builder = if let Ok(path) = config_file_location {
            let config_file_path = PathBuf::from(path);

            builder.add_source(config::File::from(config_file_path))
        } else {
            builder
        };

        builder
            .add_source(
                config::Environment::with_prefix("BOTIFACTORY_CLI_")
                    .convert_case(config::Case::Snake)
                    .separator("__")
                    .list_separator(" "),
            )
            .build()?
            .try_deserialize()
    }
}
