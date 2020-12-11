use crate::{prelude::*, Links};

#[command]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Shop);
    if cutter.fetch().await.is_ok() {
        if let Some(name_nodes) = cutter.get_nodes_vec("item_name") {
            if let Some(url_nodes) = cutter.get_nodes_vec("shop_url") {
                let site_url = &crate::CONFIG.lock().await.site_url;

                let zipped = name_nodes.iter().zip(url_nodes).fold(
                    String::new(),
                    |mut message, (name, path)| {
                        message.push_str(&name);
                        message.push_str("\n");
                        let mut url = site_url.clone();
                        url.push_str(&path);
                        message.push_str(&url);
                        message.push_str("\n\n");
                        message
                    },
                );
                announce(ctx, msg, zipped, &CommandResponse::Channel).await?;
            }
        }
    }
    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::About);
    if cutter.fetch().await.is_ok() {
        let mut message = String::new();
        if let Some(about1) = cutter.get_node_val("about1") {
            message.push_str(&about1);
        }
        if let Some(about2) = cutter.get_node_val("about2") {
            message.push_str(&about2);
        }
        announce(ctx, msg, message, &CommandResponse::Channel).await?;
    }
    Ok(())
}

#[command]
async fn goal(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Goals);
    if cutter.fetch().await.is_ok() {
        if let Some(goal_nodes) = cutter.get_nodes_vec("goal") {
            let mut rand_int = rand::random::<usize>();
            let num_goals = goal_nodes.len();
            if rand_int > num_goals {
                rand_int = rand_int % num_goals;
            }
            let rand_goal = &goal_nodes[rand_int];
            announce(ctx, msg, rand_goal, &CommandResponse::Reply).await?;
        }
    }
    Ok(())
}

#[command]
async fn goals(ctx: &Context, msg: &Message) -> CommandResult {
    announce(
        ctx,
        msg,
        Links::Goals.display().await,
        &CommandResponse::Channel,
    )
    .await
}

/// dm's server owner with request_invite_msg and user's name who made request, then
/// dm's requester with confirmation/error message.
#[command]
async fn minecraft(ctx: &Context, msg: &Message) -> CommandResult {
    let request_invite_msg = format!(
        "{} has requested an invite to the minecraft server!",
        &msg.author.name
    );

    let success_msg = if let Err(e) =
        announce(ctx, msg, request_invite_msg, &CommandResponse::Dm).await
    {
        error!(
            "MC request by {}: DM to owner failed : {}",
            &msg.author,
            e.to_string()
        );
        "Oops, something went wrong! Please try again or dm MM your request directly if the issue persists."
    } else {
        "Your request has been sent!"
    };

    if let Err(e) = msg.author.dm(ctx, |m| m.content(success_msg)).await {
        error!(
            "MC request by {}: DM to requester failed : {}",
            &msg.author,
            e.to_string()
        );
    }
    Ok(())
}
