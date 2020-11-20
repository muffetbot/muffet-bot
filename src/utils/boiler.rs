/// SEE https://github.com/serenity-rs/serenity/blob/current/examples/e05_command_framework/src/main.rs for example
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::standard::{
        help_commands,
        macros::{help, hook},
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

pub struct ShardManagerContainer {}

use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Handler {}
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} entered the channel", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!help" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "hi!").await {
                eprintln!("{}", e);
            }
        }
    }
}

#[help]
#[command_not_found_text = "`{}` is not a command!"]
#[individual_command_tip = "To get more info on a command, pass it as an argument.
i.e. `!help goals`"]
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
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    if let Err(e) = crate::prelude::announce(
        ctx,
        msg,
        &format!("unknown command: {}", unknown_command_name),
    )
    .await
    {
        eprintln!("{}", e);
    }
}
