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

use crate::prelude::announce;

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
        eprintln!("{}", e);
    }
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    if let Err(e) = announce(
        ctx,
        msg,
        &format!("unknown command: {}", unknown_command_name),
    )
    .await
    {
        eprintln!("{}", e);
    }
}
