use crate::prelude::*;

/// dm's server owner with request_invite_msg and user's name who made request, then
/// dm's requester with confirmation/error message.
#[command]
async fn minecraft(ctx: &Context, msg: &Message) -> CommandResult {
    let request_invite_msg = format!(
        "{} has requested an invite to the minecraft server!",
        &msg.author.name
    );
    let owner = &crate::OWNER.lock().await;
    let dm = owner.direct_message(ctx, |m| m.content(request_invite_msg));

    let success_msg = if dm.await.is_ok() {
        "Your request has been sent!"
    } else {
        "Oops, something went wrong! Please try again or dm MM your request directly if the issue persists."
    };

    msg.author
        .direct_message(ctx, |m| m.content(success_msg))
        .await?;
    Ok(())
}
