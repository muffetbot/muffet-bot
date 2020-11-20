use crate::{prelude::*, Links};

/*
*   scraping functions (SteelCutter) require more setup/know-how
*   if you really wanna make one, dm me
*/

#[command]
async fn contact(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Contact);

    if cutter.fetch().await.is_ok() {
        if let Some(test_title) = cutter.get_node_val("title") {
            announce(ctx, msg, &test_title).await?;
        }
        if let Some(test_title) = cutter.get_node_val("meat") {
            announce(ctx, msg, &test_title).await?;
        }
    }
    Ok(())
}

#[command]
async fn donate(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Links::Donate, ctx, msg).await
}

#[command]
async fn goals(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Goals);
    if cutter.fetch().await.is_ok() {
        if let Some(tree) = cutter.get_nodes_vec("entry") {
            let goals_str = tree.join("\n");
            announce(ctx, msg, goals_str).await?;
        }
    }
    Ok(())
}

#[command]
async fn poetry(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Links::Poetry, ctx, msg).await
}

#[command]
async fn stream(ctx: &Context, msg: &Message) -> CommandResult {
    open_in_browser(Links::Stream, ctx, msg).await
}
