use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Params {
    #[arg(short, long, default_value = "https://botifactory.izzys.place")]
    pub base_url: Url,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ChannelIdentifier {
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub id: Option<i64>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct ReleaseIdentifier {
    #[arg(long)]
    pub name: Option<String>,
    #[arg(long)]
    pub id: Option<i64>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Project {
        #[arg(short, long)]
        name: String,

        #[arg(value_enum)]
        verb: ProjectVerb,
    },
    Channel {
        #[arg(short, long)]
        project_name: String,

        #[command(flatten)]
        identifier: ChannelIdentifier,

        #[arg(value_enum)]
        verb: ChannelVerb,
    },
    Release {
        #[arg(short, long)]
        project_name: String,
        #[arg(short, long)]
        channel_name: String,
        #[command(flatten)]
        identifier: ReleaseIdentifier,

        #[command(subcommand)]
        verb: ReleaseVerb,
    },
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectVerb {
    Create,
    Show,
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChannelVerb {
    Create,
    Show,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseVerb {
    Create {
        #[arg(short, long)]
        path: PathBuf,
        #[arg(long)]
        version: String,
    },
    Show,
    Download {
        #[arg(short, long)]
        path: PathBuf,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Params::command().debug_assert();
}
