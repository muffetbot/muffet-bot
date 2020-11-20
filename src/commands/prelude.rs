pub use crate::utils::scraper::SteelCutter;
pub use serenity::prelude::*;
pub use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

/// site argument accepts only Links enum field bc more secure than any given string slice
pub async fn open_in_browser(site: crate::Links, ctx: &Context, msg: &Message) -> CommandResult {
    match webbrowser::open(site.as_ref()) {
        Ok(_) => Ok(()),
        Err(e) => announce(ctx, msg, &e.to_string()).await,
    }
}

use serenity::utils::{content_safe, ContentSafeOptions};
/// announces given message to entire thread. `announcement` can be any type that implements as_ref for string slice
pub async fn announce<S>(ctx: &Context, msg: &Message, announcement: S) -> CommandResult
where
    S: AsRef<str>,
{
    let content = content_safe(&ctx.cache, announcement, &ContentSafeOptions::default()).await;

    if let Err(e) = msg.channel_id.say(&ctx.http, &content).await {
        eprintln!("Error sending message: {:#?}", e);
    }

    Ok(())
}
