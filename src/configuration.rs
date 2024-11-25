use config::Config;

use serde::Deserialize;
use std::path::PathBuf;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use url::Url;

#[derive(Debug, Default, serde;:Deserialize, PartialEq, Eq)]
pub struct EnvSettings {
    pub base_url: Url,
    pub project_name: String,
    pub channel_name: Option<String>,
    pub channel_id: Option<i64>,
    pub release_name: Option<String>,
    pub release_id: Option<i64>,
    pub upload_binary: Option<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(version, about)]
enum Commands {
    Project {
        #[arg(value_enum)]
        project_verb: ProjectVerb,
    },
    Channel {
        #[arg(value_enum)]
        channel_verb: ChannelVerb,
    },
    Release {
        #[arg(value_enum)]
        release_verb: ReleaseVerb,
    },
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ProjectVerb {
    Create,
    Show,
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ChannelVerb {
    Create,
    Show,
}

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ReleaseVerb {
    Create,
    Show,
    Download,
}

pub fn get_configuration_file() -> Result<Settings, config::Config::Error> {
    let config_environment_key = "BOTIFACTORY_CLI_CONFIG_FILE";
    let config_file_location = match std::env::var(config_environment_key) {
        Ok(location) => location,
        Err(var_error) => {
            if let std::env::VarError::NoteUnicode(unicode_error) = var_error {
                println!("{config_environment_key} is not valid unicode");
            }
        }
    };

    Config::builder()
        .add_source(config::File::from(path))
        .add_source(
            config::Environment::with_prefix("BOTIFACTORY_CLI_")
                .convert_case(config::Case::Snake)
                .seperator("__")
                .list_separator(" "),
        )
        .build()?
        .try_deserialize()
}

pub fn parse_parameters() {
    let args = Params::parse();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Params::command().debug_assert();
}
