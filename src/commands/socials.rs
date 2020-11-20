use crate::{prelude::*, Links::*};

/*
*   template function:
*
*   #[command]
*   async fn func_name(ctx: &Context, msg: &Message) -> CommandResult {
*       open_in_browser(NewLinksField, ctx, msg).await // see README for proper creation of NewLinksField
*   }
*/

#[command]
async fn email(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Email, ctx, msg).await
}

#[command]
async fn patreon(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Patreon, ctx, msg).await
}

#[command]
async fn twitter(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Twitter, ctx, msg).await
}

#[command]
async fn venmo(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Venmo, ctx, msg).await
}

#[command]
async fn youtube(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(YouTube, ctx, msg).await
}
