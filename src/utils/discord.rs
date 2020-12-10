/// SEE https://github.com/serenity-rs/serenity/blob/current/examples/e05_command_framework/src/main.rs for example
use serenity::{
    async_trait,
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

use crate::prelude::announce;
use log::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!help" {
            let borrowed_config = &(&(*crate::CONFIG)).lock().await;
            let config_help_message = &borrowed_config.help_message;
            let config_commands = (&borrowed_config.commands).into_iter().fold(
                String::new(),
                |mut help_msg, (name, _target)| {
                    help_msg.push_str("\n");
                    help_msg.push_str(&name);
                    help_msg
                },
            );

            if let Err(e) = announce(
                &ctx,
                &msg,
                format!("{}\n\n{}", config_help_message, config_commands),
            )
            .await
            {
                error!("{}", e);
            }
        }
    }
}

#[help]
#[command_not_found_text = "`{}` is not a command!"]
#[individual_command_tip = ""]
#[strikethrough_commands_tip_in_dm = ""]
#[strikethrough_commands_tip_in_guild = ""]
pub async fn muffet_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    if let Err(e) = announce(context, msg, "Hi!").await {
        error!("{}", e);
    }
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    let config_commands = &(&(*crate::CONFIG)).lock().await.commands;

    for cmd in config_commands {
        if unknown_command_name.trim() == cmd.0 {
            if let Err(e) = announce(ctx, msg, &cmd.1).await {
                error!("{}", e);
            }
        }
    }
}
