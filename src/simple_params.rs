use crate::parameters::{ChannelVerb, ProjectVerb};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Params {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReleaseVerb {
    Create,
    Show,
    Download,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Project { verb: ProjectVerb },
    Channel { verb: ChannelVerb },
    Release { verb: ReleaseVerb },
}

pub fn parse_parameters() {
    let args = Params::parse();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Params::command().debug_assert();
}
