use crate::{prelude::*, Links::*};

/*
*   template function:
*
*   #[command]
*   async fn func_name(ctx: &Context, msg: &Message) -> CommandResult {
*       announce(ctx, msg, NewLinksField).await // see README for proper creation of NewLinksField
*   }
*/

#[command]
async fn email(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Email).await
}

#[command]
async fn patreon(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Patreon).await
}

#[command]
async fn twitter(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Twitter).await
}

#[command]
async fn venmo(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, Venmo).await
}

#[command]
async fn youtube(ctx: &Context, msg: &Message) -> CommandResult {
    announce(ctx, msg, YouTube).await
}
