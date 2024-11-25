use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use url::Url;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Params {
    #[arg(short, long, default_value = "https://botifactory.izzys.place")]
    base_url: Url,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct ChannelIdentifier {
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    id: Option<i64>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct ReleaseIdentifier {
    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    id: Option<i64>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Project {
        #[arg(short, long)]
        name: String,

        #[arg(value_enum)]
        project_verb: ProjectVerb,
    },
    Channel {
        #[arg(short, long)]
        project_name: String,

        #[command(flatten)]
        identifier: ChannelIdentifier,

        #[arg(value_enum)]
        channel_verb: ChannelVerb,
    },
    Release {
        #[arg(short, long)]
        project_name: String,
        #[arg(short, long)]
        channel_name: String,
        #[command(flatten)]
        identifier: ReleaseIdentifier,

        #[command(subcommand)]
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

#[derive(Subcommand, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ReleaseVerb {
    Create {
        #[arg(short, long)]
        path: PathBuf,
    },
    Show,
    Download {
        #[arg(short, long)]
        path: PathBuf,
    },
}

pub fn parse_parameters() {
    let args = Params::parse();
    println!("hello world");

    match args.command {
        (Commands::Project { name, project_verb }) => {
            println!("project subcommand");
            match project_verb {
                ProjectVerb::Show {} => {
                    println!("project show subcommand");
                }
                ProjectVerb::Create {} => {
                    println!("project create subcommand");
                }
            }
        }
        (Commands::Channel {
            project_name,
            channel_verb,
            identifier,
        }) => {
            println!("channel subcommand");
            match channel_verb {
                ChannelVerb::Create => {
                    println!("create verb");
                }
                ChannelVerb::Show => {
                    println!("show verb");
                }
            }
        }
        (Commands::Release {
            project_name,
            channel_name,
            release_verb,
            identifier,
        }) => {
            println!("release subcommand");
            match release_verb {
                ReleaseVerb::Show => {
                    println!("show verb");
                }
                ReleaseVerb::Create { path } => {
                    println!(
                        "theortically would upload whatever's at: {}",
                        path.display()
                    );
                }
                ReleaseVerb::Download { path } => {
                    println!(
                        "theortically would download to whatever's at: {}",
                        path.display()
                    );
                }
            }
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Params::command().debug_assert();
}
