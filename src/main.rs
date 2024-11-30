use configuration::{EnvSettings, CONFIG_ENVIRONMENT_KEY, CONFIG_PATH_ENVIRONMENT_KEY};

mod configuration;
mod error;
mod parameters;
mod request_actions;
mod simple_params;

use clap::Parser;

fn main() {
    let config_env_key = std::env::var(CONFIG_ENVIRONMENT_KEY);
    let config_env_path = std::env::var(CONFIG_PATH_ENVIRONMENT_KEY);
    if !(config_env_key.is_ok() || config_env_path.is_ok()) {
        let args = parameters::Params::parse();
    } else {
        let settings = EnvSettings::from_env();
        let args = simple_params::Params::parse();
    }
}
