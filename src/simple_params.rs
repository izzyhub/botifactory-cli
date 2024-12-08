use crate::parameters::{ChannelVerb, ProjectVerb};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Params {
    #[command(subcommand)]
    pub command: Commands,
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

#[derive(Subcommand, Debug)]
pub enum Commands {
    Project {
        verb: ProjectVerb,
    },
    Channel {
        verb: ChannelVerb,
    },
    Release {
        #[command(subcommand)]
        verb: ReleaseVerb,
    },
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Params::command().debug_assert();
}
