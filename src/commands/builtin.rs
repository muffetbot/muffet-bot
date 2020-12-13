use crate::{prelude::*, Links};

#[command]
#[description = "support me and buy my poetry!"]
#[usage = "`!shop`"]
async fn shop(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Shop);
    if cutter.fetch().await.is_ok() {
        if let Some(name_nodes) = cutter.get_nodes_vec("item_name") {
            if let Some(url_nodes) = cutter.get_nodes_vec("shop_url") {
                let site_url = &crate::CONFIG.lock().await.site_url;

                let zipped = name_nodes.iter().zip(url_nodes).fold(
                    String::new(),
                    |mut message, (name, path)| {
                        message += &format!("{}\n{}{}\n\n", name, &site_url, path);
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
#[description = "this is the about of about - how meta"]
#[usage = "`!about`"]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::About);
    if cutter.fetch().await.is_ok() {
        let mut message = String::new();
        if let Some(about_nodes) = cutter.get_nodes_vec("about") {
            message += &about_nodes[0..2].join("\n\n");
        }
        announce(ctx, msg, message, &CommandResponse::Channel).await?;
    }
    Ok(())
}

#[command]
#[description = "get a goal at random from steelcutkawaii.com!"]
#[usage = "`!goal`"]
async fn goal(ctx: &Context, msg: &Message) -> CommandResult {
    let mut cutter = SteelCutter::new(Links::Goals);
    if cutter.fetch().await.is_ok() {
        if let Some(goal_nodes) = cutter.get_nodes_vec("goal") {
            let rand_int = rand::random::<usize>() % goal_nodes.len();
            let rand_goal = &goal_nodes[rand_int];
            announce(ctx, msg, rand_goal, &CommandResponse::Reply).await?;
        }
    }
    Ok(())
}

#[command]
#[description = "check out this month's goals!"]
#[usage = "`!goals`"]
async fn goals(ctx: &Context, msg: &Message) -> CommandResult {
    announce(
        ctx,
        msg,
        Links::Goals.display().await,
        &CommandResponse::Channel,
    )
    .await
}

#[command]
#[description = "request an invite to the minecraft server!"]
#[usage = "`!minecraft`"]
async fn minecraft(ctx: &Context, msg: &Message) -> CommandResult {
    let request_invite_msg = format!(
        "{} has requested an invite to the minecraft server!",
        &msg.author.name
    );

    let success_msg = if let Err(e) =
        announce(ctx, msg, request_invite_msg, &CommandResponse::DmOwner).await
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

    if let Err(e) = announce(ctx, msg, success_msg, &CommandResponse::Dm).await {
        error!(
            "MC request by {}: DM to requester failed : {}",
            &msg.author,
            e.to_string()
        );
    }

    Ok(())
}
