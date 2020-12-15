/// SEE https://github.com/serenity-rs/serenity/blob/current/examples/e05_command_framework/src/main.rs for example
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{
        macros::{help, hook},
        CommandGroup, HelpOptions,
    },
    model::id::UserId,
};

pub struct ShardManagerContainer;

use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

use crate::prelude::*;
use crate::utils::config::{CommandData, ConfigData};

#[instrument]
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

    let mut commands: Vec<CommandData> = vec![];
    for group in groups {
        for cmd in group.options.commands {
            if cmd.options.owners_only && owners.get(&msg.author.id).is_none() {
                continue;
            }
            commands.push((*cmd).into());
        }
    }
    for cmd in borrowed_config.get_commands() {
        if cmd.restricted() && owners.get(&msg.author.id).is_none() {
            continue;
        }
        commands.push(cmd.clone());
    }

    commands.sort_by(|a, b| a.get_trigger().partial_cmp(b.get_trigger()).unwrap());
    if let Ok(next_arg) = args.single::<String>() {
        for cmd in &commands {
            if next_arg == cmd.get_trigger() {
                caught_error = if cmd.restricted() {
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
        info!("Help command failed: {}", e.to_string());
    }
    Ok(())
}

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

async fn embedded_pm(ctx: &Context, msg: &Message, cmd_data: &CommandData) -> CommandResult {
    msg.author
        .direct_message(ctx, |m| {
            m.embed(|embed| {
                embed.colour(cmd_data.get_color());
                embed.field(cmd_data.get_trigger(), &cmd_data.get_help(), true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

fn flatten_cmds(cmds: &Vec<CommandData>) -> String {
    let mut flattened = String::new();
    for cmd in cmds {
        flattened += &format!("`{}`\n", cmd.get_trigger());
    }
    flattened
}

async fn echo_group_cmds(
    ctx: &Context,
    msg: &Message,
    config_data: &ConfigData,
    cmds: &Vec<CommandData>,
) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|embed| {
                embed.colour(config_data.get_help_color().clone());
                embed.description(config_data.get_help_message());
                embed.field("Muffetbot", flatten_cmds(cmds), true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

async fn single_help(ctx: &Context, msg: &Message, cmd_data: &CommandData) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|embed| {
                embed.colour(cmd_data.get_color());
                embed.field(cmd_data.get_trigger(), cmd_data.get_help(), true);
                embed
            });
            m
        })
        .await?;
    Ok(())
}

#[hook]
#[instrument]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    for cmd in crate::CONFIG.lock().await.get_commands() {
        if unknown_command_name == cmd.get_trigger() {
            if let CommandResponse::Embed = cmd.get_response_type() {
                if let Err(e) = embedded_cmd(ctx, msg, cmd).await {
                    info!("Config command announcement failed: {}", e);
                }
            } else {
                if let Err(e) = announce(ctx, msg, &cmd.get_value(), &cmd.get_response_type()).await
                {
                    info!("Config command announcement failed: {}", e);
                }
            }
            break;
        }
    }
}
