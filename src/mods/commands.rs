use crate::prelude::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(about_custom)]
struct CustomCommands;

const ABOUT_CUSTOM_COMMANDS: &str = r#"
Custom commands offering more flexibility than
what the config offers are able to be compiled
as part of the mods folder.

This grants access to all of the Discord API,
Serenity API, and muffetbot's built-in webscraper.
"#;

#[instrument]
#[command]
#[owners_only]
#[description = "info on creating advanced custom commands"]
#[usage = "`!about_custom`"]
async fn about_custom(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, ABOUT_CUSTOM_COMMANDS, &CommandResponse::DmOwner).await
}
