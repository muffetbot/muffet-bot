/// SEE https://github.com/serenity-rs/serenity/blob/current/examples/e05_command_framework/src/main.rs for example
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{
        help_commands,
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
use log::*;

const ADMIN_HELP: &str = r#"
Admin commands:
addcom: add a command
    usage: `!addcom <command trigger> <display value>`
    example: `!addcom ig https://www.instagram.com/me`

rmcom: remove a command
    usage: `!rmcom <command trigger>`
    example: `!rmcom ig`

set_help: change the message that displays before the help command
    usage: `!set_help <new help message>`
    example: `!set_help this is my new help message`
    
"#;

#[help]
#[command_not_found_text = "`{}` is not a command!"]
#[individual_command_tip = ""]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
pub async fn muffet_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    if has_permissions(msg).await {
        if let Err(e) = announce(ctx, msg, ADMIN_HELP, &CommandResponse::DmOwner).await {
            error!("Admin help commands failure: {}", e.to_string());
        }
    }

    let content = &msg.content;
    let mut help_message = String::new();
    let borrowed_config = &crate::CONFIG.lock().await;
    let commands = &borrowed_config.commands;

    if let Some(subcommand) = &content.splitn(2, "!help").nth(1) {
        let subcommand = subcommand.trim_start();
        for cmd in commands {
            if subcommand.starts_with(cmd.get_trigger()) {
                help_message += &format!("{}: {}", cmd.get_trigger(), cmd.get_help());
                break;
            }
        }
    } else {
        help_message += &format!("{}\n\n", &borrowed_config.help_message);
        for cmd in commands {
            help_message += &format!("{}\n", cmd.get_trigger());
        }
    }
    if let Err(e) = announce(ctx, msg, help_message, &CommandResponse::Channel).await {
        error!("Help command failed: {}", e);
    }
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let config_commands = &crate::CONFIG.lock().await.commands;

    for cmd in config_commands {
        if unknown_command_name.trim() == cmd.get_trigger() {
            if let Err(e) = announce(ctx, msg, &cmd.get_value(), &cmd.get_response_type()).await {
                error!("Config command announcement failed: {}", e);
            }
        }
    }
}
