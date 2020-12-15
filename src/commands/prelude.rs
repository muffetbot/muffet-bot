pub use crate::utils::{config::CommandResponse, scraper::SteelCutter};
use serenity::utils::{content_safe, ContentSafeOptions};
pub use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};
pub use tracing::{info, instrument};

pub async fn has_permissions(msg: &Message) -> bool {
    msg.author == *crate::OWNER.lock().await
}

/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
pub async fn announce<S>(
    ctx: &Context,
    msg: &Message,
    announcement: S,
    response_type: &CommandResponse,
) -> CommandResult
where
    S: std::fmt::Display + std::fmt::Debug + AsRef<str>,
{
    use CommandResponse::*;
    match response_type {
        Dm => direct_message(ctx, msg, announcement, false).await,
        DmOwner => direct_message(ctx, msg, announcement, true).await,
        Reply => reply_to_sender(ctx, msg, announcement).await,
        Channel => announce_to_channel(ctx, msg, announcement).await,
        _ => Ok(()),
    }
}

/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
#[instrument]
async fn announce_to_channel<S>(ctx: &Context, msg: &Message, announcement: S) -> CommandResult
where
    S: AsRef<str> + std::fmt::Debug,
{
    let content = content_safe(&ctx.cache, announcement, &ContentSafeOptions::default()).await;

    if let Err(e) = msg.channel_id.say(&ctx.http, &content).await {
        info!("Announce error: {:#?}", e);
    }
    Ok(())
}

/// bot sends dm to admin to avoid leaking info to channel
#[instrument]
async fn direct_message<S>(ctx: &Context, msg: &Message, dm: S, to_owner: bool) -> CommandResult
where
    S: std::fmt::Display + std::fmt::Debug,
{
    if to_owner && !has_permissions(msg).await {
        return Ok(());
    }

    let color = crate::CONFIG.lock().await.get_help_color().clone();
    if let Err(e) = msg
        .author
        .direct_message(ctx, |m| {
            m.embed(|embed| {
                embed.color(color);
                embed.field("hi!", dm, true);
                embed
            });
            m
        })
        .await
    {
        info!("DM to Admin failed: {}", e.to_string());
    }
    Ok(())
}

/// bot replies to message sender in channel
#[instrument]
async fn reply_to_sender<S>(ctx: &Context, msg: &Message, reply: S) -> CommandResult
where
    S: std::fmt::Display + std::fmt::Debug,
{
    if let Err(e) = msg.reply(ctx, reply).await {
        info!("Error replying to {}: {}", &msg.author, e.to_string());
    }
    Ok(())
}
