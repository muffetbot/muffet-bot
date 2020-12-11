pub use crate::utils::config::CommandResponse;
pub use crate::utils::scraper::SteelCutter;
pub use log::*;
pub use serenity::prelude::*;
use serenity::utils::{content_safe, ContentSafeOptions};
pub use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
pub async fn announce<S>(
    ctx: &Context,
    msg: &Message,
    announcement: S,
    response_type: &CommandResponse,
) -> CommandResult
where
    S: std::fmt::Display + AsRef<str>,
{
    use CommandResponse::*;
    match response_type {
        Dm => direct_message_admin(ctx, msg, announcement).await,
        Reply => reply_to_sender(ctx, msg, announcement).await,
        Channel => announce_to_channel(ctx, msg, announcement).await,
    }
}

/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
async fn announce_to_channel<S>(ctx: &Context, msg: &Message, announcement: S) -> CommandResult
where
    S: AsRef<str>,
{
    let content = content_safe(&ctx.cache, announcement, &ContentSafeOptions::default()).await;

    match msg.channel_id.say(&ctx.http, &content).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Announce error: {:#?}", e);
            Err(Box::new(e))
        }
    }
}

/// bot sends dm to admin to avoid leaking info to channel
async fn direct_message_admin<S>(ctx: &Context, msg: &Message, dm: S) -> CommandResult
where
    S: std::fmt::Display,
{
    match msg.author.direct_message(ctx, |m| m.content(dm)).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("DM to Admin failed: {}", e.to_string());
            Err(Box::new(e))
        }
    }
}

/// bot replies to message sender in channel
async fn reply_to_sender<S>(ctx: &Context, msg: &Message, reply: S) -> CommandResult
where
    S: std::fmt::Display,
{
    match msg.reply(ctx, reply).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error replying to {}: {}", &msg.author, e.to_string());
            Err(Box::new(e))
        }
    }
}
