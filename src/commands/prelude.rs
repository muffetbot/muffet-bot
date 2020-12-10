pub use crate::utils::scraper::SteelCutter;
pub use log::*;
pub use serenity::prelude::*;
use serenity::utils::{content_safe, ContentSafeOptions};
pub use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
pub async fn announce<S>(ctx: &Context, msg: &Message, announcement: S) -> CommandResult
where
    S: AsRef<str>,
{
    let content = content_safe(&ctx.cache, announcement, &ContentSafeOptions::default()).await;

    if let Err(e) = msg.channel_id.say(&ctx.http, &content).await {
        error!("Error sending message: {:#?}", e);
    }

    Ok(())
}
