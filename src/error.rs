use config;
use thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotifactoryCLIError {
    #[error("configuration error")]
    ConfigError,
    #[error("configuration error")]
    ContextConfigError(#[from] config::ConfigError),
    #[error("Env variable is not valid unicode")]
    EnvVarError(#[from] std::env::VarError),
}
