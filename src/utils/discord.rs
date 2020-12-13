/// SEE https://github.com/serenity-rs/serenity/blob/current/examples/e05_command_framework/src/main.rs for example
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{
        macros::{help, hook},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

pub struct ShardManagerContainer;

use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

use crate::prelude::*;
use crate::utils::config::{CommandData, ConfigData};
use log::*;

async fn embedded_cmd(ctx: &Context, msg: &Message, cmd_data: &CommandData) -> CommandResult {
    msg.channel_id
        .send_message(ctx, |m| {
            m.embed(|embed| {
                embed.colour(cmd_data.get_color());
                embed.field(cmd_data.get_trigger(), cmd_data.get_value(), true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

async fn embedded_pm(ctx: &Context, msg: &Message, cmd_data: &CommandMetaData) -> CommandResult {
    msg.author
        .direct_message(ctx, |m| {
            m.embed(|embed| {
                embed.colour(cmd_data.color.clone());
                embed.field(&cmd_data.trigger, &cmd_data.help, true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

#[help]
pub async fn muffet_help(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    _help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let borrowed_config = &crate::CONFIG.lock().await;
    let mut caught_error: Result<(), serenity::framework::standard::CommandError> = Ok(());

    let mut commands = vec![];
    for group in groups {
        for cmd in group.options.commands {
            if cmd.options.owners_only && owners.get(&msg.author.id).is_none() {
                continue;
            }
            commands.push(CommandType::Serenity(*cmd).metadata());
        }
    }
    for cmd in &borrowed_config.commands {
        if cmd.restricted() && owners.get(&msg.author.id).is_none() {
            continue;
        }
        commands.push(CommandType::Muffet(cmd).metadata());
    }

    commands.sort_by(|a, b| a.trigger.partial_cmp(&b.trigger).unwrap());
    if let Ok(next_arg) = args.single::<String>() {
        for cmd in &commands {
            if next_arg == cmd.trigger {
                caught_error = if cmd.restricted {
                    embedded_pm(ctx, msg, cmd).await
                } else {
                    single_help(ctx, msg, cmd).await
                };
                break;
            }
        }
    } else {
        caught_error = echo_group_cmds(ctx, msg, borrowed_config, &commands).await;
    }

    if let Err(e) = caught_error {
        error!("Help command failed: {}", e.to_string());
    }
    Ok(())
}

use crate::utils::config::Color;
struct CommandMetaData {
    color: Color,
    help: String,
    restricted: bool,
    trigger: String,
}

enum CommandType<'a> {
    Serenity(&'static serenity::framework::standard::Command),
    Muffet(&'a CommandData),
}

impl<'a> CommandType<'a> {
    fn metadata(self) -> CommandMetaData {
        match self {
            CommandType::Serenity(cmd) => CommandMetaData {
                color: crate::utils::config::Color::default(),
                help: {
                    let help = if cmd.options.owners_only {
                        String::from("*Admin command*\n")
                    } else {
                        String::new()
                    };
                    if cmd.options.help_available {
                        help + cmd.options.desc.unwrap_or_default()
                            + "\n"
                            + cmd
                                .options
                                .usage
                                .unwrap_or("No help available for this command")
                            + "\n"
                            + cmd.options.examples.join("\n").as_ref()
                    } else {
                        help
                    }
                },
                restricted: cmd.options.owners_only,
                trigger: cmd.options.names[0].to_string(),
            },
            CommandType::Muffet(cmd) => CommandMetaData {
                color: cmd.get_color().clone(),
                help: {
                    let help = if cmd.restricted() {
                        String::from("*Admin command*\n")
                    } else {
                        String::new()
                    };
                    help + cmd.get_help()
                },
                restricted: cmd.restricted(),
                trigger: cmd.get_trigger().to_string(),
            },
        }
    }
}

fn flatten_cmd_meta(cmds: &Vec<CommandMetaData>) -> String {
    let mut flattened = String::new();
    for cmd in cmds {
        flattened += &format!("`{}`\n", cmd.trigger);
    }
    flattened
}

async fn echo_group_cmds(
    ctx: &Context,
    msg: &Message,
    config_data: &ConfigData,
    cmds: &Vec<CommandMetaData>,
) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|embed| {
                embed.colour(config_data.help_color.clone());
                embed.description(&config_data.help_message);
                embed.field("Muffetbot", flatten_cmd_meta(cmds), true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

async fn single_help(ctx: &Context, msg: &Message, cmd_data: &CommandMetaData) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|embed| {
                embed.colour(cmd_data.color.clone());
                embed.field(&cmd_data.trigger, &cmd_data.help, true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let config_commands = &crate::CONFIG.lock().await.commands;

    for cmd in config_commands {
        if unknown_command_name == cmd.get_trigger() {
            if let CommandResponse::Embed = cmd.get_response_type() {
                if let Err(e) = embedded_cmd(ctx, msg, cmd).await {
                    error!("Config command announcement failed: {}", e);
                }
            } else {
                if let Err(e) = announce(ctx, msg, &cmd.get_value(), &cmd.get_response_type()).await
                {
                    error!("Config command announcement failed: {}", e);
                }
            }
            break;
        }
    }
}
