use configuration::{EnvSettings, CONFIG_ENVIRONMENT_KEY, CONFIG_PATH_ENVIRONMENT_KEY};

mod configuration;
mod error;
mod parameters;
mod simple_params;

use anyhow::Result;
use botifactory_common::{Botifactory, Identifier, NewRelease};
use parameters::{ChannelVerb, ProjectVerb};
use thiserror::Error;

#[derive(Error, Debug)]
enum CLIError {
    #[error("Bad arguments")]
    BadArguments,
}

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let config_env_key = std::env::var(CONFIG_ENVIRONMENT_KEY);
    let config_env_path = std::env::var(CONFIG_PATH_ENVIRONMENT_KEY);
    if !(config_env_key.is_ok() || config_env_path.is_ok()) {
        use parameters::*;
        let args = parameters::Params::parse();
        match args.command {
            Commands::Project { name, verb } => {
                let api = Botifactory::new(args.base_url, &name);
                match verb {
                    ProjectVerb::Show => {
                        let response = api.get_project().await?;
                        println!("response: {response}");
                    }
                    ProjectVerb::Create => {
                        let (response, _) = api.new_project(&name).await?;
                        println!("response: {response}");
                    }
                }
            }
            Commands::Channel {
                project_name,
                verb,
                identifier,
            } => {
                let api = Botifactory::new(args.base_url, &project_name);

                match verb {
                    ChannelVerb::Create => {
                        let name = if identifier.name.is_some() {
                            Ok(identifier.name.expect("Not sure how this happened"))
                        } else {
                            println!("Need a channel name or a channel id to show a channel");
                            Err(CLIError::BadArguments)
                        }?;

                        let (channel_json, _) = api.new_channel(&name).await?;
                        println!("channel: {channel_json}")
                    }
                    ChannelVerb::Show => {
                        let id = if identifier.name.is_some() {
                            Ok(Identifier::Name(
                                identifier.name.expect("Not sure how this happened"),
                            ))
                        } else if identifier.id.is_some() {
                            Ok(Identifier::Id(
                                identifier.id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a channel name or a channel id to show a channel");
                            Err(CLIError::BadArguments)
                        }?;

                        let channel_api = api.channel(id);
                        let response = channel_api.get_channel().await?;
                        println!("response: {response}");
                    }
                }
            }
            Commands::Release {
                project_name,
                channel_name,
                verb,
                identifier,
            } => {
                let api = Botifactory::new(args.base_url, &project_name);

                match verb {
                    ReleaseVerb::Show => {
                        let release_id = if identifier.name.is_some() {
                            Ok(Identifier::Name(
                                identifier.name.expect("Not sure how this happened"),
                            ))
                        } else if identifier.id.is_some() {
                            Ok(Identifier::Id(
                                identifier.id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a release name or a releasechannel id to show a release");
                            Err(CLIError::BadArguments)
                        }?;

                        let channel_api = api.channel(Identifier::Name(channel_name));

                        let release_api = channel_api.release(release_id);
                        let response = release_api.release_info().await?;
                        println!("response: {response}");
                    }
                    ReleaseVerb::Create { path, version } => {
                        let channel_api = api.channel(Identifier::Name(channel_name));
                        let (release_json, _) = channel_api
                            .new_release(NewRelease::new(version, path))
                            .await?;

                        println!("release_json: {release_json}");
                    }
                    ReleaseVerb::Download { path } => {
                        let release_id = if identifier.name.is_some() {
                            Ok(Identifier::Name(
                                identifier.name.expect("Not sure how this happened"),
                            ))
                        } else if identifier.id.is_some() {
                            Ok(Identifier::Id(
                                identifier.id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a releasechannel name or a releasechannel id to show a release");
                            Err(CLIError::BadArguments)
                        }?;

                        let channel_api = api.channel(Identifier::Name(channel_name));
                        let release_api = channel_api.release(release_id);
                        release_api.release_binary_path(path).await?;
                        println!("dl finished");
                    }
                }
            }
        }
    } else {
        use simple_params::*;
        let settings = EnvSettings::from_env()?;
        let args = simple_params::Params::parse();
        let api = Botifactory::new(settings.base_url, &settings.project_name);

        match args.command {
            Commands::Project { verb } => {
                match verb {
                    ProjectVerb::Show => {
                        //let url = api.get_project_url()?;
                        //println!("project_url: {url}");
                        let response = api.get_project().await?;
                        println!("response: {response}")
                    }
                    ProjectVerb::Create => {
                        //let url = api.new_project_url()?;
                        //println!("project_url: {url}");
                        let (response, _) = api.new_project(&settings.project_name).await?;
                        println!("response: {response}")
                    }
                }
            }
            Commands::Channel { verb } => match verb {
                ChannelVerb::Show => {
                    let id = if settings.channel_name.is_some() {
                        Ok(Identifier::Name(
                            settings.channel_name.expect("Not sure how this happened"),
                        ))
                    } else if settings.channel_id.is_some() {
                        Ok(Identifier::Id(
                            settings.channel_id.expect("Not sure how this happened"),
                        ))
                    } else {
                        println!("Need a channel name or a channel id to show a channel");
                        Err(CLIError::BadArguments)
                    }?;

                    let channel = api.channel(id);
                    //let url = channel.get_channel_url()?;
                    //println!("url: {url}");
                    let response = channel.get_channel().await?;
                    println!("response: {response}")
                }
                ChannelVerb::Create => {
                    let channel_name = if settings.channel_name.is_some() {
                        Ok(settings.channel_name.expect("Not sure how this happened"))
                    } else {
                        println!("Need a channel name to create a channel");
                        Err(CLIError::BadArguments)
                    }?;

                    //let url = api.create_channel_url()?;
                    //println!("url: {url}");

                    let (channel_json, _) = api.new_channel(&channel_name).await?;
                    println!("channel_json: {channel_json}")
                }
            },
            Commands::Release { verb } => {
                match verb {
                    ReleaseVerb::Show => {
                        let channel_id = if settings.channel_name.is_some() {
                            Ok(Identifier::Name(
                                settings.channel_name.expect("Not sure how this happened"),
                            ))
                        } else if settings.channel_id.is_some() {
                            Ok(Identifier::Id(
                                settings.channel_id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a channel name or a channel id to show a channel");
                            Err(CLIError::BadArguments)
                        }?;

                        let channel_api = api.channel(channel_id);

                        let release_id = if settings.release_name.is_some() {
                            Ok(Identifier::Name(
                                settings.release_name.expect("Not sure how this happened"),
                            ))
                        } else if settings.release_id.is_some() {
                            Ok(Identifier::Id(
                                settings.release_id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a releasechannel name or a releasechannel id to show a release");
                            Err(CLIError::BadArguments)
                        }?;
                        //println!("release_id: {release_id:#?}");
                        let release_api = channel_api.release(release_id);
                        //let url = release_api.release_url()?;
                        //println!("url: {url}");

                        let response = release_api.release_info().await?;
                        println!("response: {response}");
                    }
                    ReleaseVerb::Create { path, version } => {
                        let channel_id = if settings.channel_name.is_some() {
                            Ok(Identifier::Name(
                                settings.channel_name.expect("Not sure how this happened"),
                            ))
                        } else if settings.channel_id.is_some() {
                            Ok(Identifier::Id(
                                settings.channel_id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a channel name or a channel id to show a channel");
                            Err(CLIError::BadArguments)
                        }?;
                        println!("channel_id: {channel_id:#?}");

                        let channel_api = api.channel(channel_id);

                        //let url = channel_api.new_release_url()?;
                        //println!("url: {url}");

                        let (release_json, _) = channel_api
                            .new_release(NewRelease::new(version, path))
                            .await?;

                        println!("release_json: {release_json}");
                    }
                    ReleaseVerb::Download { path } => {
                        println!("release verb download");
                        let channel_id = if settings.channel_name.is_some() {
                            Ok(Identifier::Name(
                                settings.channel_name.expect("Not sure how this happened"),
                            ))
                        } else if settings.channel_id.is_some() {
                            Ok(Identifier::Id(
                                settings.channel_id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a channel name or a channel id to show a channel");
                            Err(CLIError::BadArguments)
                        }?;
                        //println!("channel_id: {channel_id:#?}");

                        let channel_api = api.channel(channel_id);

                        let release_id = if settings.release_name.is_some() {
                            Ok(Identifier::Name(
                                settings.release_name.expect("Not sure how this happened"),
                            ))
                        } else if settings.release_id.is_some() {
                            Ok(Identifier::Id(
                                settings.release_id.expect("Not sure how this happened"),
                            ))
                        } else {
                            println!("Need a releasechannel name or a releasechannel id to show a release");
                            Err(CLIError::BadArguments)
                        }?;
                        println!("release_id: {release_id:#?}");
                        let release_api = channel_api.release(release_id);
                        //let url = release_api.release_url()?;
                        //println!("url: {url}");

                        release_api.release_binary_path(path).await?;
                        println!("dl finished");
                    }
                }
            }
        }
    }
    Ok(())
}
